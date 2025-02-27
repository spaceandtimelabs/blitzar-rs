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
use curve25519_dalek::{ristretto::RistrettoPoint, traits::Identity};

fn main() {
    let mut generators = vec![Default::default(); 3];
    get_curve25519_generators(&mut generators, 0);

    /////////////////////////////////////////////
    // The first one_commit must always be the identity
    /////////////////////////////////////////////
    assert_eq!(get_one_curve25519_commit(0), RistrettoPoint::identity());

    /////////////////////////////////////////////
    // The second one_commit must always be the first generator
    /////////////////////////////////////////////
    assert_eq!(get_one_curve25519_commit(1), generators[0]);

    /////////////////////////////////////////////
    // The third one_commit must always be the sum of the first and second generator
    /////////////////////////////////////////////
    assert_eq!(get_one_curve25519_commit(2), generators[0] + generators[1]);

    /////////////////////////////////////////////
    // The fourth one_commit must always be the sum of the first through third generators
    /////////////////////////////////////////////
    assert_eq!(
        get_one_curve25519_commit(3),
        generators[0] + generators[1] + generators[2]
    );
}
