use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::Instant,
};

use chashmap::CHashMap;
use clap::Parser;
use dashmap::DashMap;
use fnv::FnvBuildHasher;

#[derive(Parser)]
struct Opts {
    pub n_rounds: usize,
    pub n_workers: usize,
}

fn main() {
    let opts = Opts::parse();

    let since = Instant::now();
    run_std_hashmap(&opts);
    println!("std hashmap {:?}", since.elapsed());

    let since = Instant::now();
    run_dashmap(&opts);
    println!("dashmap {:?}", since.elapsed());

    let since = Instant::now();
    run_chashmap(&opts);
    println!("chashmap {:?}", since.elapsed());

    let since = Instant::now();
    run_flurry(&opts);
    println!("flurry {:?}", since.elapsed());
}

fn run_std_hashmap(opts: &Opts) {
    let Opts {
        n_rounds,
        n_workers,
    } = *opts;

    let map = Arc::new(RwLock::new(HashMap::with_hasher(FnvBuildHasher::default())));

    let spawns: Vec<_> = (0..n_workers)
        .map(|_| {
            let map = map.clone();

            thread::spawn(move || {
                (0..n_rounds).for_each(|i| {
                    map.write().unwrap().insert(i, i);
                });
            })
        })
        .collect();

    for handle in spawns {
        handle.join().unwrap();
    }
}

fn run_dashmap(opts: &Opts) {
    let Opts {
        n_rounds,
        n_workers,
    } = *opts;

    let map = Arc::new(DashMap::with_hasher(FnvBuildHasher::default()));

    let spawns: Vec<_> = (0..n_workers)
        .map(|_| {
            let map = map.clone();

            thread::spawn(move || {
                (0..n_rounds).for_each(|i| {
                    map.insert(i, i);
                });
            })
        })
        .collect();

    for handle in spawns {
        handle.join().unwrap();
    }
}

fn run_chashmap(opts: &Opts) {
    let Opts {
        n_rounds,
        n_workers,
    } = *opts;

    let map = Arc::new(CHashMap::new());

    let spawns: Vec<_> = (0..n_workers)
        .map(|_| {
            let map = map.clone();

            thread::spawn(move || {
                (0..n_rounds).for_each(|i| {
                    map.insert(i, i);
                });
            })
        })
        .collect();

    for handle in spawns {
        handle.join().unwrap();
    }
}

fn run_flurry(opts: &Opts) {
    let Opts {
        n_rounds,
        n_workers,
    } = *opts;

    let map = Arc::new(flurry::HashMap::with_hasher(FnvBuildHasher::default()));

    let spawns: Vec<_> = (0..n_workers)
        .map(|_| {
            let map = map.clone();

            thread::spawn(move || {
                (0..n_rounds).for_each(|i| {
                    map.pin().insert(i, i);
                });
            })
        })
        .collect();

    for handle in spawns {
        handle.join().unwrap();
    }
}
