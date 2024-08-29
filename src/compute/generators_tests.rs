// Copyright 2024-present Space and Time Labs, Inc.
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

use super::*;
use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
    traits::Identity,
};

#[test]
fn get_generators_is_the_same_used_in_commitment_computation() {
    // generate input table
    let offset_generators = 0_u64;
    let data: Vec<u16> = vec![2, 3, 1, 5, 4, 7, 6, 8, 9, 10];
    let mut commitments = vec![CompressedRistretto::default(); 1];
    let mut generators = vec![RistrettoPoint::from_uniform_bytes(&[0_u8; 64]); data.len()];

    // convert the generator points to compressed ristretto
    get_curve25519_generators(&mut generators, 0_u64);

    compute_curve25519_commitments(&mut commitments, &[(&data).into()], offset_generators);

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
    assert_ne!(CompressedRistretto::default(), commitments[0]);
}

#[test]
fn get_generators_with_offset_is_the_same_used_in_commitment_computation() {
    // generate input table
    let data: Vec<u32> = vec![0, 0, 0, 0, 4, 7, 6, 8, 9, 10, 0, 0, 0];
    let offset_generators: usize = 4;
    let generators_len = data.len() - offset_generators;
    let mut generators = vec![RistrettoPoint::from_uniform_bytes(&[0_u8; 64]); generators_len];
    let mut commitments = vec![CompressedRistretto::default(); 1];

    get_curve25519_generators(&mut generators, offset_generators as u64);

    compute_curve25519_commitments(&mut commitments, &[(&data).into()], 0_u64);

    let expected_commit = data[offset_generators..]
        .iter()
        .zip(generators.iter())
        .map(|(x, y)| Scalar::from(*x) * y)
        .sum::<RistrettoPoint>()
        .compress();

    assert_eq!(commitments[0], expected_commit);
    assert_ne!(CompressedRistretto::default(), commitments[0]);
}

#[test]
fn get_one_commit_is_valid() {
    let generators_len = 3;
    let mut generators = vec![RistrettoPoint::from_uniform_bytes(&[0_u8; 64]); generators_len];

    get_curve25519_generators(&mut generators, 0);

    assert_eq!(get_one_curve25519_commit(0), RistrettoPoint::identity());
    assert_eq!(get_one_curve25519_commit(1), generators[0]);
    assert_eq!(get_one_curve25519_commit(2), generators[0] + generators[1]);
}
