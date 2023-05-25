// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>
use super::backend::init_backend;
use curve25519_dalek::ristretto::RistrettoPoint;
use std::mem::MaybeUninit;

#[doc = include_str!("../../docs/commitments/get_generators.md")]
///
/// # Example - Getting the Generators used in the `compute_commitments` function
//
/// ```no_run
#[doc = include_str!("../../examples/get_generators.rs")]
/// ```
#[tracing::instrument(name = "compute.generators.get_generators", level = "info", skip_all)]
pub fn get_generators(generators: &mut [RistrettoPoint], offset_generators: u64) {
    init_backend();

    unsafe {
        let sxt_ristretto_generators = generators.as_mut_ptr() as *mut blitzar_sys::sxt_ristretto;

        let ret_get_generators = blitzar_sys::sxt_get_generators(
            sxt_ristretto_generators,
            generators.len() as u64,
            offset_generators,
        );

        if ret_get_generators != 0 {
            panic!("Error during get_generators call");
        }
    }
}

#[doc = include_str!("../../docs/commitments/get_one_commit.md")]
///
/// # Example - Getting the i-th One Commit
//
/// ```no_run
#[doc = include_str!("../../examples/get_one_commit.rs")]
/// ```
#[tracing::instrument(name = "compute.generators.get_one_commit", level = "info", skip_all)]
pub fn get_one_commit(n: u64) -> RistrettoPoint {
    init_backend();

    unsafe {
        let mut one_commit: MaybeUninit<RistrettoPoint> = MaybeUninit::uninit();
        let one_commit_ptr = one_commit.as_mut_ptr() as *mut blitzar_sys::sxt_ristretto;

        let ret_get_one_commit = blitzar_sys::sxt_get_one_commit(one_commit_ptr, n);

        if ret_get_one_commit != 0 {
            panic!("Error during get_one_commit call");
        }

        one_commit.assume_init()
    }
}
