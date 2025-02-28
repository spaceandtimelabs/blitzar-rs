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

use blitzar::compute::*;
use curve25519_dalek::{ristretto::CompressedRistretto, scalar::Scalar};

fn main() {
    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation. Each vector
    // is a Dalek Scalar, which is simply a 256-bit integer < ℓ (the group prime order)
    //
    // For instance:
    //     commitment[0] = g[0]*data1[0] + g[1]*data1[1] + g[2]*data1[2] + g[3]^data1[3]
    //
    /////////////////////////////////////////////
    let mut data: Vec<Scalar> = vec![
        Scalar::ZERO,
        Scalar::ONE,
        Scalar::ZERO,
        Scalar::from_bytes_mod_order([4; 32]),
    ];

    // data[2] = 2000 as 256-bits
    for _i in 0..2000 {
        data[2] += Scalar::ONE;
    }

    /////////////////////////////////////////////
    // We need to define a commitment vector which
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::default(); 1];

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_curve25519_commitments(&mut commitments, &[(&data).into()], 0_u64);

    for (i, commit) in commitments.iter().enumerate() {
        println!("commitment {}: {:?}\n", i, commit);
    }
}
