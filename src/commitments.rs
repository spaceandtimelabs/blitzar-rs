
extern crate proofs_gpu;
extern crate curve25519_dalek;

use crate::enums::Backend;
use crate::enums::Sequence;

pub type Commitment = curve25519_dalek::ristretto::CompressedRistretto;

pub fn init_backend(curr_backend: Backend) {
    let ret_init;
    let config: proofs_gpu::sxt_config = proofs_gpu::sxt_config {
        backend: curr_backend.value() as i32
    };

    unsafe {
        ret_init = proofs_gpu::sxt_init(&config);
    };

    if ret_init != 0 {
        panic!("Error during backend initialization");
    }
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
        let num_bytes = data[i].sizeof();
        let num_rows = data[i].len() as u64;

        let descriptor = proofs_gpu::sxt_dense_sequence_descriptor {
            element_nbytes: num_bytes,  // number bytes
            n: num_rows,            // number rows
            data: data[i].as_ptr()   // data pointer
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

#[test]
mod test_commitments {
    use super::*;

    fn compute_commitments_works() {
        // initialize backend, choosing between GPU and CPU
        init_backend(Backend::GPU);

        // generate input table
        let data1 = vec![2000, 7500, 5000, 1500];
        let data2 = vec![5000, 0, 400000, 10, 0, 0];
        let data3 = vec![2000 + 5000, 7500 + 0, 5000 + 400000, 1500 + 10];

        let mut table: Vec<Sequence> = Vec::new();
        
        table.push(Sequence::Bytes16(&data1));
        table.push(Sequence::Bytes32(&data2));
        table.push(Sequence::Bytes64(&data3));

        let commit1 = Commitment::from_slice(
            &([
                4,105,58,131,59,69,150,106,
                120,137,32,225,175,244,82,115,
                216,180,206,150,21,250,240,98,
                251,192,146,244,54,169,199,97
            ] as [u8; 32])
        );

        let mut commitments = vec![commit1; table.len()];
        
        compute_commitments(& mut commitments[..], &table);

        let commit2 = Commitment::from_slice(
            &([
                2,254,178,195,198,238,44,156,
                24,29,88,196,37,63,157,50,
                236,159,61,49,153,181,79,126,
                55,188,67,1,228,248,72,51
            ] as [u8; 32])
        );

        let commit3 = Commitment::from_slice(
            &([
                30,237,163,234,252,111,45,133,
                235,227,21,117,229,188,88,149,
                240,109,205,90,6,130,199,152,
                5,221,57,231,168,9,141,122
            ] as [u8; 32])
        );

        // verify if commitment results are correct
        assert_eq!(commitments[0], commit1);
        assert_eq!(commitments[1], commit2);
        assert_eq!(commitments[2], commit3);
    }
}
