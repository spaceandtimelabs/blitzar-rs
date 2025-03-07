use super::*;
use crate::proof::SumcheckTranscript;
use ark_grumpkin::Fq;
use ark_ff::Field;
use merlin::Transcript;

struct TestTranscript {
    base: Transcript,
}

impl TestTranscript {
    pub fn new() -> Self {
        Self {
            base: Transcript::new(b"abc123"),
        }
    }
}

impl SumcheckTranscript<Fq> for TestTranscript {
    fn init(&mut self, num_variables: usize, round_degree: usize) {}

    fn round_challenge(&mut self, polynomial: &[Fq]) -> Fq {
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                polynomial.as_ptr() as *const u8,
                polynomial.len() * std::mem::size_of::<Fq>(),
            )
        };
        self.base.append_message(b"p", bytes);
        let mut challenge : [u8; 32] = [0; 32];
        self.base.challenge_bytes(b"r", &mut challenge);
        Fq::from_random_bytes(&challenge).unwrap()
    }
}

#[test]
fn we_can_prove_sumcheck_with_an_mle_with_a_single_element() {
    let mles = vec![
        Fq::from(8),
        Fq::from(3),
    ];
    let product_table = vec![
        (Fq::from(1), 1),
    ];
    let product_terms = vec![
        0,
    ];
    let mut transcript = TestTranscript::new();
    let proof = SumcheckProof::new(
        &mut transcript,
        &mles,
        &product_table,
        &product_terms,
        2
    );
    assert_eq!(proof.round_polynomials[0], mles[0]);
}
