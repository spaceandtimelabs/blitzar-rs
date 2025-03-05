use super::*;
use ark_grumpkin::Fr;
use merlin::Transcript;
use crate::proof::SumcheckTranscript;

struct TestTranscript {
    base: Transcript,
}

impl TestTranscript {
    pub fn new() -> Self {
        Self{
            base: Transcript::new(b"abc123")
        }
    }
}

impl SumcheckTranscript<Fr> for TestTranscript {
    fn init(&mut self, num_variables: usize, round_degree: usize) {
    }

    fn round_challenge(&mut self, polynomial: &[Fr]) -> Fr {
        Fr::from(123)
    }
}
    // let mut transcript = Transcript::new(b"innerproducttest");

#[test]
fn we_can_prove_sumcheck_with_an_mle_with_a_single_element() {
    let mles = vec![Fr::from(123)];
    // pub fn new<Transcript: SumcheckTranscript<T>>(
    //     transcript: &mut Transcript,
    //     mles: &[T],
    //     product_table: &[(T, u32)],
    //     product_terms: &[u32],
    //     n: u32,
    // ) -> Self {
}
