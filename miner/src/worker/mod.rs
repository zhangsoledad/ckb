mod cuckoo_simple;
mod dummy;

use crate::config::WorkerConfig;
use ckb_logger::error;
use ckb_pow::{CuckooEngine, DummyPowEngine, PowEngine};
use ckb_types::{packed::Seal, H256};
use crossbeam_channel::{unbounded, Sender};
use cuckoo_simple::CuckooSimple;
use dummy::Dummy;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{random, Rng};
use std::ops::Range;
use std::sync::Arc;
use std::thread;

pub use cuckoo_simple::CuckooSimpleConfig;
pub use dummy::DummyConfig;

#[derive(Clone)]
pub enum WorkerMessage {
    Stop,
    Start,
    NewWork(H256),
}

pub struct WorkerController {
    inner: Vec<Sender<WorkerMessage>>,
}

impl WorkerController {
    pub fn new(inner: Vec<Sender<WorkerMessage>>) -> Self {
        Self { inner }
    }

    pub fn send_message(&self, message: WorkerMessage) {
        for worker_tx in self.inner.iter() {
            if let Err(err) = worker_tx.send(message.clone()) {
                error!("worker_tx send error {:?}", err);
            };
        }
    }
}

fn partition_nonce(id: u64, total: u64) -> Range<u64> {
    let span = u64::max_value() / total;
    let start = span * id;
    let end = match id {
        x if x < total - 1 => start + span,
        x if x == total - 1 => u64::max_value(),
        _ => unreachable!(),
    };
    Range { start, end }
}

fn nonce_generator(range: Range<u64>) -> impl FnMut() -> u64 {
    let mut rng = rand::thread_rng();
    let Range { start, end } = range;
    move || rng.gen_range(start, end)
}

const PROGRESS_BAR_TEMPLATE: &str = "{prefix:.bold.dim} {spinner:.green} [{elapsed_precise}] {msg}";

pub fn start_worker(
    pow: Arc<dyn PowEngine>,
    config: &WorkerConfig,
    seal_tx: Sender<(H256, Seal)>,
    mp: &MultiProgress,
) -> WorkerController {
    match config {
        WorkerConfig::Dummy(config) => {
            if let Some(_dummy_engine) = pow.as_any().downcast_ref::<DummyPowEngine>() {
                let worker_name = "Dummy-Worker";
                let pb = mp.add(ProgressBar::new(100));
                pb.set_style(ProgressStyle::default_bar().template(PROGRESS_BAR_TEMPLATE));
                pb.set_prefix(&worker_name);

                let (worker_tx, worker_rx) = unbounded();
                let mut worker = Dummy::new(config, seal_tx, worker_rx);

                thread::Builder::new()
                    .name(worker_name.to_string())
                    .spawn(move || {
                        let rng = || random();
                        worker.run(rng, pb);
                    })
                    .expect("Start `Dummy` worker thread failed");
                WorkerController::new(vec![worker_tx])
            } else {
                panic!("incompatible pow engine and worker type");
            }
        }
        WorkerConfig::CuckooSimple(config) => {
            if let Some(cuckoo_engine) = pow.as_any().downcast_ref::<CuckooEngine>() {
                let worker_txs = (0..config.threads)
                    .map(|i| {
                        let worker_name = format!("CuckooSimple-Worker-{}", i);
                        let nonce_range = partition_nonce(i as u64, config.threads as u64);
                        // `100` is the len of progress bar, we can use any dummy value here,
                        // since we only show the spinner in console.
                        let pb = mp.add(ProgressBar::new(100));
                        pb.set_style(ProgressStyle::default_bar().template(PROGRESS_BAR_TEMPLATE));
                        pb.set_prefix(&worker_name);

                        let (worker_tx, worker_rx) = unbounded();
                        let (cuckoo, seal_tx) = (cuckoo_engine.cuckoo.clone(), seal_tx.clone());
                        thread::Builder::new()
                            .name(worker_name)
                            .spawn(move || {
                                let mut worker = CuckooSimple::new(cuckoo, seal_tx, worker_rx);
                                let rng = nonce_generator(nonce_range);
                                worker.run(rng, pb);
                            })
                            .expect("Start `CuckooSimple` worker thread failed");
                        worker_tx
                    })
                    .collect();

                WorkerController::new(worker_txs)
            } else {
                panic!("incompatible pow engine and worker type");
            }
        }
    }
}

pub trait Worker {
    fn run<G: FnMut() -> u64>(&mut self, rng: G, progress_bar: ProgressBar);
}
