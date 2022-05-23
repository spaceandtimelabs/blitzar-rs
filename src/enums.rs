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

pub enum Sequence {
    Bytes8(Vec<u8>),
    Bytes16(Vec<u16>),
    Bytes32(Vec<u32>),
    Bytes64(Vec<u64>),
}

impl Sequence {
    pub fn sizeof(&self) -> u8 {
        match &self {
            Sequence::Bytes8(_v) => 1,
            Sequence::Bytes16(_v) => 2,
            Sequence::Bytes32(_v) => 4,
            Sequence::Bytes64(_v) => 8,
        }
    }
    
    pub fn as_ptr(&self) -> *const u8 {
        match &self {
            Sequence::Bytes8(v) => v.as_ptr() as *const u8,
            Sequence::Bytes16(v) => v.as_ptr() as *const u8,
            Sequence::Bytes32(v) => v.as_ptr() as *const u8,
            Sequence::Bytes64(v) => v.as_ptr() as *const u8,
        }
    }

    pub fn len(&self) -> usize {
        match &self {
            Sequence::Bytes8(v) => v.len(),
            Sequence::Bytes16(v) => v.len(),
            Sequence::Bytes32(v) => v.len(),
            Sequence::Bytes64(v) => v.len(),
        }
    }
}
