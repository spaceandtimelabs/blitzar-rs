extern crate blitzar;
extern crate curve25519_dalek;

use blitzar::compute::*;
use blitzar::sequences::*;

use byte_slice_cast::AsByteSlice;
use curve25519_dalek::ristretto::CompressedRistretto;

fn main() {
    let num_precomputed_generators: u64 = 100;

    init_backend_with_config(BackendConfig {
        num_precomputed_generators,
    });

    // generate input table
    let mut table: Vec<Sequence> = Vec::new();

    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation
    /////////////////////////////////////////////
    let data: Vec<u64> = vec![40, 32, 21, 10, 20, 35, 444];

    /////////////////////////////////////////////
    // Fill the table with entries
    /////////////////////////////////////////////
    table.push(Sequence::Dense(DenseSequence {
        data_slice: data.as_byte_slice(),
        element_size: std::mem::size_of_val(&data[0]),
    }));

    /////////////////////////////////////////////
    // Define a commitment vector to store all the results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); table.len()];

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_commitments(&mut commitments, &table, 0_u64);

    for (i, commit) in commitments.iter().enumerate() {
        println!("commitment {}: {:?}\n", i, commit);
    }
}
