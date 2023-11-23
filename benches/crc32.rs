use criterion::{criterion_group, criterion_main, BatchSize, Bencher, Criterion, Throughput};

use hash::crc32;
use hash::Hash32;

fn benchmark(c: &mut Criterion) {
    benchmark_all(c, "poly=IEEE", crc32::new_ieee);
}

fn benchmark_all<H>(c: &mut Criterion, name: &str, new_hash: fn() -> H)
where
    H: Hash32,
{
    for size in [15usize, 40, 512, 1 << 10, 4 << 10, 32 << 10] {
        let sub = if size < 1024 {
            size.to_string()
        } else {
            format!("{}kB", size >> 10)
        };

        let mut grp = c.benchmark_group(format!("{name}/size={sub}"));
        grp.throughput(Throughput::Bytes(size as u64));

        for align in 0usize..=1 {
            grp.bench_function(format!("align={align}"), |b| {
                bench(b, new_hash, size, align)
            });
        }
    }
}

fn bench<H>(b: &mut Bencher, new_hash: fn() -> H, n: usize, alignment: usize)
where
    H: Hash32,
{
    let setup = || {
        let data = vec![0u8; n + alignment];
        let mut data = data[alignment..].to_vec();
        for i in 0..data.len() {
            data[i] = i as u8;
        }

        let mut h = new_hash();

        let input = vec![0u8; h.size()];

        // warm up
        h.reset();
        let _ = h.write(&data);
        h.sum(Some(input.clone()));

        (h, data, Some(input))
    };

    let routine = |(h, data, input): (H, Vec<u8>, Option<Vec<u8>>)| {
        let mut h = h;

        h.reset();
        let _ = h.write(&data);
        h.sum(input);
    };

    b.iter_batched(setup, routine, BatchSize::SmallInput);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
