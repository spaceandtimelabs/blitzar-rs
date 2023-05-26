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
use super::error::ProofError;
use crate::compute::init_backend;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use merlin::Transcript;
use serde::{Deserialize, Serialize};

/// InnerProductProof construct.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InnerProductProof {
    pub(crate) l_vector: Vec<CompressedRistretto>,
    pub(crate) r_vector: Vec<CompressedRistretto>,
    pub(crate) ap_value: Scalar,
}

impl InnerProductProof {
    /// Creates an inner product proof
    ///
    /// The proof is created with respect to the base G, provided by:
    ///
    /// ```text
    /// let np = 1ull << ceil(log2(n));
    /// let G = vec![RISTRETTO_BASEPOINT_POINT; np + 1];
    /// crate::compute::get_generators(G, generators_offset)`.
    /// ```
    ///
    /// The `verifier` transcript is passed in as a parameter so that the
    /// challenges depend on the *entire* transcript (including parent
    /// protocols).
    ///
    /// Note that we don't have any restriction to the `n` value, other than
    /// it has to be non-zero.
    ///
    /// # Algorithm description
    ///
    /// Initially, we compute G and Q = G[np], where np = 1ull << ceil(log2(n))
    /// and G is zero-indexed.
    ///
    /// The protocol consists of k = ceil(lg_2(n)) rounds, indexed by j = k - 1 , ... , 0.
    ///
    /// In the j-th round, the prover computes:
    ///
    /// ```text
    /// a_lo = {a[0], a[1], ..., a[n / 2 - 1]}
    /// a_hi = {a[n/2], a[n/2 + 1], ..., a[n - 1]}
    /// b_lo = {b[0], b[1], ..., b[n / 2 - 1]}
    /// b_hi = {b[n/2], b[n/2 + 1], ..., b[n - 1]}
    /// G_lo = {G[0], G[1], ..., G[n / 2 - 1]}
    /// G_hi = {G[n/2], G[n/2 + 1], ..., G[n-1]}
    ///
    /// l_vector[j] = <a_lo, G_hi> + <a_lo, b_hi> * Q
    /// r_vector[j] = <a_hi, G_lo> + <a_hi, b_lo> * Q
    /// ```
    ///
    /// Note that if the `a` or `b` length is not a power of 2,
    /// then `a` or `b` is padded with zeros until it has a power of 2.
    /// G always has a power of 2 given how it is constructed.
    ///
    /// Then the prover sends l_vector[j] and r_vector[j] to the verifier,
    /// and the verifier responds with a
    /// challenge value u[j] <- Z_p (finite field of order p),
    /// which is non-interactively simulated by
    /// the input strobe-based transcript:
    ///
    /// ```text
    /// transcript.append("L", l_vector[j]);
    /// transcript.append("R", r_vector[j]);
    ///
    /// u[j] = transcript.challenge_value("x");
    /// ```
    ///
    /// Then the prover uses u[j] to compute
    ///
    /// ```text
    /// a = a_lo * u[j] + (u[j]^-1) * a_hi;
    /// b = b_lo * (u[j]^-1) + u[j] * b_hi;
    /// ```
    ///
    /// Then, the prover and verifier both compute
    ///
    /// ```text
    /// G = G_lo * (u[j]^-1) + u[j] * G_hi
    ///
    /// n = n / 2;
    /// ```
    ///
    /// and use these vectors (all of length 2^j) for the next round.
    ///
    /// After the last (j = 0) round, the prover sends ap_value = a[0] to the verifier.
    ///
    /// # Arguments:
    ///
    /// - transcript (in/out): a single strobe-based transcript
    /// - a (in): array with non-zero length n
    /// - b (in): array with non-zero length n
    /// - generators_offset (in): offset used to fetch the bases
    #[tracing::instrument(name = "proof.inner_product.create", level = "info", skip_all)]
    pub fn create(
        transcript: &mut Transcript,
        a: &[Scalar],
        b: &[Scalar],
        generators_offset: u64,
    ) -> InnerProductProof {
        init_backend();

        let n: u64 = a.len() as u64;

        assert!(n > 0);
        assert!(n == b.len() as u64);

        let ceil_lg2_n = n.next_power_of_two().trailing_zeros() as usize;
        let mut ap_value = Scalar::default();
        let mut l_vector: Vec<CompressedRistretto> =
            vec![CompressedRistretto::default(); ceil_lg2_n];
        let mut r_vector: Vec<CompressedRistretto> =
            vec![CompressedRistretto::default(); ceil_lg2_n];

        unsafe {
            let a = a.as_ptr() as *const blitzar_sys::sxt_scalar;
            let b = b.as_ptr() as *const blitzar_sys::sxt_scalar;
            let transcript = transcript as *mut Transcript as *mut blitzar_sys::sxt_transcript;

            let ap_value = &mut ap_value as *mut Scalar as *mut blitzar_sys::sxt_scalar;
            let l_vector = l_vector.as_mut_ptr() as *mut blitzar_sys::sxt_compressed_ristretto;
            let r_vector = r_vector.as_mut_ptr() as *mut blitzar_sys::sxt_compressed_ristretto;

            blitzar_sys::sxt_prove_inner_product(
                l_vector,
                r_vector,
                ap_value,
                transcript,
                n,
                generators_offset,
                a,
                b,
            );
        }

        InnerProductProof {
            l_vector,
            r_vector,
            ap_value,
        }
    }

    /// Verifies an inner product proof
    ///
    /// The proof is verified with respect to the base G, provided by:
    ///
    /// ```text
    /// let np = 1ull << ceil(log2(n));
    /// let G = vec![RISTRETTO_BASEPOINT_POINT; np + 1];
    /// crate::compute::get_generators(G, generators_offset)`.
    /// ```
    ///
    /// Note that we don't have any restriction to the `n` value, other than
    /// it has to be non-zero.
    ///
    /// # Arguments:
    ///
    /// - transcript (in/out): a single strobe-based transcript
    /// - a_commit (in): a single ristretto point,
    ///                  represented by <a, G> (the inner product of the two vectors)
    /// - product (in): a single scalar, represented by <a, b>,
    ///                 the inner product of the two vectors `a` and `b` used by
    ///                 `InnerProductProof::create(...)`
    /// - b (in): array with non-zero length n, the same one used by `InnerProductProof::create(...)`
    /// - generators_offset (in): offset used to fetch the bases
    #[tracing::instrument(name = "proof.inner_product.verify", level = "info", skip_all)]
    pub fn verify(
        &self,
        transcript: &mut Transcript,
        a_commit: &RistrettoPoint,
        product: &Scalar,
        b: &[Scalar],
        generators_offset: u64,
    ) -> Result<(), ProofError> {
        init_backend();

        let n = b.len();
        assert!(n > 0);

        let ceil_lg2_n = n.next_power_of_two().trailing_zeros() as usize;

        if ceil_lg2_n != self.l_vector.len() || ceil_lg2_n != self.r_vector.len() {
            return Err(ProofError::VerificationError);
        }

        let transcript = transcript as *mut Transcript as *mut blitzar_sys::sxt_transcript;
        let b = b.as_ptr() as *const blitzar_sys::sxt_scalar;
        let product = product as *const Scalar as *const blitzar_sys::sxt_scalar;
        let a_commit = a_commit as *const RistrettoPoint as *const blitzar_sys::sxt_ristretto;
        let ap_value = &self.ap_value as *const Scalar as *const blitzar_sys::sxt_scalar;
        let l_vector = self.l_vector.as_ptr() as *const blitzar_sys::sxt_compressed_ristretto;
        let r_vector = self.r_vector.as_ptr() as *const blitzar_sys::sxt_compressed_ristretto;

        unsafe {
            let ret = blitzar_sys::sxt_verify_inner_product(
                transcript,
                n as u64,
                generators_offset,
                b,
                product,
                a_commit,
                l_vector,
                r_vector,
                ap_value,
            );

            if ret == 1 {
                return Ok(());
            }
        }

        Err(ProofError::VerificationError)
    }
}
