use std::{collections::HashMap, thread, time::Instant};

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

    let spawns: Vec<_> = (0..n_workers)
        .map(|_| {
            thread::spawn(move || {
                (0..n_rounds).for_each(|i| {
                    let mut map = HashMap::with_hasher(FnvBuildHasher::default());
                    map.insert(i, i);
                });
            })
        })
        .collect();

    for handle in spawns {
        handle.join().unwrap();
    }
}

fn run_dashmap(opts: &Opts) {
    #[cfg(feature = "dashmap_cache_shared_amount")]
    use once_cell::sync::Lazy;
    #[cfg(feature = "dashmap_cache_shared_amount")]
    use std::thread::available_parallelism;

    #[cfg(feature = "dashmap_cache_shared_amount")]
    static DEFAULT_SHARD_AMOUNT: Lazy<usize> =
        Lazy::new(|| (available_parallelism().map_or(1, usize::from) * 4).next_power_of_two());

    let Opts {
        n_rounds,
        n_workers,
    } = *opts;

    let spawns: Vec<_> = (0..n_workers)
        .map(|_| {
            thread::spawn(move || {
                (0..n_rounds).for_each(|i| {
                    #[cfg(feature = "dashmap_cache_shared_amount")]
                    let map = DashMap::with_capacity_and_hasher_and_shard_amount(
                        0,
                        FnvBuildHasher::default(),
                        *DEFAULT_SHARD_AMOUNT,
                    );

                    #[cfg(not(feature = "dashmap_cache_shared_amount"))]
                    let map = DashMap::with_hasher(FnvBuildHasher::default());

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

    let spawns: Vec<_> = (0..n_workers)
        .map(|_| {
            thread::spawn(move || {
                (0..n_rounds).for_each(|i| {
                    let map = CHashMap::new();
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

    let spawns: Vec<_> = (0..n_workers)
        .map(|_| {
            thread::spawn(move || {
                (0..n_rounds).for_each(|i| {
                    let map = flurry::HashMap::with_hasher(FnvBuildHasher::default());
                    map.pin().insert(i, i);
                });
            })
        })
        .collect();

    for handle in spawns {
        handle.join().unwrap();
    }
}
