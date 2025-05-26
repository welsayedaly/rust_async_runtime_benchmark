use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn tokio_runtime_benchmark(c: &mut Criterion) {
    c.bench_function("tokio_spawn_1000", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                let mut handles = Vec::new();
                for i in 0..1000 {
                    let handle = tokio::spawn(async move {
                        fibonacci(black_box(i % 20))
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.await.unwrap();
                }
            });
        });
    });
}

fn async_std_runtime_benchmark(c: &mut Criterion) {
    c.bench_function("async_std_spawn_1000", |b| {
        b.iter(|| {
            async_std::task::block_on(async {
                let mut handles = Vec::new();
                for i in 0..1000 {
                    let handle = async_std::task::spawn(async move {
                        fibonacci(black_box(i % 20))
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.await;
                }
            });
        });
    });
}

fn smol_runtime_benchmark(c: &mut Criterion) {
    c.bench_function("smol_spawn_1000", |b| {
        b.iter(|| {
            smol::block_on(async {
                let mut handles = Vec::new();
                for i in 0..1000 {
                    let handle = smol::spawn(async move {
                        fibonacci(black_box(i % 20))
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.await;
                }
            });
        });
    });
}

criterion_group!(
    benches,
    tokio_runtime_benchmark,
    async_std_runtime_benchmark,
    smol_runtime_benchmark
);
criterion_main!(benches);