// Copyright 2025-present Space and Time Labs, Inc.
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

use ark_bn254::G1Affine as Bn254G1Affine;
use blitzar::compute::*;
use criterion::{criterion_group, criterion_main, Criterion};
use halo2curves::bn256::G1Affine as Halo2Bn256G1Affine;

mod arkworks_halo2_interop_benchmarks {
    use super::*;
    use ark_ff::UniformRand;

    fn benchmark_convert_to_ark_bn254_g1_affine(c: &mut Criterion, num_points: usize) {
        let mut rnd = rand::thread_rng();
        let points = (0..num_points)
            .map(|_| Halo2Bn256G1Affine::random(&mut rnd))
            .collect::<Vec<Halo2Bn256G1Affine>>();

        c.bench_function(
            &format!("convert_to_ark_bn254_g1_affine {} points", num_points),
            |b| {
                b.iter(|| {
                    for point in &points {
                        let _ = convert_to_ark_bn254_g1_affine(point);
                    }
                })
            },
        );
    }

    fn benchmark_convert_to_halo2_bn256_g1_affine(c: &mut Criterion, num_points: usize) {
        let mut rnd = rand::thread_rng();
        let points = (0..num_points)
            .map(|_| Bn254G1Affine::rand(&mut rnd))
            .collect::<Vec<Bn254G1Affine>>();

        c.bench_function(
            &format!("convert_to_halo2_bn256_g1_affine {} points", num_points),
            |b| {
                b.iter(|| {
                    for point in &points {
                        let _ = convert_to_halo2_bn256_g1_affine(point);
                    }
                })
            },
        );
    }

    fn compute(c: &mut Criterion) {
        let bench_runs = vec![1024, 8192, 65536, 524288, 1048576, 2097152];
        for num_points in bench_runs {
            benchmark_convert_to_ark_bn254_g1_affine(c, num_points);
            benchmark_convert_to_halo2_bn256_g1_affine(c, num_points);
        }
    }

    criterion_group! {
      name = benchmark_conversion;
      config = Criterion::default().sample_size(15);
      targets = compute
    }
}

criterion_main!(arkworks_halo2_interop_benchmarks::benchmark_conversion);
