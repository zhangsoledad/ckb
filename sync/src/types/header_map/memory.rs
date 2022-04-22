use std::{clone, cmp, default, hash};

use crate::types::HeaderView;
use ckb_types::packed::Byte32;
use ckb_util::shrink_to_fit;
use ckb_util::LinkedHashMap;
use ckb_util::{RwLock, RwLockUpgradableReadGuard};

use crate::types::SHRINK_THRESHOLD;

pub(crate) struct MemoryMap(RwLock<LinkedHashMap<Byte32, HeaderView>>);

impl default::Default for MemoryMap {
    fn default() -> Self {
        Self(RwLock::new(default::Default::default()))
    }
}

impl MemoryMap {
    pub(crate) fn len(&self) -> usize {
        self.0.read().len()
    }

    pub(crate) fn contains_key(&self, key: &Byte32) -> bool {
        self.0.read().contains_key(key)
    }

    pub(crate) fn get_refresh(&self, key: &Byte32) -> Option<HeaderView> {
        self.0.write().get_refresh(key).cloned()
    }

    pub(crate) fn insert(&self, key: Byte32, value: HeaderView) {
        self.0.write().insert(key, value);
    }

    pub(crate) fn remove(&self, key: &Byte32) {
        let mut guard = self.0.write();
        guard.remove(key);
        shrink_to_fit!(guard, SHRINK_THRESHOLD);
    }

    pub(crate) fn front(&self, size_limit: usize) -> Option<Vec<(Byte32, HeaderView)>> {
        let guard = self.0.read();
        let size = guard.len();
        if size > size_limit {
            let num = size - size_limit;
            Some(
                guard
                    .iter()
                    .take(num)
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            )
        } else {
            None
        }
    }

    pub(crate) fn delete_batch<'a>(&self, keys: impl Iterator<Item = &'a Byte32>) {
        let mut guard = self.0.write();
        for key in keys {
            guard.remove(key);
        }
        shrink_to_fit!(guard, SHRINK_THRESHOLD);
    }
}
