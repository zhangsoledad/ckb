use ckb_types::{core::PlainHashMap, packed};
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use criterion::*;

fn bench(c: &mut Criterion) {
    let mut keys = Vec::with_capacity(20000);
    for _ in 0..20000 {
        let mut arr = [0u8; 32];
        thread_rng().fill(&mut arr[..]);
        keys.push(packed::Byte32::new(arr));
    }
    c.bench_function(
        "std hashmap insert",
        |b| b.iter(|| {
            let mut map = HashMap::new();
            for key in keys.clone() {
                map.insert(key, ());
            }
        }),
    );

    c.bench_function(
        "plain hashmap insert",
        |b| b.iter(|| {
            let mut map = PlainHashMap::default();
            for key in keys.clone() {
                map.insert(key, ());
            }
        }),
    );


    let mut stdmap = HashMap::new();
    let mut plainmap = PlainHashMap::default();
    for key in keys.clone() {
        stdmap.insert(key.clone(), ());
        plainmap.insert(key, ());
    }

    c.bench_function(
        "std hashmap query",
        |b| b.iter(|| {
            for key in &keys {
                stdmap.get(key);
            }
        }),
    );


    c.bench_function(
        "plain hashmap query",
        |b| b.iter(|| {
            for key in &keys {
                plainmap.get(key);
            }
        }),
    );

    c.bench_function(
        "std hashmap remove",
        |b| b.iter(|| {
            for key in &keys {
                stdmap.remove(key);
            }
        }),
    );


    c.bench_function(
        "plain hashmap remove",
        |b| b.iter(|| {
            for key in &keys {
                plainmap.remove(key);
            }
        }),
    );

}


criterion_group!(plain_hash, bench);
