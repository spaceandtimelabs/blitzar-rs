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
use super::backend::init_backend;
use curve25519_dalek::ristretto::RistrettoPoint;
use std::mem::MaybeUninit;

#[doc = include_str!("../../docs/commitments/get_curve25519_generators.md")]
///
/// # Example - Getting the Generators used in the `compute_curve25519_commitments` function
/// ```no_run
#[doc = include_str!("../../examples/get_generators.rs")]
/// ```
pub fn get_curve25519_generators(generators: &mut [RistrettoPoint], offset_generators: u64) {
    init_backend();

    unsafe {
        let sxt_ristretto_generators =
            generators.as_mut_ptr() as *mut blitzar_sys::sxt_ristretto255;

        let ret_get_generators = blitzar_sys::sxt_ristretto255_get_generators(
            sxt_ristretto_generators,
            generators.len() as u64,
            offset_generators,
        );

        if ret_get_generators != 0 {
            panic!("Error during get_curve25519_generators call");
        }
    }
}

#[doc = include_str!("../../docs/commitments/get_one_curve25519_commit.md")]
///
/// # Example - Getting the `n`-th One Commit
/// ```no_run
#[doc = include_str!("../../examples/get_one_commit.rs")]
/// ```
pub fn get_one_curve25519_commit(n: u64) -> RistrettoPoint {
    init_backend();

    unsafe {
        let mut one_commit: MaybeUninit<RistrettoPoint> = MaybeUninit::uninit();
        let one_commit_ptr = one_commit.as_mut_ptr() as *mut blitzar_sys::sxt_ristretto255;

        let ret_get_one_commit = blitzar_sys::sxt_curve25519_get_one_commit(one_commit_ptr, n);

        if ret_get_one_commit != 0 {
            panic!("Error during get_one_curve25519_commit call");
        }

        one_commit.assume_init()
    }
}
