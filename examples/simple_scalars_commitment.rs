extern crate curve25519_dalek;
extern crate proofs_gpu;

use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use proofs_gpu::compute::*;

fn main() {
    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation. Each vector
    // is a Dalek Scalar, which is simply a 256-bit integer < ℓ (the group prime order)
    //
    // For instance:
    //     commitment[0] = g[0]*data1[0] + g[1]*data1[1] + g[2]*data1[2] + g[3]^data1[3]
    //
    /////////////////////////////////////////////
    let mut data: Vec<Scalar> = vec![
        Scalar::zero(),
        Scalar::one(),
        Scalar::zero(),
        Scalar::from_bytes_mod_order([4; 32]),
    ];

    // data[2] = 2000 as 256-bits
    for _i in 0..2000 {
        data[2] += Scalar::one();
    }

    /////////////////////////////////////////////
    // Fill the table with entries
    /////////////////////////////////////////////
    let table: Vec<&[Scalar]> = vec![&data];

    /////////////////////////////////////////////
    // We need to define a commitment vector which
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); table.len()];

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_commitments(&mut commitments, &table);

    for (i, commit) in commitments.iter().enumerate() {
        println!("commitment {}: {:?}\n", i, commit);
    }
}
