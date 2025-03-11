/// Provide random challenges for sumcheck proof.
pub trait SumcheckTranscript<T> {
    /// Provides dimensions of sumcheck proof.
    fn init(&mut self, num_variables: usize, round_degree: usize);

    /// Produce the challenge field element for a sumcheck
    /// round given the round polynomial
    ///   polynomial[0] + polynomial[1] X^1 + ... + polynomial[d] X^d
    fn round_challenge(&mut self, polynomial: &[T]) -> T;
}
