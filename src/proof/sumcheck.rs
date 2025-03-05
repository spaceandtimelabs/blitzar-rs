use crate::proof::field::FieldId;
use crate::proof::sumcheck_transcript::SumcheckTranscript;
use serde::{Deserialize, Serialize};
use std::os::raw::c_void;

/// SumcheckProof construct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SumcheckProof<T: FieldId> {
    /// TODO: doc me
    pub evaluation_point: Vec<T>,

    /// TODO: doc me
    pub round_polynomials: Vec<T>,
}

impl<T: FieldId + Default + Clone> SumcheckProof<T> {
    /// TODO: doc me
    pub fn new<Transcript: SumcheckTranscript<T>>(
        transcript: &mut Transcript,
        mles: &[T],
        product_table: &[(T, u32)],
        product_terms: &[u32],
        n: u32,
    ) -> Self {
        let num_mles = mles.len() / n as usize;
        assert_eq!(mles.len(), num_mles * n as usize);
        let num_rounds = n.next_power_of_two().trailing_zeros() as usize;
        let evaluation_point = vec![T::default(); num_rounds];
        let round_degree = product_table.iter().map(|entry| entry.1).max().unwrap() as usize;
        let round_len = round_degree + 1;
        let round_polynomials = vec![T::default(); round_len * num_rounds];

        transcript.init(num_rounds, round_degree);

        let fptr: fn(*mut T, *mut c_void, *const T, u32) = round_challenge::<T, Transcript>;

        let descriptor: blitzar_sys::sumcheck_descriptor = blitzar_sys::sumcheck_descriptor {
            mles: mles.as_ptr() as *const c_void,
            product_table: product_table.as_ptr() as *const c_void,
            product_terms: product_terms.as_ptr(),
            n: n,
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
            evaluation_point: evaluation_point,
            round_polynomials: round_polynomials,
        }
    }
}

fn round_challenge<T, Transcript: SumcheckTranscript<T>>(
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
