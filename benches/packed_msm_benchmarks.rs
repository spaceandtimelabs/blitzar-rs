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

extern crate rand;
use crate::rand::Rng;
use curve25519_dalek::ristretto::RistrettoPoint;

use criterion::{criterion_group, criterion_main, Criterion};

use blitzar::compute::*;

use rand_core::OsRng;

mod packed_msm_benches {
    use super::*;

    fn packed_msm_commitment_computation(c: &mut Criterion) {
        init_backend();

        let mut rng = OsRng;
        let num_commits = 1024;
        let num_rows = 1024;

        let mut res = vec![RistrettoPoint::default(); num_commits];

        let generators: Vec<RistrettoPoint> = (0..num_rows)
            .map(|_| RistrettoPoint::random(&mut rng))
            .collect();

        let handle = MsmHandle::new(&generators);

        let bench_runs_bit_size = vec![256, 128, 64, 32, 16, 8, 1];

        for curr_bench_bit_size in bench_runs_bit_size {
            let bit_size: u32 = curr_bench_bit_size;

            let output_bit_table: Vec<u32> = vec![bit_size; num_rows];

            let scalars: Vec<u8> = if bit_size > 1 {
                (0..num_rows * bit_size as usize)
                    .map(|_| rng.gen::<u8>())
                    .collect()
            } else {
                (0..num_rows).map(|_| 255).collect()
            };

            let label = format!(
                "New Benchmark - {} commits, {} rows, {} bit size",
                num_commits, num_rows, bit_size
            );

            c.bench_function(&label, |b| {
                b.iter(|| handle.packed_msm(&mut res, &output_bit_table, &scalars))
            });
        }
    }

    criterion_group! {
        name = blitzar_packed_msm_commitments;
        // Lower the sample size to run the benchmarks faster
        config = Criterion::default().sample_size(15);
        targets = packed_msm_commitment_computation
    }
}

criterion_main!(packed_msm_benches::blitzar_packed_msm_commitments);
