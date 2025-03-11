use crate::{
    compute::init_backend,
    proof::{field::FieldId, sumcheck_transcript::SumcheckTranscript},
};
use serde::{Deserialize, Serialize};
use std::{cmp::max, os::raw::c_void};

/// Proof in sumcheck protocol up to evaluation at a random point
///
/// See https://people.cs.georgetown.edu/jthaler/sumcheck.pdf
/// for background.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SumcheckProof<T: FieldId> {
    /// Random elements chosen for each round of sumcheck
    pub evaluation_point: Vec<T>,

    /// Univariate polynomials produced for each round of sumcheck.
    ///
    /// If d denotes the degree of the round polynomial, then
    /// the polynomial for the ith round is given by
    ///     round_polynomial[i * (d + 1)] +
    ///     round_polynomial[i * (d + 1) + 1] * X +
    ///     round_polynomial[i * (d + 1) + 2] * X^2 +
    ///     ...
    ///     round_polynomial[i * (d + 1) + d] * X^d +
    pub round_polynomials: Vec<T>,
}

impl<T: FieldId + Default + Clone> SumcheckProof<T> {
    /// Construct a proof for sumcheck using a polynomial of the form
    ///
    ///    sum_i^num_products {mult_i x prod_j^product_length_i f_j(X1, ..., Xr)}
    ///
    ///  where f_j(X1, ..., Xr) denotes a multilinear extension of r variables.
    ///
    /// transcript provides the random challenge for each round of sumcheck
    ///
    /// mles describes an n by (num_mles) column major matrix
    /// reference by sumcheck
    ///
    /// product_table describes each product of the sumcheck polynomial
    /// with entries of the form
    ///    (multiplier, product_length)
    ///
    /// product_terms identifies which MLEs the sumcheck product terms reference.
    pub fn new<Transcript: SumcheckTranscript<T>>(
        transcript: &mut Transcript,
        mles: &[T],
        product_table: &[(T, u32)],
        product_terms: &[u32],
        n: u32,
    ) -> Self {
        init_backend();
        assert!(n > 0);
        assert!(!product_table.is_empty());
        let num_mles = mles.len() / n as usize;
        assert_eq!(mles.len(), num_mles * n as usize);
        for mle_index in product_terms {
            assert!((*mle_index as usize) < num_mles);
        }
        let num_product_terms: u32 = product_table.iter().map(|entry| entry.1).sum();
        assert!(product_terms.len() == num_product_terms as usize);

        let num_rounds = max(n.next_power_of_two().trailing_zeros(), 1) as usize;
        let evaluation_point = vec![T::default(); num_rounds];
        let round_degree = product_table.iter().map(|entry| entry.1).max().unwrap() as usize;
        let round_len = round_degree + 1;
        let round_polynomials = vec![T::default(); round_len * num_rounds];

        transcript.init(num_rounds, round_degree);

        let fptr: extern "C" fn(*mut T, *mut c_void, *const T, u32) =
            round_challenge::<T, Transcript>;

        let descriptor: blitzar_sys::sumcheck_descriptor = blitzar_sys::sumcheck_descriptor {
            mles: mles.as_ptr() as *const c_void,
            product_table: product_table.as_ptr() as *const c_void,
            product_terms: product_terms.as_ptr(),
            n,
            num_mles: num_mles as u32,
            num_products: product_table.len() as u32,
            num_product_terms: product_terms.len() as u32,
            round_degree: round_degree as u32,
        };
        unsafe {
            blitzar_sys::sxt_prove_sumcheck(
                round_polynomials.as_ptr() as *mut c_void,
                evaluation_point.as_ptr() as *mut c_void,
                T::FIELD_ID,
                &descriptor,
                fptr as *mut c_void,
                std::ptr::from_ref(transcript) as *mut c_void,
            );
        }
        Self {
            evaluation_point,
            round_polynomials,
        }
    }
}

extern "C" fn round_challenge<T, Transcript: SumcheckTranscript<T>>(
    r: *mut T,
    ctx: *mut c_void,
    polynomial: *const T,
    len: u32,
) {
    unsafe {
        let transcript = &mut *(ctx as *mut Transcript);
        let p = std::slice::from_raw_parts(polynomial, len as usize);
        *r.as_mut().unwrap() = transcript.round_challenge(p);
    }
}
