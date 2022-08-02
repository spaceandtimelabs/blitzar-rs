// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>
use super::backend::init_backend;
use curve25519_dalek::ristretto::RistrettoPoint;

#[doc = include_str!("../../docs/commitments/get_generators.md")]
///
/// # Example - Getting the Generators used in the `compute_commitments` function
//
/// ```no_run
#[doc = include_str!("../../examples/get_generators.rs")]
/// ```
pub fn get_generators(generators: &mut [RistrettoPoint], offset_generators: u64) {
    init_backend();

    unsafe {
        let sxt_ristretto_generators = generators.as_mut_ptr() as *mut proofs_gpu::sxt_ristretto;

        let ret_get_generators = proofs_gpu::sxt_get_generators(
            sxt_ristretto_generators,
            generators.len() as u64,
            offset_generators,
        );

        if ret_get_generators != 0 {
            panic!("Error during get_generators call");
        }
    }
}
