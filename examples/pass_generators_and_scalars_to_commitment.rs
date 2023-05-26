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
extern crate blitzar;
extern crate curve25519_dalek;

use blitzar::compute::*;

extern crate rand_core;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;

fn main() {
    // generate input table
    let mut table: Vec<&[Scalar]> = Vec::new();

    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation. Each vector
    // is a Dalek Scalar, which is simply a 256-bit integer < ℓ (the group prime order)
    //
    // For instance:
    //     commitment[0] = gs[0]*data[0] + gs[1]*data[1] + gs[2]*data[2] + gs[3]^data[3]
    //
    // You must provide a generator vector `gs` to the computation here.
    //
    // Consult the Dalek Scalar documentation for more info
    // [here](https://docs.rs/curve25519-dalek/0.19.1/curve25519_dalek/scalar/index.html).
    // In summary,
    // Scalar::from_bytes_mod_order([2; 32]) wrapps a 32 byte array, containing
    // the number 2 at every byte, reducing this 256-bit integer mod ℓ, the prime order
    // of the group.
    /////////////////////////////////////////////
    let data: Vec<Scalar> = vec![
        Scalar::from_bytes_mod_order([2; 32]),
        Scalar::from_bytes_mod_order([3; 32]),
        Scalar::from_bytes_mod_order([1; 32]),
        Scalar::from_bytes_mod_order([10; 32]),
    ];

    /////////////////////////////////////////////
    // randomly obtain the generator points
    /////////////////////////////////////////////
    let mut rng = OsRng;
    let gs: Vec<RistrettoPoint> = (0..data.len())
        .map(|_| RistrettoPoint::random(&mut rng))
        .collect();

    /////////////////////////////////////////////
    // Fill the table with entries
    /////////////////////////////////////////////
    table.push(&data);

    /////////////////////////////////////////////
    // We need to define a commitment vector which
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); table.len()];

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_commitments_with_generators(&mut commitments, &table, &gs);

    /////////////////////////////////////////////
    // Use Dalek library to obtain the same
    // commitment that was computed in the GPU or
    // CPU above. Following, we randomly
    // obtain the generators
    /////////////////////////////////////////////
    let mut expected_commit = RistrettoPoint::from_uniform_bytes(&[0_u8; 64]);

    /////////////////////////////////////////////
    // Then we use the above generators `gs`,
    // as well as the data table as scalars
    // to obtain the same commitment that
    // was computed in the GPU / CPU, the `expected_commit`
    /////////////////////////////////////////////
    for i in 0..gs.len() {
        let g_i = gs[i];

        expected_commit += data[i] * g_i;
    }

    /////////////////////////////////////////////
    // Compare the Dalek and our CPU/GPU commitment
    /////////////////////////////////////////////
    println!("Computed Commitment: {:?}\n", commitments[0]);
    println!("Expected Commitment: {:?}\n", expected_commit.compress());
}
