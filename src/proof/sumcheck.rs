use serde::{Deserialize, Serialize};
use crate::proof::field::FieldId;
use crate::proof::sumcheck_transcript::SumcheckTranscript;

/// SumcheckProof construct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SumcheckProof<T: FieldId> {
    evaluation_point: Vec<T>,
    round_polynomials: Vec<T>,
}

impl<T: FieldId + Default + Clone> SumcheckProof<T> {
    /// TODO: doc me
    pub fn new(
        transcript: &mut dyn SumcheckTranscript<T>,
        mles: & [T],
        product_table: &[(T, u32)],
        product_terms: &[u32],
        n: u32
    ) -> Self {
        let num_rounds = n.next_power_of_two().trailing_zeros() as usize;
        let mut evaluation_point = vec![T::default(); num_rounds];
        let round_degree = product_table.iter().map(|entry| entry.1).max().unwrap() as usize;
        let round_len = round_degree + 1;
        let mut round_polynomials = vec![T::default(); round_len * num_rounds];

        transcript.init(num_rounds, round_degree);

        Self{
            evaluation_point: evaluation_point,
            round_polynomials: round_polynomials,
        }
    }
}
