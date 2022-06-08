extern crate pedersen;
extern crate curve25519_dalek;

use pedersen::sequence::*;
use pedersen::commitments::*;

fn main() {
    init_backend();

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
        data_slice: &data.as_byte_slice(),
        element_size: std::mem::size_of_val(&data[0])
    }));

    /////////////////////////////////////////////
    // Define a commitment vector to store all the results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0 as u8; 32]); table.len()];

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_commitments(& mut commitments, &table);

    for i in 0..table.len() {
        println!("commitment {}: {:?}\n", i, commitments[i]);
    }
}
