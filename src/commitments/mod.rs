
extern crate proofs_gpu;
extern crate curve25519_dalek;

use std::sync::Once;

use crate::sequence::Sequence;

pub type Commitment = curve25519_dalek::ristretto::CompressedRistretto;

static mut INIT_STATE: i32 = 0;
static INIT: Once = Once::new();

pub fn init_backend() {
    unsafe {
        INIT.call_once(|| {
            let curr_backend;
            
            if cfg!(feature = "cpu") {
                curr_backend = proofs_gpu::SXT_BACKEND_CPU;
            } else {
                curr_backend = proofs_gpu::SXT_BACKEND_GPU;
            }

            let config: proofs_gpu::sxt_config = proofs_gpu::sxt_config {
                backend: curr_backend as i32
            };
        
            INIT_STATE = proofs_gpu::sxt_init(&config);
        });
        
        if INIT_STATE != 0 {
            panic!("Error during backend initialization");
        }
    };
}

fn to_sxt_commitments(num_sequences: usize)
    -> Vec<proofs_gpu::sxt_commitment> {

    let mut cbinding_commitments: Vec<proofs_gpu::
            sxt_commitment> = Vec::with_capacity(num_sequences);

    unsafe {
        cbinding_commitments.set_len(num_sequences);
    }

    return cbinding_commitments;
}

fn to_sxt_descriptors(data: & [Sequence])
     -> Vec<proofs_gpu::sxt_sequence_descriptor> {

    let num_sequences = data.len();
    let mut cbinding_descriptors: Vec<proofs_gpu::
        sxt_sequence_descriptor> = Vec::with_capacity(num_sequences);

    unsafe {
        cbinding_descriptors.set_len(num_sequences);
    }

    for i in 0..num_sequences {
        let curr_data = match &data[i] {
            Sequence::Dense(x) => x
        };

        debug_assert!(curr_data.data_slice.len() % curr_data.element_size == 0);

        let num_rows = curr_data.data_slice.len() / curr_data.element_size;

        let descriptor = proofs_gpu::sxt_dense_sequence_descriptor {
            element_nbytes: curr_data.element_size as u8,  // number bytes
            n: num_rows as u64,            // number rows
            data: curr_data.data_slice.as_ptr()   // data pointer
        };

        cbinding_descriptors[i] = proofs_gpu::sxt_sequence_descriptor {
            sequence_type: proofs_gpu::SXT_DENSE_SEQUENCE_TYPE as u8,
            __bindgen_anon_1: proofs_gpu::sxt_sequence_descriptor__bindgen_ty_1 {
                dense: descriptor
            }
        };
    }

    return cbinding_descriptors;
}

fn to_commitments(commitments: & mut[Commitment], sxt_commitments: &[proofs_gpu::sxt_commitment]) {
    let num_sequences = sxt_commitments.len();
    
    // copy results back to commitments vector
    for i in 0..num_sequences {
        commitments[i] = Commitment::
                from_slice(&sxt_commitments[i].ristretto_bytes);
    }
}

pub fn compute_commitments(commitments: & mut[Commitment], data: & [Sequence])  {
    let ret_compute;
    let num_sequences = data.len();
    let mut sxt_descriptors = to_sxt_descriptors(data);
    let mut sxt_commitments = to_sxt_commitments(num_sequences);

    init_backend();

    unsafe {
        ret_compute = proofs_gpu::sxt_compute_pedersen_commitments(
            sxt_commitments.as_mut_ptr(),
            num_sequences as u32,
            sxt_descriptors.as_mut_ptr(),
        );
    }

    if ret_compute != 0 {
        panic!("Error during commitments computation");
    }

    to_commitments(commitments, &sxt_commitments);
}

mod tests;
