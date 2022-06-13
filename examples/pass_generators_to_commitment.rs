extern crate pedersen;
extern crate curve25519_dalek;

use pedersen::sequence::*;
use pedersen::commitments::*;

extern crate rand_core;
use rand_core::OsRng;

fn main() {
    // generate input table
    let mut table: Vec<Sequence> = Vec::new();

    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation. Each vector
    // will be translated into a single 32 bytes dalek CompressedRistretto data
    //
    // Note that you must specify the vector element type (u8, u16, u32, u64, u128)
    //
    // Also, you must provide a generator vector `gs`, below defined, to the computation
    //
    // For instance:
    //     commitment[0] = gs[0]*data[0] + gs[1]*data[1] + gs[2]*data[2] + gs[3]^data[3]
    //                   = gs[0]*2 + gs[1]*3 + gs[2]*1 + gs[3]*5 + ... + gs[9] ^ 10
    //
    /////////////////////////////////////////////
    let data: Vec<u16> = vec![2, 3, 1, 5, 4, 7, 6, 8, 9, 10];

    /////////////////////////////////////////////
    // randomly obtain the generator points
    /////////////////////////////////////////////
    let mut rng = OsRng;
    let gs: Vec<CompressedRistretto> =
        (0..data.len()).map(|_| RistrettoPoint::random(&mut rng).compress()).collect();

    /////////////////////////////////////////////
    // Fill the table with entries
    // 
    // We need to wrapper the vector array inside the table object.
    // This object holds a slice of the data vector and the
    // total amount of bytes of each element stored in the vector
    /////////////////////////////////////////////
    table.push(Sequence::Dense(DenseSequence {
        data_slice: &data.as_byte_slice(),
        element_size: std::mem::size_of_val(&data[0])
    }));

    /////////////////////////////////////////////
    // We need to define a commitment vector which 
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0 as u8; 32]); table.len()];

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_commitments_with_sequences_and_generators(
        &mut commitments,
        &table,
        &gs
    );

    /////////////////////////////////////////////
    // Use Dalek library to obtain the same
    // commitment that was computed in the GPU or
    // CPU above. Following, we randomly
    // obtain the generators
    /////////////////////////////////////////////
    let mut expected_commit = match CompressedRistretto::from_slice(&[0 as u8; 32]).decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    /////////////////////////////////////////////
    // Then we use the above generators `gs`,
    // as well as the data table as scalars
    // to obtain the same commitment that
    // was computed in the GPU / CPU, the `expected_commit`
    /////////////////////////////////////////////
    for i in 0..gs.len() {
        let mut scalar_bytes: [u8; 32] = [0; 32];
        scalar_bytes[0] = data[i] as u8;

        // Construct a Scalar by reducing a 256-bit little-endian integer modulo the group order ℓ.
        let ristretto_sc = curve25519_dalek::scalar::Scalar::from_bytes_mod_order(scalar_bytes);

        let g_i = match gs[i].decompress() {
            Some(pt) => pt,
            None => panic!("Invalid ristretto point decompression")
        };

        expected_commit = expected_commit + ristretto_sc * g_i;
    }

    /////////////////////////////////////////////
    // Compare the Dalek and our CPU/GPU commitment
    /////////////////////////////////////////////
    println!("Computed Commitment: {:?}\n", commitments[0]);
    println!("Expected Commitment: {:?}\n", expected_commit.compress());
}
