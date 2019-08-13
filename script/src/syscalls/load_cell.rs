use crate::syscalls::{
    utils::store_data, CellField, Source, SourceEntry, INDEX_OUT_OF_BOUND, ITEM_MISSING,
    LOAD_CELL_BY_FIELD_SYSCALL_NUMBER, LOAD_CELL_SYSCALL_NUMBER, SUCCESS,
};
use byteorder::{LittleEndian, WriteBytesExt};
use ckb_types::{
    core::{
        cell::{CellMeta, ResolvedOutPoint},
        Capacity,
    },
    packed::CellOutput,
    prelude::*,
    H256,
};
use ckb_vm::{
    registers::{A0, A3, A4, A5, A7},
    Error as VMError, Register, SupportMachine, Syscalls,
};

pub struct LoadCell<'a> {
    outputs: &'a [CellMeta],
    resolved_inputs: &'a [ResolvedOutPoint],
    resolved_deps: &'a [ResolvedOutPoint],
    group_inputs: &'a [usize],
    group_outputs: &'a [usize],
}

impl<'a> LoadCell<'a> {
    pub fn new(
        outputs: &'a [CellMeta],
        resolved_inputs: &'a [ResolvedOutPoint],
        resolved_deps: &'a [ResolvedOutPoint],
        group_inputs: &'a [usize],
        group_outputs: &'a [usize],
    ) -> LoadCell<'a> {
        LoadCell {
            outputs,
            resolved_inputs,
            resolved_deps,
            group_inputs,
            group_outputs,
        }
    }

    fn fetch_cell(&self, source: Source, index: usize) -> Result<&'a CellMeta, u8> {
        match source {
            Source::Transaction(SourceEntry::Input) => self
                .resolved_inputs
                .get(index)
                .ok_or(INDEX_OUT_OF_BOUND)
                .and_then(|r| r.cell().ok_or(ITEM_MISSING)),
            Source::Transaction(SourceEntry::Output) => {
                self.outputs.get(index).ok_or(INDEX_OUT_OF_BOUND)
            }
            Source::Transaction(SourceEntry::Dep) => self
                .resolved_deps
                .get(index)
                .ok_or(INDEX_OUT_OF_BOUND)
                .and_then(|r| r.cell().ok_or(ITEM_MISSING)),
            Source::Group(SourceEntry::Input) => self
                .group_inputs
                .get(index)
                .ok_or(INDEX_OUT_OF_BOUND)
                .and_then(|actual_index| {
                    self.resolved_inputs
                        .get(*actual_index)
                        .ok_or(INDEX_OUT_OF_BOUND)
                })
                .and_then(|r| r.cell().ok_or(ITEM_MISSING)),
            Source::Group(SourceEntry::Output) => self
                .group_outputs
                .get(index)
                .ok_or(INDEX_OUT_OF_BOUND)
                .and_then(|actual_index| self.outputs.get(*actual_index).ok_or(INDEX_OUT_OF_BOUND)),
            Source::Group(SourceEntry::Dep) => Err(INDEX_OUT_OF_BOUND),
        }
    }

    fn load_full<Mac: SupportMachine>(
        &self,
        machine: &mut Mac,
        output: &CellOutput,
    ) -> Result<(u8, usize), VMError> {
        // NOTE: this is a very expensive operation here since we need to copy
        // everything in a cell to a flatbuffer object, serialize the object
        // into a buffer, and then copy requested data to VM memory space. So
        // we should charge cycles proportional to the full Cell size no matter
        // how much data the actual script is requesting, the per-byte cycle charged
        // here, should also be significantly higher than LOAD_CELL_BY_FIELD.
        // Also, while this is debatable, I suggest we charge full cycles for
        // subsequent calls even if we have cache implemented here.
        // TODO: find a way to cache this without consuming too much memory
        store_data(machine, output.as_slice())?;
        Ok((SUCCESS, output.as_slice().len()))
    }

    fn load_by_field<Mac: SupportMachine>(
        &self,
        machine: &mut Mac,
        cell: &CellMeta,
    ) -> Result<(u8, usize), VMError> {
        let field = CellField::parse_from_u64(machine.registers()[A5].to_u64())?;
        let output = &cell.cell_output;

        let result = match field {
            CellField::Capacity => {
                let capacity: Capacity = output.capacity().unpack();
                let mut buffer = vec![];
                buffer.write_u64::<LittleEndian>(capacity.as_u64())?;
                store_data(machine, &buffer)?;
                (SUCCESS, buffer.len())
            }
            CellField::OccupiedCapacity => {
                let mut buffer = vec![];
                buffer.write_u64::<LittleEndian>(
                    cell.occupied_capacity()
                        .map_err(|_| VMError::Unexpected)?
                        .as_u64(),
                )?;
                store_data(machine, &buffer)?;
                (SUCCESS, buffer.len())
            }
            CellField::DataHash => {
                let hash: H256 = output.data_hash().unpack();
                let bytes = hash.as_bytes();
                store_data(machine, bytes)?;
                (SUCCESS, bytes.len())
            }
            CellField::Lock => {
                let lock = output.lock();
                store_data(machine, lock.as_slice())?;
                (SUCCESS, lock.as_slice().len())
            }
            CellField::LockHash => {
                let hash = output.lock().calc_hash();
                let bytes = hash.as_bytes();
                store_data(machine, &bytes)?;
                (SUCCESS, bytes.len())
            }
            CellField::Type => match output.type_().to_opt() {
                Some(type_) => {
                    store_data(machine, type_.as_slice())?;
                    (SUCCESS, type_.as_slice().len())
                }
                None => (ITEM_MISSING, 0),
            },
            CellField::TypeHash => match output.type_().to_opt() {
                Some(type_) => {
                    let hash = type_.calc_hash();
                    let bytes = hash.as_bytes();
                    store_data(machine, &bytes)?;
                    (SUCCESS, bytes.len())
                }
                None => (ITEM_MISSING, 0),
            },
        };
        Ok(result)
    }
}

impl<'a, Mac: SupportMachine> Syscalls<Mac> for LoadCell<'a> {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), VMError> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, VMError> {
        let (load_by_field, cycle_factor) = match machine.registers()[A7].to_u64() {
            LOAD_CELL_SYSCALL_NUMBER => (false, 100),
            LOAD_CELL_BY_FIELD_SYSCALL_NUMBER => (true, 10),
            _ => return Ok(false),
        };

        let index = machine.registers()[A3].to_u64();
        let source = Source::parse_from_u64(machine.registers()[A4].to_u64())?;

        let cell = self.fetch_cell(source, index as usize);
        if cell.is_err() {
            machine.set_register(A0, Mac::REG::from_u8(cell.unwrap_err()));
            return Ok(true);
        }
        let cell = cell.unwrap();
        let (return_code, len) = if load_by_field {
            self.load_by_field(machine, cell)?
        } else {
            self.load_full(machine, &cell.cell_output)?
        };

        machine.add_cycles(len as u64 * cycle_factor)?;
        machine.set_register(A0, Mac::REG::from_u8(return_code));
        Ok(true)
    }
}
