extern crate pedersen;
extern crate curve25519_dalek;

use pedersen::commitments::*;

fn main() {
    // generate input table
    let mut table: Vec<&[Scalar]> = Vec::new();

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
    for _i in 0..2000 { data[2] = data[2] + Scalar::one(); }

    /////////////////////////////////////////////
    // Fill the table with entries
    /////////////////////////////////////////////
    table.push(&data);

    /////////////////////////////////////////////
    // We need to define a commitment vector which 
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0 as u8; 32]); table.len()];
    
    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_commitments(& mut commitments, &table);

    for i in 0..commitments.len() {
        println!("commitment {}: {:?}\n", i, commitments[i]);
    }
}
