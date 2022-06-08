// -*- mode: rust; -*-
//
// Authors:
// - Joe <jose@spaceandtime.io>
// - Ryan Burn <ryan@spaceandtime.io>

use criterion::{criterion_group, criterion_main, Criterion};

use pedersen::sequence::*;
use pedersen::commitments::*;

mod pedersen_benches {
    use super::*;

    fn run_benchmark(num_commits: usize, num_rows: usize, c: &mut Criterion) {
        init_backend();

        let mut data: Vec<u32> = Vec::with_capacity(num_commits*num_rows);
        let mut table = Vec::with_capacity(num_commits);
        let mut commitments = Vec::with_capacity(num_commits);

        for i in 0..(num_commits * num_rows) {
            data.push(i as u32);
        }

        for i in 0..num_commits {
            table.push(Sequence::Dense(DenseSequence {
                data_slice: &data[i * num_rows .. (i + 1) * num_rows].as_byte_slice(),
                element_size: std::mem::size_of::<u32>()
            }));

            commitments.push(CompressedRistretto::from_slice(&[0 as u8; 32]));
        }

        let label1: String = num_commits.to_string() + &" commits".to_owned();
        let label2: String = num_rows.to_string() + &" rows".to_owned();
        
        let mut group = c.benchmark_group(&label1);
        group.throughput(criterion::Throughput::Elements((num_commits * num_rows) as u64));
        group.measurement_time(std::time::Duration::from_micros(1));
        group.sample_size(10);
        group.sampling_mode(criterion::SamplingMode::Flat);
        group.bench_function(&label2, |b| b.iter(|| compute_commitments(& mut commitments, &table)));
        group.finish();
    }

    fn batch_1_commitment_computation(c: &mut Criterion) {
        let rows = vec![1, 10, 100, 1000, 10000, 100000, 1000000];

        for i in rows {
            run_benchmark(1, i, c);
        }
    }

    fn batch_10_commitments_computation(c: &mut Criterion) {
        let rows = vec![10, 100, 1000];

        for i in rows {
            run_benchmark(10, i, c);
        }
    }

    fn batch_100_commitments_computation(c: &mut Criterion) {
        let rows = vec![10, 100, 1000];

        for i in rows {
            run_benchmark(100, i, c);
        }
    }

    fn batch_1000_commitments_computation(c: &mut Criterion) {
        let rows = vec![10, 100, 1000];

        for i in rows {
            run_benchmark(1000, i, c);
        }
    }

    criterion_group! {
        name = pedersen_benches;
        config = Criterion::default();
        targets =
            batch_1_commitment_computation,
            batch_10_commitments_computation,
            batch_100_commitments_computation,
            batch_1000_commitments_computation
    }
}

criterion_main!(
    pedersen_benches::pedersen_benches
);
