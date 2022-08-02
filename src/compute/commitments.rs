// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

use super::backend::init_backend;
use super::generators::get_generators;
use crate::sequences::{to_sxt_descriptors, Descriptor};
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};

#[doc = include_str!("../../docs/commitments/compute_commitments.md")]
///
/// # Example 1 - Simple Commitment Computation
///```no_run
#[doc = include_str!("../../examples/simple_commitment.rs")]
///```
///
/// # Example 2 - Adding and Multiplying Commitments
///```no_run
#[doc = include_str!("../../examples/add_mult_commitments.rs")]
///```
///
/// # Example 3 - Compute Commitments with Dalek Scalars
///```no_run
#[doc = include_str!("../../examples/simple_scalars_commitment.rs")]
///```
///
/// # Example 4 - Compute Commitments with Sparse Sequences
///```no_run
#[doc = include_str!("../../examples/simple_sparse_commitment.rs")]
///```
pub fn compute_commitments<T: Descriptor>(commitments: &mut [CompressedRistretto], data: &[T]) {
    init_backend();

    let (sxt_descriptors, _longest_row) = to_sxt_descriptors(data);

    unsafe {
        let sxt_compressed_ristretto =
            commitments.as_mut_ptr() as *mut proofs_gpu::sxt_compressed_ristretto;

        // computes the commitments using the lower-level rust sys crate
        let ret_compute = proofs_gpu::sxt_compute_pedersen_commitments(
            sxt_compressed_ristretto,
            sxt_descriptors.len() as u32,
            sxt_descriptors.as_ptr(),
        );

        if ret_compute != 0 {
            panic!("Error during commitments computation");
        }
    }
}

#[doc = include_str!("../../docs/commitments/compute_commitments_with_generators.md")]
///
///# Example 1 - Pass generators to Commitment Computation
///```no_run
#[doc = include_str!("../../examples/pass_generators_to_commitment.rs")]
///```
///
/// Example 2 - Compute Commitments with Dalek Scalars and User Generators
///```no_run
#[doc = include_str!("../../examples/pass_generators_and_scalars_to_commitment.rs")]
///```
pub fn compute_commitments_with_generators<T: Descriptor>(
    commitments: &mut [CompressedRistretto],
    data: &[T],
    generators: &[RistrettoPoint],
) {
    let (sxt_descriptors, longest_row) = to_sxt_descriptors(data);

    assert!(
        longest_row <= generators.len(),
        "generators has a length smaller than the longest sequence in the input data"
    );

    init_backend();

    unsafe {
        let sxt_ristretto_generators = generators.as_ptr() as *const proofs_gpu::sxt_ristretto;

        let sxt_compressed_ristretto =
            commitments.as_mut_ptr() as *mut proofs_gpu::sxt_compressed_ristretto;

        // computes the commitments using the lower-level rust sys crate
        let ret_compute = proofs_gpu::sxt_compute_pedersen_commitments_with_generators(
            sxt_compressed_ristretto,
            sxt_descriptors.len() as u32,
            sxt_descriptors.as_ptr(),
            sxt_ristretto_generators,
        );

        if ret_compute != 0 {
            panic!("Error during commitments computation with generators");
        }
    }
}

#[doc = include_str!("../../docs/commitments/update_commitments.md")]
///
/// # Example - Update commitments with dense, sparse, and dalek scalars
//
/// ```no_run
#[doc = include_str!("../../examples/simple_update_commitment.rs")]
/// ```
pub fn update_commitment<T: Descriptor>(
    commitment: &mut CompressedRistretto,
    offset_generators: u64,
    data: T,
) {
    let mut partial_commitment = [CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    // When the data is a sparse sequence,
    // we don't use the offset_generators,
    // because each data element is already
    // tied with its own row
    if data.is_sparse() {
        compute_commitments(&mut partial_commitment, &[data]);
    } else {
        // Otherwise, we fetch the generators from our proofs_gpu sys crate
        // and then we use them to compute the partial commitment out of the given data
        let mut generators = vec![RistrettoPoint::from_uniform_bytes(&[0_u8; 64]); data.len()];

        get_generators(&mut generators, offset_generators);

        compute_commitments_with_generators(&mut partial_commitment, &[data], &generators);
    }

    // using the A = `partial_commitment` and the B = `commitment`
    // given by the user, we compute a new commitment as B = A + B,
    // and then we write the result back to the `commitment` variable
    let c_a = match (*commitment).decompress() {
        Some(pt) => pt,
        None => panic!("invalid ristretto point decompression on update_commitment"),
    };

    let c_b = match partial_commitment[0].decompress() {
        Some(pt) => pt,
        None => panic!("invalid ristretto point decompression on update_commitment"),
    };

    (*commitment) = (c_a + c_b).compress();
}
