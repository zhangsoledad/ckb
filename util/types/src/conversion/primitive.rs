use crate::{bytes::Bytes, packed, prelude::*};

impl Pack<packed::Bool> for bool {
    fn pack(&self) -> packed::Bool {
        let b = if *self { 0u8 } else { 1 };
        packed::Bool::new_unchecked(Bytes::from(vec![b]))
    }
}

impl<'r> Unpack<bool> for packed::BoolReader<'r> {
    fn unpack(&self) -> bool {
        match self.as_slice()[0] {
            0 => true,
            1 => false,
            _ => unreachable!(),
        }
    }
}
impl_conversion_for_entity_unpack!(bool, Bool);

impl Pack<packed::Uint32> for u32 {
    fn pack(&self) -> packed::Uint32 {
        packed::Uint32::new_unchecked(Bytes::from(&self.to_le_bytes()[..]))
    }
}

impl Pack<packed::Uint64> for u64 {
    fn pack(&self) -> packed::Uint64 {
        packed::Uint64::new_unchecked(Bytes::from(&self.to_le_bytes()[..]))
    }
}

impl Pack<packed::Uint32> for usize {
    fn pack(&self) -> packed::Uint32 {
        (*self as u32).pack()
    }
}

impl<'r> Unpack<u32> for packed::Uint32Reader<'r> {
    fn unpack(&self) -> u32 {
        let le = self.as_slice().as_ptr() as *const u32;
        u32::from_le(unsafe { *le })
    }
}
impl_conversion_for_entity_unpack!(u32, Uint32);

impl<'r> Unpack<u64> for packed::Uint64Reader<'r> {
    fn unpack(&self) -> u64 {
        let le = self.as_slice().as_ptr() as *const u64;
        u64::from_le(unsafe { *le })
    }
}
impl_conversion_for_entity_unpack!(u64, Uint64);

impl<'r> Unpack<usize> for packed::Uint32Reader<'r> {
    fn unpack(&self) -> usize {
        let x: u32 = self.unpack();
        x as usize
    }
}
impl_conversion_for_entity_unpack!(usize, Uint32);

impl Pack<packed::Bytes> for [u8] {
    fn pack(&self) -> packed::Bytes {
        let len = self.len();
        let mut vec: Vec<u8> = Vec::with_capacity(4 + len);
        vec.extend_from_slice(&(len as u32).to_le_bytes()[..]);
        vec.extend_from_slice(self);
        packed::Bytes::new_unchecked(Bytes::from(vec))
    }
}

impl<'r> Unpack<Vec<u8>> for packed::BytesReader<'r> {
    fn unpack(&self) -> Vec<u8> {
        self.raw_data().to_owned()
    }
}
impl_conversion_for_entity_unpack!(Vec<u8>, Bytes);

impl Pack<packed::Bytes> for str {
    fn pack(&self) -> packed::Bytes {
        self.as_bytes().pack()
    }
}

impl<'r> packed::BytesReader<'r> {
    pub fn as_unpack_slice(&self) -> &[u8] {
        self.raw_data()
    }

    pub fn as_utf8(&self) -> Result<&str, ::std::str::Utf8Error> {
        ::std::str::from_utf8(self.as_unpack_slice())
    }

    pub unsafe fn as_utf8_unchecked(&self) -> &str {
        ::std::str::from_utf8_unchecked(self.as_unpack_slice())
    }

    pub fn is_utf8(&self) -> bool {
        self.as_utf8().is_ok()
    }
}

impl packed::Bytes {
    pub fn as_unpack_bytes(&self) -> Bytes {
        self.as_bytes().slice_from(4)
    }
}

impl Pack<packed::Bytes> for String {
    fn pack(&self) -> packed::Bytes {
        self.as_str().pack()
    }
}

impl_conversion_for_option!(bool, BoolOpt, BoolOptReader);
impl_conversion_for_vector!(u32, Uint32Vec, Uint32VecReader);
impl_conversion_for_vector!(usize, Uint32Vec, Uint32VecReader);
impl_conversion_for_vector!(u64, Uint64Vec, Uint64VecReader);
impl_conversion_for_option_pack!(&str, BytesOpt);
impl_conversion_for_option_pack!(String, BytesOpt);
