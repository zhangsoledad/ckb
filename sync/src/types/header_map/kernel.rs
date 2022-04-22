use std::path;

use ckb_async_runtime::Handle;
// #[cfg(feature = "stats")]
// use ckb_logger::trace;
use ckb_types::packed::Byte32;

use super::MemoryMap;
use super::RocksDB;
use crate::types::HeaderView;
use ckb_util::{shrink_to_fit, LinkedHashMap, RwLock};

pub(crate) struct HeaderMapKernel {
    memory: MemoryMap,
    store: RocksDB,
    // Configuration
    primary_limit: usize,
    // backend_close_threshold: usize,

    // Statistics
    #[cfg(feature = "stats")]
    stats: HeaderMapKernelStats,
}

// #[cfg(feature = "stats")]
// #[derive(Default)]
// struct HeaderMapKernelStats {
//     frequency: usize,

//     trace_progress: usize,

//     primary_contain: usize,
//     primary_select: usize,
//     primary_insert: usize,
//     primary_delete: usize,

//     backend_contain: usize,
//     backend_insert: usize,
//     backend_delete: usize,
// }

impl HeaderMapKernel {
    pub(crate) fn new<P>(tmpdir: Option<P>, primary_limit: usize) -> Self
    where
        P: AsRef<path::Path>,
    {
        let memory = Default::default();
        let store = RocksDB::new(tmpdir);
        Self {
            memory,
            store,
            primary_limit,
            // backend_close_threshold,
        }

        // #[cfg(feature = "stats")]
        // {
        //     Self {
        //         primary,
        //         backend,
        //         primary_limit,
        //         backend_close_threshold,
        //         stats: HeaderMapLruKernelStats::new(50_000),
        //     }
        // }
    }

    pub(crate) fn contains_key(&self, hash: &Byte32) -> bool {
        if self.memory.contains_key(hash) {
            return true;
        }
        self.store.contains_key(hash)
    }

    pub(crate) fn get(&self, hash: &Byte32) -> Option<HeaderView> {
        if let Some(view) = self.memory.get_refresh(hash) {
            return Some(view);
        }
        self.store.get(hash)
    }

    pub(crate) fn insert(&self, view: HeaderView) {
        self.memory.insert(view.hash(), view)
    }

    pub(crate) fn remove(&self, hash: &Byte32) {
        self.memory.remove(hash);
        self.store.remove(hash);
    }

    pub(crate) fn limit_memory(&self) {
        if let Some(pairs) = self.memory.front(self.primary_limit) {
            tokio::task::block_in_place(|| {
                self.store.insert_batch(&pairs);
                self.memory.delete_batch(pairs.iter().map(|pair| &pair.0))
            })
        }
    }

    // #[cfg(feature = "stats")]
    // fn trace(&mut self) {
    //     let progress = self.stats().trace_progress();
    //     let frequency = self.stats().frequency();
    //     if progress % frequency == 0 {
    //         trace!(
    //             "Header Map Statistics\
    //         \n>\t| storage | length  |  limit  | contain |   select   | insert  | delete  |\
    //         \n>\t|---------+---------+---------+---------+------------+---------+---------|\
    //         \n>\t| primary |{:>9}|{:>9}|{:>9}|{:>12}|{:>9}|{:>9}|\
    //         \n>\t| backend |{:>9}|{:>9}|{:>9}|{:>12}|{:>9}|{:>9}|\
    //         ",
    //             self.primary.len(),
    //             self.primary_limit,
    //             self.stats().primary_contain,
    //             self.stats().primary_select,
    //             self.stats().primary_insert,
    //             self.stats().primary_delete,
    //             self.backend.len(),
    //             self.backend.is_opened(),
    //             self.stats().backend_contain,
    //             '-',
    //             self.stats().backend_insert,
    //             self.stats().backend_delete,
    //         );
    //         self.mut_stats().trace_progress_reset();
    //     } else {
    //         self.mut_stats().trace_progress_tick();
    //     }
    // }

    // #[cfg(feature = "stats")]
    // fn stats(&self) -> &HeaderMapLruKernelStats {
    //     &self.stats
    // }

    // #[cfg(feature = "stats")]
    // fn mut_stats(&mut self) -> &mut HeaderMapLruKernelStats {
    //     &mut self.stats
    // }
}

// #[cfg(feature = "stats")]
// impl HeaderMapLruKernelStats {
//     fn new(frequency: usize) -> Self {
//         Self {
//             frequency,
//             ..Default::default()
//         }
//     }

//     fn frequency(&self) -> usize {
//         self.frequency
//     }

//     fn trace_progress(&self) -> usize {
//         self.trace_progress
//     }

//     fn trace_progress_reset(&mut self) {
//         self.trace_progress = 1;
//     }

//     fn trace_progress_tick(&mut self) {
//         self.trace_progress += 1;
//     }

//     fn tick_primary_contain(&mut self) {
//         self.primary_contain += 1;
//     }

//     fn tick_backend_contain(&mut self) {
//         self.backend_contain += 1;
//     }

//     fn tick_primary_select(&mut self) {
//         self.primary_select += 1;
//     }

//     fn tick_primary_insert(&mut self) {
//         self.primary_insert += 1;
//     }

//     fn tick_backend_insert(&mut self) {
//         self.backend_insert += 1;
//     }

//     fn tick_primary_delete(&mut self) {
//         self.primary_delete += 1;
//     }

//     fn tick_backend_delete(&mut self) {
//         self.backend_delete += 1;
//     }
// }
