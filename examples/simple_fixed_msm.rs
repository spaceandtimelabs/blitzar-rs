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
use rand_core::OsRng;

fn main() {
    /////////////////////////////////////////////
    // Create some random generators for the MSM
    /////////////////////////////////////////////
    let mut rng = OsRng;
    let generators: Vec<RistrettoPoint> =
        (0..3).map(|_| RistrettoPoint::random(&mut rng)).collect();

    /////////////////////////////////////////////
    // Create a handle from the generators to compute
    // MSMs.
    //
    // This step can be expensive, but we only need
    // to do it once and it saves us time if we
    // end up computing many MSMs with the same set
    // of generators.
    /////////////////////////////////////////////
    let handle = MsmHandle::new(&generators);

    /////////////////////////////////////////////
    // Define an array of scalars for the MSM
    //
    // We'll use the 2x3 scalar array
    //    1 3 5
    //    2 4 6
    // which will correspond to the MSM
    //  1 * g[0] + 3 * g[1] + 5 * g[2]
    //  2 * g[0] + 4 * g[4] + 6 * g[2]
    /////////////////////////////////////////////
    let data: &[u8] = &[1, 2, 3, 4, 5, 6];

    /////////////////////////////////////////////
    // Define a vector to store the MSM result
    /////////////////////////////////////////////
    let mut res = vec![RistrettoPoint::default(); 2];

    /////////////////////////////////////////////
    // Do the actual MSM computation
    /////////////////////////////////////////////
    handle.msm(&mut res, 1, data);

    /////////////////////////////////////////////
    // Print result
    /////////////////////////////////////////////
    for (i, resi) in res.iter().enumerate() {
        println!("result {}: {:?}\n", i, resi);
    }
}
