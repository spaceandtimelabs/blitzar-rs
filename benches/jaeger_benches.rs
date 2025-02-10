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
//! cargo bench --bench jaeger_benches bn254_g1_halo2
//! ```
//! Then, navigate to <http://localhost:16686> to view the traces.

use blitzar;
use halo2curves::bn256::{
    Fr as Halo2Bn256Fr, G1Affine as Halo2Bn256G1Affine, G1 as Halo2Bn256G1Projective,
};
use rand::Rng;
use std::env;

const SIZE: usize = 1 << 20;

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

    match benchmark_type.as_str() {
        "bn254_g1_halo2" => {
            let mut rng = rand::thread_rng();

            // Generate random data
            let data: Vec<u64> = (0..SIZE)
                .map(|_| rng.gen_range(u64::MIN..u64::MAX))
                .collect();

            // Generate random generators
            let generators: Vec<Halo2Bn256G1Affine> = (0..data.len())
                .map(|_| Halo2Bn256G1Affine::random(&mut rng))
                .collect();

            // Create commitments
            let mut commitments = vec![Halo2Bn256G1Projective::default(); 1];

            // Convert data to scalar
            let scalar_data: Vec<Halo2Bn256Fr> =
                data.iter().map(|&d| Halo2Bn256Fr::from(d)).collect();

            for _ in 0..3 {
                blitzar::compute::compute_bn254_g1_uncompressed_commitments_with_halo2_generators(
                    &mut commitments,
                    &[(&scalar_data).into()],
                    &generators,
                );
            }
        }
        _ => panic!("Invalid benchmark type specified."),
    }
}
