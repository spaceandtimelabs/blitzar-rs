extern crate curve25519_dalek;
extern crate proofs_gpu;

use byte_slice_cast::AsByteSlice;
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use proofs_gpu::compute::*;
use proofs_gpu::sequences::*;

fn main() {
    /////////////////////////////////////////////
    // The sparse_data is associated with the
    // rows 0, 2, 4, 5, and 9
    /////////////////////////////////////////////
    let sparse_data: Vec<u32> = vec![1, 2, 3, 4, 9];
    let sparse_indices: Vec<u64> = vec![0, 2, 4, 5, 9];

    /////////////////////////////////////////////
    // This dense_data is exactly the same as the
    // sparse_data, expect that it stores zeros and
    // the sparse_data does not. But the non-zero
    // elements from dense_data are also present in
    // the sparse_data array
    /////////////////////////////////////////////
    let dense_data: Vec<u32> = vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0];

    /////////////////////////////////////////////
    // This scalars_data will effectively only
    // have information in the positions 2 and 3
    /////////////////////////////////////////////
    let mut scalar_data: Vec<Scalar> = vec![Scalar::zero(); 4];

    for _i in 0..5000 {
        scalar_data[2] += Scalar::one();
    }
    for _i in 0..1500 {
        scalar_data[3] += Scalar::one();
    }

    /////////////////////////////////////////////
    // We build the array with the expected results
    // expected_data = sparse_data + dense_data + scalar_data
    /////////////////////////////////////////////
    let expected_data: Vec<u32> = vec![2, 0, 5004, 1500, 6, 8, 0, 0, 0, 18, 0];

    let mut commitment = CompressedRistretto::from_slice(&[0_u8; 32]);
    let mut expected_commitment = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    /////////////////////////////////////////////
    // We compute the commitments using the exact
    // data, which stores `sparse_data + dense_data + scalar_data`
    /////////////////////////////////////////////
    compute_commitments(
        &mut expected_commitment,
        &[Sequence::Dense(DenseSequence {
            data_slice: expected_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&expected_data[0]),
        })],
        0_u64,
    );

    /////////////////////////////////////////////
    // Up to this point, commitment was 0. Then
    // we update it, so that `commitment = dense_data`
    /////////////////////////////////////////////
    update_commitment(
        &mut commitment,
        0_u64,
        Sequence::Dense(DenseSequence {
            data_slice: dense_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&dense_data[0]),
        }),
    );

    /////////////////////////////////////////////
    // We then we update the commiment, so that
    // `commitment = dense_data + sparse_data`
    /////////////////////////////////////////////
    update_commitment(
        &mut commitment,
        0_u64,
        Sequence::Sparse(SparseSequence {
            data_slice: sparse_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&sparse_data[0]),
            data_indices: &sparse_indices,
        }),
    );

    /////////////////////////////////////////////
    // We then we update the commiment, so that
    // `commitment = dense_data + sparse_data + scalar_data`
    // Notice that we only pass the scalar values from 2 to 3.
    // Therefore, we need to specify the offsets
    // that will be used do query the correct generators.
    // For instance, the following is aplied:
    // commitment += (generator[0 + 2] * scalar_data[0] +
    //                  + generator[1 + 2] * scalar_data[1])
    /////////////////////////////////////////////
    update_commitment(&mut commitment, 2_u64, &scalar_data[2..]);

    /////////////////////////////////////////////
    // We then compare the commitment results
    /////////////////////////////////////////////
    if commitment == expected_commitment[0] {
        println!("Commitments are equal: {:?}", commitment);
    } else {
        println!("Commitments are different:");
        println!("Commitment 1: {:?}", commitment);
        println!("Commitment 1: {:?}", expected_commitment);
    }
}
