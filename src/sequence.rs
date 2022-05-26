pub struct DenseSequence<'a> {
    pub data_slice: &'a [u8],
    pub element_size: usize
}

pub enum Sequence<'a> {
    Dense(DenseSequence<'a>)
}
