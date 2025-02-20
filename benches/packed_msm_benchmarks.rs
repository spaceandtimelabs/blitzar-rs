// Copyright 2024-present Space and Time Labs, Inc.
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

extern crate rand;

use crate::rand::Rng;
use ark_bls12_381::G1Affine as Bls12381G1Affine;
use ark_bn254::G1Affine as Bn254G1Affine;
use ark_std::UniformRand;
use blitzar::compute::*;
use criterion::{Criterion, criterion_group, criterion_main};
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;

mod packed_msm_benches {
    use super::*;

    fn bls12_381_msm_benchmark(
        c: &mut Criterion,
        num_commits: usize,
        num_rows: usize,
        bench_runs_bit_size: &[u32],
    ) {
        let mut rng = ark_std::test_rng();
        let mut res = vec![ElementP2::<ark_bls12_381::g1::Config>::default(); num_commits];

        let generators: Vec<ElementP2<ark_bls12_381::g1::Config>> = (0..num_rows)
            .map(|_| Bls12381G1Affine::rand(&mut rng).into())
            .collect();

        let handle = MsmHandle::new(&generators);

        for &curr_bench_bit_size in bench_runs_bit_size {
            let bit_size: u32 = curr_bench_bit_size;

            let output_bit_table: Vec<u32> = vec![bit_size; num_rows];

            let scalars: Vec<u8> = if bit_size > 1 {
                (0..num_rows * bit_size as usize)
                    .map(|_| rng.r#gen::<u8>())
                    .collect()
            } else {
                (0..num_rows).map(|_| 255).collect()
            };

            let label = format!(
                "bls12-381 g1 packed_msm_benchmark - {} commits, {} rows, {} bit size",
                num_commits, num_rows, bit_size
            );

            c.bench_function(&label, |b| {
                b.iter(|| handle.packed_msm(&mut res, &output_bit_table, &scalars))
            });
        }
    }

    fn bn254_msm_benchmark(
        c: &mut Criterion,
        num_commits: usize,
        num_rows: usize,
        bench_runs_bit_size: &[u32],
    ) {
        let mut rng = ark_std::test_rng();
        let mut res = vec![ElementP2::<ark_bn254::g1::Config>::default(); num_commits];

        let generators: Vec<ElementP2<ark_bn254::g1::Config>> = (0..num_rows)
            .map(|_| Bn254G1Affine::rand(&mut rng).into())
            .collect();

        let handle = MsmHandle::new(&generators);

        for &curr_bench_bit_size in bench_runs_bit_size {
            let bit_size: u32 = curr_bench_bit_size;

            let output_bit_table: Vec<u32> = vec![bit_size; num_rows];

            let scalars: Vec<u8> = if bit_size > 1 {
                (0..num_rows * bit_size as usize)
                    .map(|_| rng.r#gen::<u8>())
                    .collect()
            } else {
                (0..num_rows).map(|_| 255).collect()
            };

            let label = format!(
                "bn254 g1 packed_msm_benchmark - {} commits, {} rows, {} bit size",
                num_commits, num_rows, bit_size
            );

            c.bench_function(&label, |b| {
                b.iter(|| handle.packed_msm(&mut res, &output_bit_table, &scalars))
            });
        }
    }

    fn curve25519_msm_benchmark(
        c: &mut Criterion,
        num_commits: usize,
        num_rows: usize,
        bench_runs_bit_size: &[u32],
    ) {
        let mut rng = OsRng;
        let mut res = vec![RistrettoPoint::default(); num_commits];

        let generators: Vec<RistrettoPoint> = (0..num_rows)
            .map(|_| RistrettoPoint::random(&mut rng))
            .collect();

        let handle = MsmHandle::new(&generators);

        for &curr_bench_bit_size in bench_runs_bit_size {
            let bit_size: u32 = curr_bench_bit_size;

            let output_bit_table: Vec<u32> = vec![bit_size; num_rows];

            let scalars: Vec<u8> = if bit_size > 1 {
                (0..num_rows * bit_size as usize)
                    .map(|_| rng.r#gen::<u8>())
                    .collect()
            } else {
                (0..num_rows).map(|_| 255).collect()
            };

            let label = format!(
                "curve25519 packed_msm_benchmark - {} commits, {} rows, {} bit size",
                num_commits, num_rows, bit_size
            );

            c.bench_function(&label, |b| {
                b.iter(|| handle.packed_msm(&mut res, &output_bit_table, &scalars))
            });
        }
    }

    fn packed_msm_commitment_computation(c: &mut Criterion) {
        init_backend();

        let num_commits = 1024;
        let num_rows = 1024;
        let bench_runs_bit_size = vec![256, 128, 64, 32, 16, 8, 1];

        bls12_381_msm_benchmark(c, num_commits, num_rows, &bench_runs_bit_size);
        bn254_msm_benchmark(c, num_commits, num_rows, &bench_runs_bit_size);
        curve25519_msm_benchmark(c, num_commits, num_rows, &bench_runs_bit_size);
    }

    criterion_group! {
        name = blitzar_packed_msm_commitments;
        // Lower the sample size to run the benchmarks faster
        config = Criterion::default().sample_size(15);
        targets = packed_msm_commitment_computation
    }
}

criterion_main!(packed_msm_benches::blitzar_packed_msm_commitments);
