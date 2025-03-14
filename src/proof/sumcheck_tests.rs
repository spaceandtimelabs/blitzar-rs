use super::*;
use crate::proof::SumcheckTranscript;
use ark_grumpkin::Fq;
use ark_std::One;
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
    fn init(&mut self, _num_variables: usize, _round_degree: usize) {}

    fn round_challenge(&mut self, polynomial: &[Fq]) -> Fq {
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                polynomial.as_ptr() as *const u8,
                std::mem::size_of_val(polynomial),
            )
        };
        self.base.append_message(b"p", bytes);
        let mut challenge: [u8; 8] = [0; 8];
        self.base.challenge_bytes(b"r", &mut challenge);
        Fq::from(u64::from_be_bytes(challenge))
    }
}

#[test]
fn we_can_prove_sumcheck_with_an_mle_with_a_single_element() {
    let mles = [Fq::from(8)];
    let product_table = [(Fq::from(1), 1)];
    let product_terms = [0];
    let mut transcript = TestTranscript::new();
    let proof = SumcheckProof::new(&mut transcript, &mles, &product_table, &product_terms, 1);
    assert_eq!(proof.round_polynomials[0], mles[0]);
    assert_eq!(proof.round_polynomials[1], -mles[0]);
    let mut transcript = TestTranscript::new();
    assert_eq!(
        proof.evaluation_point[0],
        transcript.round_challenge(&proof.round_polynomials)
    );
}

#[test]
fn we_can_prove_sumcheck_with_an_mle_with_two_elements() {
    let mles = [Fq::from(8), Fq::from(10)];
    let product_table = [(Fq::from(1), 1)];
    let product_terms = [0];
    let mut transcript = TestTranscript::new();
    let proof = SumcheckProof::new(&mut transcript, &mles, &product_table, &product_terms, 2);
    assert_eq!(proof.round_polynomials[0], mles[0]);
    assert_eq!(proof.round_polynomials[1], mles[1] - mles[0]);
    let mut transcript = TestTranscript::new();
    assert_eq!(
        proof.evaluation_point[0],
        transcript.round_challenge(&proof.round_polynomials)
    );
}

#[test]
fn we_can_prove_sumcheck_with_multiple_mles() {
    let mles = [Fq::from(8), Fq::from(3)];
    let product_table = [(Fq::from(1), 1), (Fq::from(2), 1)];
    let product_terms = [0, 1];
    let mut transcript = TestTranscript::new();
    let proof = SumcheckProof::new(&mut transcript, &mles, &product_table, &product_terms, 1);
    assert_eq!(proof.round_polynomials[0], mles[0] + Fq::from(2) * mles[1]);
    assert_eq!(proof.round_polynomials[1], -mles[0] - Fq::from(2) * mles[1]);
    let mut transcript = TestTranscript::new();
    assert_eq!(
        proof.evaluation_point[0],
        transcript.round_challenge(&proof.round_polynomials)
    );
}

#[test]
fn we_can_prove_sumcheck_with_two_rounds() {
    let mles = [Fq::from(8), Fq::from(3), Fq::from(11), Fq::from(51)];
    let product_table = [(Fq::from(1), 1)];
    let product_terms = [0];
    let mut transcript = TestTranscript::new();
    let proof = SumcheckProof::new(&mut transcript, &mles, &product_table, &product_terms, 4);
    assert_eq!(proof.round_polynomials[0], mles[0] + mles[1]);
    assert_eq!(
        proof.round_polynomials[1],
        (mles[2] - mles[0]) + (mles[3] - mles[1])
    );
    let r = proof.evaluation_point[0];
    let mles = [
        mles[0] * (Fq::one() - r) + mles[2] * r,
        mles[1] * (Fq::one() - r) + mles[3] * r,
    ];
    assert_eq!(proof.round_polynomials[2], mles[0]);
    assert_eq!(proof.round_polynomials[3], mles[1] - mles[0]);

    let mut transcript = TestTranscript::new();
    assert_eq!(
        proof.evaluation_point[0],
        transcript.round_challenge(&proof.round_polynomials[..2])
    );
    assert_eq!(
        proof.evaluation_point[1],
        transcript.round_challenge(&proof.round_polynomials[2..])
    );
}

#[test]
fn we_can_prove_sumcheck_with_two_products() {
    let mles = [Fq::from(8), Fq::from(3), Fq::from(11), Fq::from(51)];
    let product_table = [(Fq::from(1), 2)];
    let product_terms = [0, 1];
    let mut transcript = TestTranscript::new();
    let proof = SumcheckProof::new(&mut transcript, &mles, &product_table, &product_terms, 2);
    assert_eq!(proof.round_polynomials[0], mles[0] * mles[2]);
    assert_eq!(
        proof.round_polynomials[1],
        (mles[1] - mles[0]) * mles[2] + (mles[3] - mles[2]) * mles[0]
    );
    assert_eq!(
        proof.round_polynomials[2],
        (mles[1] - mles[0]) * (mles[3] - mles[2])
    );
    let mut transcript = TestTranscript::new();
    assert_eq!(
        proof.evaluation_point[0],
        transcript.round_challenge(&proof.round_polynomials)
    );
}
