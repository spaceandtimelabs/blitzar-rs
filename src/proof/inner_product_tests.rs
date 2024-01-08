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
use super::*;

use crate::compute::get_curve25519_generators;
use core::{mem, slice};
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use merlin::Transcript;
use rand_core::SeedableRng;

fn as_byte_slice<T>(point: &T) -> &[u8] {
    let len = mem::size_of::<T>();
    unsafe { slice::from_raw_parts(point as *const T as *const u8, len) }
}

fn test_prove_and_verify_with_given_n_and_generators_offset(n: u64, generators_offset: u64) {
    assert!(n > 0);

    let mut rng = rand::rngs::StdRng::seed_from_u64(n);

    // a and b are the vectors for which we want to prove c = <a,b>
    let a: Vec<_> = (0..n).map(|_| Scalar::random(&mut rng)).collect();
    let b: Vec<_> = (0..n).map(|_| Scalar::random(&mut rng)).collect();
    let g = {
        let mut temp_g = vec![RistrettoPoint::default(); n as usize];
        get_curve25519_generators(&mut temp_g, generators_offset);
        temp_g
    };

    let mut transcript = Transcript::new(b"innerproducttest");
    let proof = InnerProductProof::create(&mut transcript, &a, &b, generators_offset);
    let product = a.iter().zip(&b).map(|(a_i, b_i)| a_i * b_i).sum::<Scalar>();
    let a_commit = a
        .iter()
        .zip(&g)
        .map(|(a_i, g_i)| a_i * g_i)
        .sum::<RistrettoPoint>();

    // We can verify a proof using a valid input data
    {
        let mut transcript = Transcript::new(b"innerproducttest");
        assert!(proof
            .verify(&mut transcript, &a_commit, &product, &b, generators_offset)
            .is_ok());
    }

    // We cannot verify a proof using an invalid transcript
    {
        // we only use the transcript with arrays containing at least one element
        if n > 1 {
            let mut transcript = Transcript::new(b"invalid");
            assert!(proof
                .verify(&mut transcript, &a_commit, &product, &b, generators_offset)
                .is_err());
        }
    }

    // We cannot verify a proof using an invalid a_commit
    {
        let mut transcript = Transcript::new(b"innerproducttest");
        let a_commit_p = Scalar::from(123_u64) * g[0];
        assert!(proof
            .verify(
                &mut transcript,
                &a_commit_p,
                &product,
                &b,
                generators_offset
            )
            .is_err());
    }

    // We cannot verify a proof using an invalid product
    {
        let mut transcript = Transcript::new(b"innerproducttest");
        let product_p = product + Scalar::from(123_u64);
        assert!(proof
            .verify(
                &mut transcript,
                &a_commit,
                &product_p,
                &b,
                generators_offset
            )
            .is_err());
    }

    // We cannot verify a proof using an invalid b
    {
        let mut transcript = Transcript::new(b"innerproducttest");
        assert!(proof
            .verify(&mut transcript, &a_commit, &product, &a, generators_offset)
            .is_err());
    }

    // We can verify the transcript compatibility
    {
        let mut transcript = Transcript::new(b"innerproducttest");
        assert!(proof
            .verify(&mut transcript, &a_commit, &product, &b, generators_offset)
            .is_ok());

        // Initialize transcript
        let mut expected_transcript = Transcript::new(b"innerproducttest");
        expected_transcript.append_message(b"domain-sep", b"inner product proof v1");
        expected_transcript.append_u64(b"n", n);

        let num_rounds = n.next_power_of_two().trailing_zeros() as usize;
        for i in 0..num_rounds {
            expected_transcript.append_message(b"L", as_byte_slice(&proof.l_vector[i]));
            expected_transcript.append_message(b"R", as_byte_slice(&proof.r_vector[i]));
            let mut buf = [0u8; 32];
            expected_transcript.challenge_bytes(b"x", &mut buf);
        }

        // We verify that both transcripts produce the same challenge byte output
        for _i in 0..16 {
            let mut buf = [0u8; 128];
            let mut expected_buf = [0u8; 128];
            transcript.challenge_bytes(b"test", &mut buf);
            expected_transcript.challenge_bytes(b"test", &mut expected_buf);

            assert_eq!(buf, expected_buf);
        }

        // We verify that transcripts will produce different challenge byte output in case they differ
        transcript.append_message(b"tampering with transcript", b"should fail");

        let mut buf = [0u8; 128];
        let mut expected_buf = [0u8; 128];
        transcript.challenge_bytes(b"test", &mut buf);
        expected_transcript.challenge_bytes(b"test", &mut expected_buf);

        assert_ne!(buf, expected_buf);
    }

    // we cannot verify a proof using tampered l_vector length
    {
        // we only use the l_vector with arrays containing at least one element
        if n > 1 {
            let mut transcript = Transcript::new(b"innerproducttest");
            let mut tampered_proof = proof;
            tampered_proof.l_vector = Vec::new();

            assert!(tampered_proof
                .verify(&mut transcript, &a_commit, &product, &b, generators_offset)
                .is_err());
        }
    }
}

#[test]
fn test_prove_and_verify_with_a_single_element() {
    // zero generators offset case
    test_prove_and_verify_with_given_n_and_generators_offset(1, 0);

    // non-zero generators offset case
    test_prove_and_verify_with_given_n_and_generators_offset(1, 1);
}

#[test]
fn test_prove_and_verify_with_two_elements() {
    // zero generators offset case
    test_prove_and_verify_with_given_n_and_generators_offset(2, 0);

    // non-zero generators offset case
    test_prove_and_verify_with_given_n_and_generators_offset(2, 3);
}

#[test]
fn test_prove_and_verify_random_proofs_of_varying_size() {
    for i in 3_u64..16_u64 {
        // zero generators offset case
        test_prove_and_verify_with_given_n_and_generators_offset(i, 0);

        // non-zero generators offset case
        test_prove_and_verify_with_given_n_and_generators_offset(i, i);
    }
}
