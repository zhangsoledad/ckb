use std::path;

use ckb_types::packed::Byte32;

use crate::types::HeaderView;
use ckb_async_runtime::Handle;
use ckb_stop_handler::{SignalSender, StopHandler};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;

// mod backend;
mod kernel;
mod memory;
mod rocksdb;

pub(crate) use self::{kernel::HeaderMapKernel, memory::MemoryMap, rocksdb::RocksDB};

pub struct HeaderMap {
    inner: Arc<HeaderMapKernel>,
    async_handle: Handle,
    stop: StopHandler<()>,
}

impl HeaderMap {
    pub(crate) fn new<P>(tmpdir: Option<P>, primary_limit: usize, async_handle: Handle) -> Self
    where
        P: AsRef<path::Path>,
    {
        let inner = Arc::new(HeaderMapKernel::new(tmpdir, primary_limit));
        let map = Arc::clone(&inner);
        let interval = Duration::from_secs(10);
        let (stop, mut stop_rx) = oneshot::channel::<()>();

        async_handle.spawn(async move {
            let mut interval = tokio::time::interval(interval);
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        map.limit_memory();
                    }
                    _ = &mut stop_rx => break,
                }
            }
        });

        Self {
            inner,
            async_handle,
            stop: StopHandler::new(SignalSender::Tokio(stop), None, "HeaderMap".to_string()),
        }
    }

    pub(crate) fn contains_key(&self, hash: &Byte32) -> bool {
        self.inner.contains_key(hash)
    }

    pub(crate) fn get(&self, hash: &Byte32) -> Option<HeaderView> {
        self.inner.get(hash)
    }

    pub(crate) fn insert(&self, view: HeaderView) {
        self.inner.insert(view)
    }

    pub(crate) fn remove(&self, hash: &Byte32) {
        self.inner.remove(hash)
    }
}
