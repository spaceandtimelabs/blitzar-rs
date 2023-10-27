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
use crate::sequence::Sequence;
use ark_bls12_381::G1Affine;
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
///```
#[tracing::instrument(
    name = "compute.commitments.compute_commitments",
    level = "info",
    skip_all
)]
pub fn compute_commitments(
    commitments: &mut [CompressedRistretto],
    data: &[Sequence],
    offset_generators: u64,
) {
    init_backend();

    let sxt_descriptors: Vec<blitzar_sys::sxt_sequence_descriptor> =
        data.iter().map(Into::into).collect();

    let sxt_ristretto255_compressed =
        commitments.as_mut_ptr() as *mut blitzar_sys::sxt_ristretto255_compressed;

    unsafe {
        blitzar_sys::sxt_curve25519_compute_pedersen_commitments(
            sxt_ristretto255_compressed,
            sxt_descriptors.len() as u32,
            sxt_descriptors.as_ptr(),
            offset_generators,
        );
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
#[tracing::instrument(
    name = "compute.commitments.compute_commitments_with_generators",
    level = "info",
    skip_all
)]
pub fn compute_commitments_with_generators(
    commitments: &mut [CompressedRistretto],
    data: &[Sequence],
    generators: &[RistrettoPoint],
) {
    init_backend();

    let sxt_descriptors: Vec<blitzar_sys::sxt_sequence_descriptor> = data
        .iter()
        .map(|s| {
            assert!(
                s.len() <= generators.len(),
                "generators has a length smaller than the longest sequence in the input data"
            );
            s.into()
        })
        .collect();

    let sxt_ristretto_generators = generators.as_ptr() as *const blitzar_sys::sxt_ristretto255;

    let sxt_ristretto255_compressed =
        commitments.as_mut_ptr() as *mut blitzar_sys::sxt_ristretto255_compressed;

    unsafe {
        blitzar_sys::sxt_curve25519_compute_pedersen_commitments_with_generators(
            sxt_ristretto255_compressed,
            sxt_descriptors.len() as u32,
            sxt_descriptors.as_ptr(),
            sxt_ristretto_generators,
        );
    }
}

/// The documentation linked here is related to curve25519.
/// The doc tag does not seem to be working as expected. When the doc tag issue
/// is sorted out, documentation for the bls12-381 G1 commitment computation
/// will be created and linked below.
#[doc = include_str!("../../docs/commitments/compute_commitments_with_generators.md")]
///
///# Example 1 - Pass generators to Commitment Computation
///```no_run
#[doc = include_str!("../../examples/pass_generators_to_commitment.rs")]
///```
///
/// Example 2 - Compute Commitments with Scalars and User Generators
///```no_run
#[doc = include_str!("../../examples/pass_generators_and_scalars_to_commitment.rs")]
///```
pub fn compute_bls12_381_g1_commitments_with_generators(
    commitments: &mut [u8; 48],
    data: &[Sequence],
    generators: &[G1Affine],
) {
    init_backend();

    let sxt_descriptors: Vec<blitzar_sys::sxt_sequence_descriptor> = data
        .iter()
        .map(|s| {
            assert!(
                s.len() <= generators.len(),
                "generators has a length smaller than the longest sequence in the input data"
            );
            s.into()
        })
        .collect();

    let sxt_bls12_381_g1_generators = generators.as_ptr() as *const blitzar_sys::sxt_bls12_381_g1;

    let sxt_bls12_381_g1_compressed =
        commitments.as_mut_ptr() as *mut blitzar_sys::sxt_bls12_381_g1_compressed;

    unsafe {
        blitzar_sys::sxt_bls12_381_g1_compute_pedersen_commitments_with_generators(
            sxt_bls12_381_g1_compressed,
            sxt_descriptors.len() as u32,
            sxt_descriptors.as_ptr(),
            sxt_bls12_381_g1_generators,
        );
    }
}

#[doc = include_str!("../../docs/commitments/update_commitments.md")]
///
/// # Example - Update commitments with dense and dalek scalars
//
/// ```no_run
#[doc = include_str!("../../examples/simple_update_commitment.rs")]
/// ```
#[tracing::instrument(
    name = "compute.commitments.update_commitments",
    level = "info",
    skip_all
)]
pub fn update_commitments(
    commitments: &mut [CompressedRistretto],
    data: &[Sequence],
    offset_generators: u64,
) {
    assert_eq!(data.len(), commitments.len());
    let num_columns: usize = commitments.len();

    let mut partial_commitments = vec![CompressedRistretto::default(); num_columns];

    compute_commitments(&mut partial_commitments, data, offset_generators);

    commitments
        .iter_mut()
        .zip(partial_commitments)
        .for_each(|(c_a, c_b)| {
            *c_a = (c_a.decompress().unwrap_or_else(|| {
                panic!("invalid ristretto point decompression on update_commitments")
            }) + c_b.decompress().unwrap_or_else(|| {
                panic!("invalid ristretto point decompression on update_commitments")
            }))
            .compress()
        });
}
