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
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::traits::Identity;

fn main() {
    let mut generators = vec![RistrettoPoint::from_uniform_bytes(&[0_u8; 64]); 3];
    get_generators(&mut generators, 0);

    /////////////////////////////////////////////
    // The first one_commit must always be the identity
    /////////////////////////////////////////////
    println!("****************************************************");
    println!(
        "First Commit (identity) is valid: {:?}\n",
        get_one_commit(0) == RistrettoPoint::identity()
    );

    /////////////////////////////////////////////
    // The second one_commit must always be the first generator
    /////////////////////////////////////////////
    println!("****************************************************");
    println!(
        "Second Commit (gen 1) is valid: {:?}\n",
        get_one_commit(1) == generators[0]
    );

    /////////////////////////////////////////////
    // The third one_commit must always be the sum of the first and second generator
    /////////////////////////////////////////////
    println!("****************************************************");
    println!(
        "Third Commit (gen 1 + gen 2) is valid: {:?}",
        get_one_commit(2) == generators[0] + generators[1]
    );

    /////////////////////////////////////////////
    // The i-th one_commit must always be the sum of all generators from 0..(i - 1)
    /////////////////////////////////////////////
}
