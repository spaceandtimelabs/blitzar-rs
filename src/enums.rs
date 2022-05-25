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

pub enum Sequence<'a> {
    Bits8(&'a [u8]),
    Bits16(&'a [u16]),
    Bits32(&'a [u32]),
    Bits64(&'a [u64]),
    Bits128(&'a [u128]),
}

impl Sequence<'_> {
    pub fn num_bytes(&self) -> u8 {
        match &self {
            Sequence::Bits8(_v) => 1,
            Sequence::Bits16(_v) => 2,
            Sequence::Bits32(_v) => 4,
            Sequence::Bits64(_v) => 8,
            Sequence::Bits128(_v) => 8,
        }
    }
    
    pub fn as_ptr(&self) -> *const u8 {
        match &self {
            Sequence::Bits8(v) => v.as_ptr() as *const u8,
            Sequence::Bits16(v) => v.as_ptr() as *const u8,
            Sequence::Bits32(v) => v.as_ptr() as *const u8,
            Sequence::Bits64(v) => v.as_ptr() as *const u8,
            Sequence::Bits128(v) => v.as_ptr() as *const u8,
        }
    }

    pub fn len(&self) -> usize {
        match &self {
            Sequence::Bits8(v) => v.len(),
            Sequence::Bits16(v) => v.len(),
            Sequence::Bits32(v) => v.len(),
            Sequence::Bits64(v) => v.len(),
            Sequence::Bits128(v) => v.len(),
        }
    }
}
