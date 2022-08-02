// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

use super::*;
use crate::sequences::{DenseSequence, Sequence};
use byte_slice_cast::AsByteSlice;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;

#[test]
fn get_generators_is_the_same_used_in_commitment_computation() {
    // generate input table
    let data: Vec<u16> = vec![2, 3, 1, 5, 4, 7, 6, 8, 9, 10];

    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];
    let mut generators = vec![RistrettoPoint::from_uniform_bytes(&[0_u8; 64]); data.len()];

    // convert the generator points to compressed ristretto
    get_generators(&mut generators, 0_u64);

    compute_commitments(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        })],
    );

    let mut expected_commit = RistrettoPoint::from_uniform_bytes(&[0_u8; 64]);

    for i in 0..generators.len() {
        let mut scalar_bytes: [u8; 32] = [0; 32];
        scalar_bytes[0] = data[i] as u8;

        // Construct a Scalar by reducing a 256-bit little-endian integer mocdulo the group order ℓ.
        let ristretto_sc = Scalar::from_bytes_mod_order(scalar_bytes);

        let g_i = generators[i];

        expected_commit += ristretto_sc * g_i;
    }

    assert_eq!(commitments[0], expected_commit.compress());
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[0]);
}

#[test]
fn get_generators_with_offset_is_the_same_used_in_commitment_computation() {
    // generate input table
    let data: Vec<u32> = vec![0, 0, 0, 0, 4, 7, 6, 8, 9, 10, 0, 0, 0];

    let offset_generators: usize = 4;
    let generators_len = data.len() - offset_generators - 3;
    let mut generators = vec![RistrettoPoint::from_uniform_bytes(&[0_u8; 64]); generators_len];
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    // convert the generator points to compressed ristretto

    get_generators(&mut generators, offset_generators as u64);

    compute_commitments(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        })],
    );

    let mut expected_commit = RistrettoPoint::from_uniform_bytes(&[0_u8; 64]);

    for i in 0..generators.len() {
        let mut scalar_bytes: [u8; 32] = [0; 32];
        scalar_bytes[0] = data[i + offset_generators] as u8;

        // Construct a Scalar by reducing a 256-bit little-endian integer modulo the group order ℓ.
        let ristretto_sc = Scalar::from_bytes_mod_order(scalar_bytes);

        let g_i = generators[i];

        expected_commit += ristretto_sc * g_i;
    }

    assert_eq!(commitments[0], expected_commit.compress());
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[0]);
}
