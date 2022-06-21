// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

extern crate rand;
use crate::rand::Rng;
use rand::thread_rng;

use criterion::{criterion_group, criterion_main, Criterion};

use pedersen::sequence::*;
use pedersen::commitments::*;

use curve25519_dalek::constants;

mod pedersen_benches {
    use super::*;
    
    fn construct_scalars_data(num_commits: usize, num_rows: usize) -> Vec<Vec<Scalar>> {
        let mut rng = thread_rng();

        (0..num_commits)
            .map(|_| ((0..num_rows).map(|_| Scalar::random(&mut rng)).collect()))
            .collect()
    }

    fn construct_sequences_data(num_commits: usize, num_rows: usize) -> Vec<Vec<u32>> {
        let mut rng = rand::thread_rng();

        (0..num_commits)
            .map(|_| ((0..num_rows).map(|_| rng.gen::<u32>()).collect()))
            .collect()
            
    }

    fn construct_generators(n: usize) -> Vec<CompressedRistretto> {
        let mut rng = thread_rng();
        (0..n)
            .map(|_| (&Scalar::random(&mut rng) * &constants::RISTRETTO_BASEPOINT_TABLE).compress())
            .collect()
    }

    fn run_computation(
        num_commits: usize, num_rows: usize, c: &mut Criterion, use_scalars: bool) {

        let generators = construct_generators(num_rows);
        let mut commitments = vec![CompressedRistretto::from_slice(&[0 as u8; 32]); num_commits];

        let num_commits_label: String = num_commits.to_string() + &" commits".to_owned();

        let without_generators_label: String =
            num_rows.to_string() + &" rows".to_owned() +
            &" - use scalars (".to_owned() + if use_scalars {"yes"} else {"no"} +
            &") - use generators (no)".to_owned();

        let wit_generators_label: String =
            num_rows.to_string() + &" rows".to_owned() +
            &" - use scalars (".to_owned() + if use_scalars {"yes"} else {"no"} +
            &") - use generators (yes)".to_owned();
        
        let mut group = c.benchmark_group(&num_commits_label);
        
        group.throughput(criterion::Throughput::Elements((num_commits * num_rows) as u64));
        
        if use_scalars {
            let data = construct_scalars_data(num_commits, num_rows);
            let table: Vec<&[Scalar]> = (0..num_commits).map(|i| (&data[i][..])).collect();

            group.bench_function(
                &without_generators_label, |b| {
                    b.iter(
                        || compute_commitments(
                            & mut commitments, &table
                        )
                    )
                }
            );
    
            group.bench_function(
                &wit_generators_label, |b| {
                    b.iter(
                        || compute_commitments_with_generators(
                            & mut commitments, &table, &generators
                        )
                    )
                }
            );
        } else {
            let data = construct_sequences_data(num_commits, num_rows);
            let table: Vec<Sequence> = (0..num_commits).map(|i| (
                Sequence::Dense(DenseSequence {
                    data_slice: &data[i].as_byte_slice(),
                    element_size: std::mem::size_of_val(&data[i][0])
                }))
            ).collect();

            group.bench_function(
                &without_generators_label, |b| {
                    b.iter(
                        || compute_commitments(
                            & mut commitments, &table
                        )
                    )
                }
            );

            group.bench_function(
                &wit_generators_label, |b| {
                    b.iter(
                        || compute_commitments_with_generators(
                            & mut commitments, &table, &generators
                        )
                    )
                }
            );
        }

        group.finish();
    }
    
    fn batch_commitment_computation_with_scalars(c: &mut Criterion) {
        init_backend();

        let bench_runs = vec![
            (1, vec![1, 10, 100, 1000, 10000, 100000, 1000000]), // 1 commits
            (10, vec![10, 100, 1000]), // 10 commits
            (100, vec![10, 100, 1000]), // 100 commits
            (1000, vec![10, 100, 1000]), // 1000 commits
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
        name = pedersen_compute_commitments;
        // Lower the sample size to run the benchmarks faster
        config = Criterion::default().sample_size(15);
        targets =
            batch_commitment_computation_with_scalars
    }
}

criterion_main!(
    pedersen_benches::pedersen_compute_commitments
);
