use serde::{Deserialize, Serialize};
use crate::proof::field::FieldId;
use crate::proof::sumcheck_transcript::SumcheckTranscript;
use std::os::raw::c_void;

/// SumcheckProof construct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SumcheckProof<T: FieldId> {
    evaluation_point: Vec<T>,
    round_polynomials: Vec<T>,
}

impl<T: FieldId + Default + Clone> SumcheckProof<T> {
    /// TODO: doc me
    pub fn new<Transcript: SumcheckTranscript<T>>(
        transcript: &mut Transcript,
        mles: & [T],
        product_table: &[(T, u32)],
        product_terms: &[u32],
        n: u32
    ) -> Self {
        let num_mles = mles.len() / n as usize;
        assert_eq!(mles.len(), num_mles * n as usize);
        let num_rounds = n.next_power_of_two().trailing_zeros() as usize;
        let mut evaluation_point = vec![T::default(); num_rounds];
        let round_degree = product_table.iter().map(|entry| entry.1).max().unwrap() as usize;
        let round_len = round_degree + 1;
        let mut round_polynomials = vec![T::default(); round_len * num_rounds];

        transcript.init(num_rounds, round_degree);

        let fptr : fn(*mut T, *mut c_void, *const T, u32) = round_challenge::<T, Transcript>;

        let mut descriptor: blitzar_sys::sumcheck_descriptor = blitzar_sys::sumcheck_descriptor{
            mles: mles.as_ptr() as *const c_void,
            product_table: product_table.as_ptr() as *const c_void,
            product_terms: product_terms.as_ptr(),
            n: n,
            num_mles: num_mles as u32,
            num_products: product_table.len() as u32,
            num_product_terms: product_terms.len() as u32,
            round_degree: round_degree as u32,
        };
        // descriptor.mles = mles.as_ptr() as *c_void;
        // unsafe {
        //     blitzar_sys::
        // }
            // let handle = blitzar_sys::sxt_multiexp_handle_new(
            //     T::CURVE_ID,
            //     generators.as_ptr() as *const std::ffi::c_void,
            //     generators.len() as u32,
            // );
        // fn f(ctx: *mut c_void, polynomial: *const T) {
        // }
        // void (*)(T* r, void* context, const T* polynomial, unsigned polynomial_len);

        Self{
            evaluation_point: evaluation_point,
            round_polynomials: round_polynomials,
        }
    }
}

fn round_challenge<T, Transcript: SumcheckTranscript<T>>(
    r: *mut T,
    ctx: *mut c_void, polynomial: *const T, len: u32) {
    unsafe {
    let mut transcript = &mut *(ctx as *mut Transcript);
    let p = std::slice::from_raw_parts(polynomial, len as usize);
    *r.as_mut().unwrap() = transcript.round_challenge(p);
    }
}
