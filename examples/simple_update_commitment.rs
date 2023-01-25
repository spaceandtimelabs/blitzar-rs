extern crate curve25519_dalek;
extern crate proofs_gpu;

use byte_slice_cast::AsByteSlice;
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use proofs_gpu::compute::*;
use proofs_gpu::sequences::*;

fn main() {
    let dense_data: Vec<u32> = vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0];

    /////////////////////////////////////////////
    // This scalars_data will effectively only
    // have information in the positions 2 and 3
    /////////////////////////////////////////////
    let scalar_data: Vec<Scalar> = vec![Scalar::from(5000_u32), Scalar::from(1500_u32)];

    /////////////////////////////////////////////
    // We build the array with the expected results
    // expected_data = dense_data + scalar_data
    /////////////////////////////////////////////
    let expected_data: Vec<u32> = vec![1, 0, 5002, 1500, 3, 4, 0, 0, 0, 9, 0];

    let mut commitment = CompressedRistretto::from_slice(&[0_u8; 32]);
    let mut expected_commitment = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    /////////////////////////////////////////////
    // We compute the commitments using the exact
    // data, which stores `dense_data + scalar_data`
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
    // `commitment = dense_data + scalar_data`
    // Notice that we only pass the scalar values from 2 to 3.
    // Therefore, we need to specify the offsets
    // that will be used do query the correct generators.
    // For instance, the following is aplied:
    // commitment += (generator[0 + 2] * scalar_data[0] +
    //                  + generator[1 + 2] * scalar_data[1])
    /////////////////////////////////////////////
    update_commitment(&mut commitment, 2_u64, &scalar_data[..]);

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
