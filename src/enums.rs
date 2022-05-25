pub enum Backend {
    CPU,
    GPU
}

impl Backend {
    pub fn value(&self) -> u32 {
        match *self {
            Backend::CPU => proofs_gpu::SXT_BACKEND_CPU,
            Backend::GPU => proofs_gpu::SXT_BACKEND_GPU,
        }
    }
}

pub struct DenseSequence<'a> {
    pub data_slice: &'a [u8],
    pub element_size: usize
}

pub enum Sequence<'a> {
    Dense(DenseSequence<'a>)
}
