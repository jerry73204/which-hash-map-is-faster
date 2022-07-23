# Which HashMap is Faster?

Which HashMap is the most speediest in the wild Rust?

## Usage

HashMap initialization benchmark with 12 threads, each running 1000 creations.

```sh
cargo run --release --bin init -- 1000 12
```

HashMap initialization benchmark but caching shared amout for DashMap.

```sh
cargo run --release --features dashmap_cache_shared_amount --bin init -- 1000 12
```

HashMap concurrent insertion to one instance benchmark with 12
threads, each inserting 1000 values.

```sh
cargo run --release --bin init -- 1000 12
```

## Results

This benchmark runs on 12-hyperthread Intel i7-10750H CPU. Compiler
version is rustc 1.62.0 (a8314ef7d 2022-06-27). FNV hasher
([docs.rs](https://docs.rs/fnv/)) is used except chashmap.


Concurrent insertion test with 12 threads, each 1000 insertions.

```
std hashmap 2.202523ms
dashmap 712.999µs
chashmap 1.564289ms
flurry 14.784688ms
```

1000 HashMap initializations per 12 threads without caching shared amount.

```
std hashmap 631.394µs
dashmap 57.643348ms
chashmap 658.374µs
flurry 4.540421ms
```

1000 HashMap initializations per 12 threads with caching shared amount.

```
std hashmap 779.049µs
dashmap 1.491426ms
chashmap 666.923µs
flurry 5.418522ms
```
