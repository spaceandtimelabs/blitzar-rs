use crate::proof::field::FieldId;

pub trait SumcheckTranscript<T: FieldId> {
    fn init(num_variables: usize, round_degree: usize);

    fn round_challenge(polynomial: &[T]) -> T;
}
