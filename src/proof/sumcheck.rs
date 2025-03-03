use serde::{Deserialize, Serialize};
use crate::proof::field::FieldId;
use crate::proof::sumcheck_transcript::SumcheckTranscript;

/// SumcheckProof construct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SumcheckProof<T: FieldId> {
    evaluation_point: Vec<T>,
    round_polynomials: Vec<T>,
}

impl<T: FieldId> SumcheckProof<T> {
    /// TODO: doc me
    pub fn new(
    ) -> Self {
        Self{
            evaluation_point: Vec::new(),
            round_polynomials: Vec::new(),
        }
    }
}
