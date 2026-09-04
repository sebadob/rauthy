// Measures OUR pipeline - the worker pool, bounded channel and dispatch of
// password_hasher - not the argon2-rust library. N hash messages round-trip
// through the real machinery (run() + hash_worker + HASH_CHANNELS); the per-hash
// cost is the real default profile (64 MiB / t=3 / p=8) so the worker-pool
// concurrency benefit is visible alongside the pipeline overhead.
//
// Run: BENCH_HASH_THREADS=1 cargo bench --bench hashing   (memory default)
//      BENCH_HASH_THREADS=2 cargo bench --bench hashing   (speed preset)
use argon2_rust::Params;
use argon2_rust::params::{Memory, TagLen};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rauthy_common::password_hasher::{
    ARGON2_PARAMS, HASH_AWAIT_WARN_TIME, HASH_CHANNELS, HASH_THREADS, HashPassword, run,
};
use tokio::runtime::Runtime;

fn setup(threads: usize) -> Runtime {
    let params = Params::builder()
        .memory(Memory::kib(64 * 1024))
        .passes(3)
        .lanes(8)
        .tag_len(TagLen::bytes(32))
        .build_or_panic();
    let _ = ARGON2_PARAMS.set(params);
    let _ = HASH_AWAIT_WARN_TIME.set(10_000);
    let _ = HASH_CHANNELS.set(flume::bounded(threads));
    let _ = HASH_THREADS.set(threads);
    let rt = Runtime::new().unwrap();
    rt.spawn(run());
    rt
}

fn bench_pipeline(c: &mut Criterion) {
    let threads = std::env::var("BENCH_HASH_THREADS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let _rt = setup(threads);
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("hash_pipeline");
    group.sample_size(10);
    for batch in [1usize, 4] {
        group.bench_function(format!("threads={threads} batch={batch}"), |b| {
            b.iter(|| {
                let v = rt.block_on(async {
                    let mut v = Vec::with_capacity(batch);
                    for _ in 0..batch {
                        v.push(
                            HashPassword::hash_password("bench-password".to_string())
                                .await
                                .unwrap(),
                        );
                    }
                    v
                });
                black_box(v)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_pipeline);
criterion_main!(benches);
