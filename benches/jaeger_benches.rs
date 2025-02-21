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

//! Benchmarking/Tracing using Jaeger.
//! To run, execute the following commands:
//! ```bash
//! docker run --rm -d --name jaeger -p 6831:6831/udp -p 16686:16686 jaegertracing/all-in-one:1.62.0
//! cargo bench --bench jaeger_benches all
//! cargo bench --bench jaeger_benches <BENCHMARK_TYPES>
//! ```
//! Then, navigate to <http://localhost:16686> to view the traces.

extern crate blitzar;
use ark_bls12_381::G1Affine as Bls12381G1Affine;
use ark_bn254::G1Affine as Bn254G1Affine;
use ark_grumpkin::Affine as GrumpkinAffine;
use ark_std::UniformRand;
use blitzar::sequence::Sequence;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use halo2curves::bn256::{G1 as Halo2Bn256G1Projective, G1Affine as Halo2Bn256G1Affine};
use rand::Rng;
use rand_core::OsRng;
use std::env;

const LENGTH: [usize; 2] = [1024, 1 << 20];
const NUM_OUTPUTS: [usize; 2] = [1024, 1];
const ITERATIONS: usize = 3;
const BENCHMARK_TYPES: &[&str] = &[
    "compute_curve25519_commitments_with_generators",
    "compute_bls12_381_g1_commitments_with_generators",
    "compute_bn254_g1_uncompressed_commitments_with_generators",
    "compute_bn254_g1_uncompressed_commitments_with_halo2_generators",
    "compute_grumpkin_uncompressed_commitments_with_generators",
];

fn run_benchmark(benchmark_type: &str) {
    match benchmark_type {
        "compute_curve25519_commitments_with_generators" => {
            for (length, num_outputs) in LENGTH.iter().zip(NUM_OUTPUTS.iter()) {
                let mut rng = OsRng;

                // Generate random data
                let scalars: Vec<Vec<u64>> = (0..*num_outputs)
                    .map(|_| {
                        (0..*length)
                            .map(|_| rng.gen_range(u64::MIN..u64::MAX))
                            .collect()
                    })
                    .collect();

                let data: Vec<Sequence> = scalars
                    .iter()
                    .map(|v| Sequence::from_raw_parts(v.as_slice(), false))
                    .collect();

                // Generate random generators
                let generators: Vec<RistrettoPoint> = (0..*length)
                    .map(|_| RistrettoPoint::random(&mut rng))
                    .collect();

                // Create commitments
                let mut commitments = vec![CompressedRistretto::default(); *num_outputs];

                for _ in 0..ITERATIONS {
                    blitzar::compute::compute_curve25519_commitments_with_generators(
                        &mut commitments,
                        &data,
                        &generators,
                    );
                }
            }
        }
        "compute_bls12_381_g1_commitments_with_generators" => {
            for (length, num_outputs) in LENGTH.iter().zip(NUM_OUTPUTS.iter()) {
                let mut rng = ark_std::test_rng();

                // Generate random data
                let scalars: Vec<Vec<u64>> = (0..*num_outputs)
                    .map(|_| {
                        (0..*length)
                            .map(|_| rng.gen_range(u64::MIN..u64::MAX))
                            .collect()
                    })
                    .collect();

                let data: Vec<Sequence> = scalars
                    .iter()
                    .map(|v| Sequence::from_raw_parts(v.as_slice(), false))
                    .collect();

                // Generate random generators
                let generators: Vec<Bls12381G1Affine> = (0..*length)
                    .map(|_| Bls12381G1Affine::rand(&mut rng))
                    .collect();

                // Create commitments
                let mut commitments = vec![[0_u8; 48]; *num_outputs];

                for _ in 0..ITERATIONS {
                    blitzar::compute::compute_bls12_381_g1_commitments_with_generators(
                        &mut commitments,
                        &data,
                        &generators,
                    );
                }
            }
        }
        "compute_bn254_g1_uncompressed_commitments_with_generators" => {
            for (length, num_outputs) in LENGTH.iter().zip(NUM_OUTPUTS.iter()) {
                let mut rng = ark_std::test_rng();

                // Generate random data
                let scalars: Vec<Vec<u64>> = (0..*num_outputs)
                    .map(|_| {
                        (0..*length)
                            .map(|_| rng.gen_range(u64::MIN..u64::MAX))
                            .collect()
                    })
                    .collect();

                let data: Vec<Sequence> = scalars
                    .iter()
                    .map(|v| Sequence::from_raw_parts(v.as_slice(), false))
                    .collect();

                // Generate random generators
                let generators: Vec<Bn254G1Affine> = (0..*length)
                    .map(|_| Bn254G1Affine::rand(&mut rng))
                    .collect();

                // Create commitments
                let mut commitments = vec![Bn254G1Affine::default(); *num_outputs];

                for _ in 0..ITERATIONS {
                    blitzar::compute::compute_bn254_g1_uncompressed_commitments_with_generators(
                        &mut commitments,
                        &data,
                        &generators,
                    );
                }
            }
        }
        "compute_bn254_g1_uncompressed_commitments_with_halo2_generators" => {
            for (length, num_outputs) in LENGTH.iter().zip(NUM_OUTPUTS.iter()) {
                let mut rng = ark_std::test_rng();

                // Generate random data
                let scalars: Vec<Vec<u64>> = (0..*num_outputs)
                    .map(|_| {
                        (0..*length)
                            .map(|_| rng.gen_range(u64::MIN..u64::MAX))
                            .collect()
                    })
                    .collect();

                let data: Vec<Sequence> = scalars
                    .iter()
                    .map(|v| Sequence::from_raw_parts(v.as_slice(), false))
                    .collect();

                // Generate random generators
                let generators: Vec<Halo2Bn256G1Affine> = (0..*length)
                    .map(|_| Halo2Bn256G1Affine::random(&mut rng))
                    .collect();

                // Create commitments
                let mut commitments = vec![Halo2Bn256G1Projective::default(); *num_outputs];

                for _ in 0..3 {
                    blitzar::compute::compute_bn254_g1_uncompressed_commitments_with_halo2_generators(
                        &mut commitments,
                        &data,
                        &generators,
                    );
                }
            }
        }
        "compute_grumpkin_uncompressed_commitments_with_generators" => {
            for (length, num_outputs) in LENGTH.iter().zip(NUM_OUTPUTS.iter()) {
                let mut rng = ark_std::test_rng();

                // Generate random data
                let scalars: Vec<Vec<u64>> = (0..*num_outputs)
                    .map(|_| {
                        (0..*length)
                            .map(|_| rng.gen_range(u64::MIN..u64::MAX))
                            .collect()
                    })
                    .collect();

                let data: Vec<Sequence> = scalars
                    .iter()
                    .map(|v| Sequence::from_raw_parts(v.as_slice(), false))
                    .collect();

                // Generate random generators
                let generators: Vec<GrumpkinAffine> = (0..*length)
                    .map(|_| GrumpkinAffine::rand(&mut rng))
                    .collect();

                // Create commitments
                let mut commitments = vec![GrumpkinAffine::default(); *num_outputs];

                for _ in 0..ITERATIONS {
                    blitzar::compute::compute_grumpkin_uncompressed_commitments_with_generators(
                        &mut commitments,
                        &data,
                        &generators,
                    );
                }
            }
        }
        _ => panic!("Invalid benchmark type specified."),
    }
}

fn main() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("benches")
        .install_simple()
        .unwrap();

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("DEBUG"));

    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(filter)
        .try_init()
        .unwrap();

    // Check for command-line arguments to select the benchmark type.
    let args: Vec<String> = env::args().collect();
    let benchmark_type = args
        .get(1)
        .expect("Please specify the benchmark type as an argument.");

    if benchmark_type == "all" {
        for &benchmark in BENCHMARK_TYPES {
            run_benchmark(benchmark);
        }
    } else {
        run_benchmark(benchmark_type);
    }
}
