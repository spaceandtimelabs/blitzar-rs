// Copyright 2023-present Space and Time Labs, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ark_bls12_381::{Fr, G1Affine};
use ark_ff::BigInt;
use ark_std::UniformRand;

use criterion::{criterion_group, criterion_main, Criterion};

extern crate rand;
use crate::rand::Rng;

use blitzar::compute::*;
use blitzar::sequence::*;

mod blitzar_bls12_381_benchmarks {
    use ark_ff::PrimeField;

    use super::*;

    fn construct_scalars_data(num_commits: usize, num_rows: usize) -> Vec<Vec<BigInt<4>>> {
        let mut rng = ark_std::test_rng();

        (0..num_commits)
            .map(|_| {
                (0..num_rows)
                    .map(|_| Fr::rand(&mut rng).into_bigint())
                    .collect()
            })
            .collect()
    }

    fn construct_sequences_data(num_commits: usize, num_rows: usize) -> Vec<Vec<u8>> {
        let mut rng = rand::thread_rng();

        (0..num_commits)
            .map(|_| ((0..num_rows).map(|_| rng.gen::<u8>()).collect()))
            .collect()
    }

    fn construct_generators(num_commits: usize) -> Vec<G1Affine> {
        let mut rng = ark_std::test_rng();
        (0..num_commits).map(|_| G1Affine::rand(&mut rng)).collect()
    }

    fn run_computation(num_commits: usize, num_rows: usize, c: &mut Criterion, use_scalars: bool) {
        let generators = construct_generators(num_rows);
        let mut commitments = vec![[0_u8; 48]; num_commits];

        let benchmark_label: String = "bls12_381_g1 ".to_string();
        let num_commits_label: String = num_commits.to_string() + " commits";
        let benchmark_group_label: String = benchmark_label + &num_commits_label;

        let with_generators_label: String = num_rows.to_string()
            + " rows"
            + " - use scalars ("
            + if use_scalars { "yes" } else { "no" }
            + ")";

        let mut group = c.benchmark_group(&benchmark_group_label);

        group.throughput(criterion::Throughput::Elements(
            (num_commits * num_rows) as u64,
        ));

        if use_scalars {
            let data = construct_scalars_data(num_commits, num_rows);
            let table: Vec<Sequence> = data
                .iter()
                .map(|v| Sequence::from_raw_parts(v.as_slice(), false))
                .collect();

            group.bench_function(&with_generators_label, |b| {
                b.iter(|| {
                    compute_bls12_381_g1_commitments_with_generators(
                        &mut commitments,
                        &table,
                        &generators,
                    )
                })
            });
        } else {
            let data = construct_sequences_data(num_commits, num_rows);
            let table: Vec<Sequence> = (0..num_commits).map(|i| (&data[i]).into()).collect();

            group.bench_function(&with_generators_label, |b| {
                b.iter(|| {
                    compute_bls12_381_g1_commitments_with_generators(
                        &mut commitments,
                        &table,
                        &generators,
                    )
                })
            });
        }

        group.finish();
    }

    fn batch_commitment_computation_with_scalars(c: &mut Criterion) {
        init_backend();

        let bench_runs = vec![
            (1, vec![1, 10, 100, 1000, 10000, 100000]), // 1 commits
            (10, vec![10, 100, 1000]),                  // 10 commits
            (100, vec![10, 100, 1000]),                 // 100 commits
            (1000, vec![10, 100, 1000]),                // 1000 commits
        ];

        // iterate through the num_commits
        for curr_bench in bench_runs {
            // iterate through the num_rows
            for i in curr_bench.1 {
                run_computation(curr_bench.0, i, c, false);
                run_computation(curr_bench.0, i, c, true);
            }
        }
    }

    criterion_group! {
        name = blitzar_compute_bls12_381_g1_commitments;
        // Lower the sample size to run the benchmarks faster
        config = Criterion::default().sample_size(15);
        targets =
            batch_commitment_computation_with_scalars
    }
}

criterion_main!(blitzar_bls12_381_benchmarks::blitzar_compute_bls12_381_g1_commitments);
