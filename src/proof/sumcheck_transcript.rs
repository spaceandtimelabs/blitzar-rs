/// todo: doc me
pub trait SumcheckTranscript<T> {
    /// todo: doc me
    fn init(&mut self, num_variables: usize, round_degree: usize);

    /// todo: doc me
    fn round_challenge(&mut self, polynomial: &[T]) -> T;
}
