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

use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;

fn main() {
    /////////////////////////////////////////////
    // Define the data slice that will be used in the computation. Each slice
    // will be translated into a single 32 bytes scalar data
    //
    //
    // For instance:
    //     commitment[0] = g[0]*data1[0] + g[1]*data1[1] + g[2]*data1[2] + g[3]*data1[3]
    //                   = g[0]*1 + g[1]*2 + g[2]*3 + g[3]*4
    //
    //     commitment[2] = g[0]*data3[0] + g[1]*data3[1] + ... + g[6]*data3[6]
    //                   = g[0]*40 + g[1]*32 + g[2]*21 + g[3]*10 + ... + g[6]*444
    //
    /////////////////////////////////////////////
    let data0: &[u16] = &[1, 2, 3, 4];
    let data1: &[u32] = &[4, 3, 2, 1];
    let data2: &[u64] = &[40, 32, 21, 10, 20, 35, 444];
    let data3: &[Scalar] = &[
        Scalar::from(1u8),
        Scalar::from(2u8),
        Scalar::from(3u8),
        Scalar::from(4u8),
    ];

    /////////////////////////////////////////////
    // We need to define a commitment vector which
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::default(); 4];

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_curve25519_commitments(
        &mut commitments,
        &[data0.into(), data1.into(), data2.into(), data3.into()],
        0,
    );

    assert_eq!(commitments[0], commitments[3]);
    for (i, commit) in commitments.iter().enumerate() {
        println!("commitment {}: {:?}\n", i, commit);
    }
}
