pub trait SumcheckTranscript<T> {
    fn init(&mut self, num_variables: usize, round_degree: usize);

    fn round_challenge(&mut self, polynomial: &[T]) -> T;
}
