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
use crate::{
    compute::conversion::{
        convert_ark_affine_to_halo2_projective_bn254_g1, convert_halo2_to_ark_bn254_g1_affine,
        convert_halo2_to_ark_bn254_g1_projective,
    },
    sequence::Sequence,
};
use ark_bls12_381::G1Affine;
use ark_bn254::G1Affine as Bn254G1Affine;
use ark_ec::CurveGroup;
use ark_grumpkin::Affine as GrumpkinAffine;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use halo2curves::bn256::{G1Affine as Halo2Bn256G1Affine, G1 as Halo2Bn256G1Projective};

#[doc = include_str!("../../docs/commitments/compute_curve25519_commitments.md")]
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
pub fn compute_curve25519_commitments(
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

#[doc = include_str!("../../docs/commitments/compute_curve25519_commitments_with_generators.md")]
///
/// # Example 1 - Pass generators to Commitment Computation
///```no_run
#[doc = include_str!("../../examples/pass_curve25519_generators_to_commitment.rs")]
///```
///
/// # Example 2 - Compute Commitments with Dalek Scalars and User Generators
///```no_run
#[doc = include_str!("../../examples/pass_generators_and_scalars_to_commitment.rs")]
///```
pub fn compute_curve25519_commitments_with_generators(
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

#[doc = include_str!("../../docs/commitments/compute_bls12_381_g1_commitments_with_generators.md")]
///
/// # Example - Pass generators to Commitment Computation
///```no_run
#[doc = include_str!("../../examples/pass_bls12_381_g1_generators_to_commitment.rs")]
///```
pub fn compute_bls12_381_g1_commitments_with_generators(
    commitments: &mut [[u8; 48]],
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

#[doc = include_str!("../../docs/commitments/compute_bn254_g1_commitments_with_generators.md")]
///
/// # Example - Pass generators to Commitment Computation
///```no_run
#[doc = include_str!("../../examples/pass_bn254_g1_generators_to_commitment.rs")]
///```
pub fn compute_bn254_g1_uncompressed_commitments_with_generators(
    commitments: &mut [Bn254G1Affine],
    data: &[Sequence],
    generators: &[Bn254G1Affine],
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

    let sxt_bn254_g1_generators = generators.as_ptr() as *const blitzar_sys::sxt_bn254_g1;

    let sxt_bn254_g1_uncompressed = commitments.as_mut_ptr() as *mut blitzar_sys::sxt_bn254_g1;

    unsafe {
        blitzar_sys::sxt_bn254_g1_uncompressed_compute_pedersen_commitments_with_generators(
            sxt_bn254_g1_uncompressed,
            sxt_descriptors.len() as u32,
            sxt_descriptors.as_ptr(),
            sxt_bn254_g1_generators,
        );
    }
}

#[doc = include_str!("../../docs/commitments/compute_halo2curves_bn256_g1_commitments_with_generators.md")]
pub fn compute_bn254_g1_uncompressed_commitments_with_halo2_generators(
    commitments: &mut [Halo2Bn256G1Projective],
    data: &[Sequence],
    generators: &[Halo2Bn256G1Affine],
) {
    // Convert the Halo2 generators to Arkworks generators
    let ark_generators = generators
        .iter()
        .map(convert_halo2_to_ark_bn254_g1_affine)
        .collect::<Vec<Bn254G1Affine>>();

    // Convert the Halo2 commitments to Arkworks commitments
    let mut ark_commitments = commitments
        .iter()
        .map(convert_halo2_to_ark_bn254_g1_projective)
        .map(|proj| proj.into_affine())
        .collect::<Vec<Bn254G1Affine>>();

    // Compute commitments
    compute_bn254_g1_uncompressed_commitments_with_generators(
        &mut ark_commitments,
        data,
        &ark_generators,
    );

    // Convert the Arkworks commitments back to Halo2 commitments
    commitments
        .iter_mut()
        .zip(
            ark_commitments
                .iter()
                .map(convert_ark_affine_to_halo2_projective_bn254_g1),
        )
        .for_each(|(c_a, c_b)| {
            *c_a = c_b;
        });
}

#[doc = include_str!("../../docs/commitments/update_curve25519_commitments.md")]
///
/// # Example - Update Commitments with Dense and Dalek Scalars
/// ```no_run
#[doc = include_str!("../../examples/simple_update_commitment.rs")]
/// ```
pub fn update_curve25519_commitments(
    commitments: &mut [CompressedRistretto],
    data: &[Sequence],
    offset_generators: u64,
) {
    assert_eq!(data.len(), commitments.len());
    let num_columns: usize = commitments.len();

    let mut partial_commitments = vec![CompressedRistretto::default(); num_columns];

    compute_curve25519_commitments(&mut partial_commitments, data, offset_generators);

    commitments
        .iter_mut()
        .zip(partial_commitments)
        .for_each(|(c_a, c_b)| {
            *c_a = (c_a.decompress().unwrap_or_else(|| {
                panic!("invalid ristretto point decompression on update_curve25519_commitments")
            }) + c_b.decompress().unwrap_or_else(|| {
                panic!("invalid ristretto point decompression on update_curve25519_commitments")
            }))
            .compress()
        });
}

#[doc = include_str!("../../docs/commitments/compute_grumpkin_commitments_with_generators.md")]
///
/// # Example - Pass generators to Commitment Computation
///```no_run
#[doc = include_str!("../../examples/pass_grumpkin_generators_to_commitment.rs")]
///```
pub fn compute_grumpkin_uncompressed_commitments_with_generators(
    commitments: &mut [GrumpkinAffine],
    data: &[Sequence],
    generators: &[GrumpkinAffine],
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

    let sxt_grumpkin_generators = generators.as_ptr() as *const blitzar_sys::sxt_grumpkin;

    let sxt_grumpkin_uncompressed = commitments.as_mut_ptr() as *mut blitzar_sys::sxt_grumpkin;

    unsafe {
        blitzar_sys::sxt_grumpkin_uncompressed_compute_pedersen_commitments_with_generators(
            sxt_grumpkin_uncompressed,
            sxt_descriptors.len() as u32,
            sxt_descriptors.as_ptr(),
            sxt_grumpkin_generators,
        );
    }
}
