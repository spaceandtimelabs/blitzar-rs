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

fn main() {
    /////////////////////////////////////////////
    // Initialize the backend
    /////////////////////////////////////////////
    init_backend();

    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation
    // and do the actual commitment computation
    /////////////////////////////////////////////
    let data: &[u64] = &[40, 32, 21, 10, 20, 35, 444];
    let mut commitments = vec![Default::default(); 1];
    compute_curve25519_commitments(&mut commitments, &[data.into()], 0_u64);
}
