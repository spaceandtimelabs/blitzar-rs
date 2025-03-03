use serde::{Deserialize, Serialize};
use crate::proof::field::FieldId;

/// SumcheckProof construct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SumcheckProof {
    // pub(crate) l_vector: Vec<CompressedRistretto>,
    // pub(crate) r_vector: Vec<CompressedRistretto>,
    // pub(crate) ap_value: Scalar,
}
