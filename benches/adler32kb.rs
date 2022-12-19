use std::io::Write;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use hash::{adler32, Hash};

fn benchmark(c: &mut Criterion) {
    let data = {
        let mut v = [0u8; 1024];
        for i in 0..v.len() {
            v[i] = i as u8;
        }
        v
    };

    let mut h = adler32::new();
    let input = vec![0u8; h.size() as usize];

    // it seems overhead of setup is neglectable.
    let mut f = |v: Option<Vec<u8>>| {
        h.reset();
        let _ = h.write(&data).expect("write");
        h.sum(v);
    };

    let mut grp = c.benchmark_group("adler32");
    grp.throughput(Throughput::Bytes((data.len() + input.len()) as u64));
    grp.bench_function("adler32kb", |b| {
        b.iter_batched(
            || Some(input.clone()),
            |v| f(v),
            criterion::BatchSize::SmallInput,
        );
    });
    grp.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
