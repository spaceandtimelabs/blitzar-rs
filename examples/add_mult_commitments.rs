extern crate pedersen;
extern crate curve25519_dalek;

use pedersen::sequence::*;
use pedersen::commitments::*;

fn main() {
    // generate input table
    let multiplier: u64 = 52;
    
    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation.
    /////////////////////////////////////////////
    let weekly_pay_data: Vec<u16> = vec![
        2000,
        7500,
        5000,
        1500
    ];

    let yearly_bonus_data: Vec<u32> = vec![
        5000,
        0,
        400000,
        10,
        0,
        0
    ];

    let total_compensation_data: Vec<u64> = vec![
        multiplier * 2000 + 5000,
        multiplier * 7500 + 0,
        multiplier * 5000 + 400000,
        multiplier * 1500 + 10
    ];

    /////////////////////////////////////////////
    // Fill the table with entries
    /////////////////////////////////////////////
    let mut table: Vec<Sequence> = Vec::new();

    table.push(Sequence::Dense(DenseSequence {
        data_slice: &weekly_pay_data.as_byte_slice(),
        element_size: std::mem::size_of_val(&weekly_pay_data[0])
    }));

    table.push(Sequence::Dense(DenseSequence {
        data_slice: &yearly_bonus_data.as_byte_slice(),
        element_size: std::mem::size_of_val(&yearly_bonus_data[0])
    }));
    
    table.push(Sequence::Dense(DenseSequence {
        data_slice: &total_compensation_data.as_byte_slice(),
        element_size: std::mem::size_of_val(&total_compensation_data[0])
    }));

    /////////////////////////////////////////////
    // We need to define a commitment vector which 
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0 as u8; 32]); table.len()];
    
    /////////////////////////////////////////////
    // Do the actual commitment computation (either in cpu / gpu)
    /////////////////////////////////////////////
    compute_commitments_with_sequences(& mut commitments, &table);
    
    /////////////////////////////////////////////
    // Converts the commitment results from dalek
    // CompressedRistretto points to u8, 32 byte arrays
    /////////////////////////////////////////////
    let commit_weekly_pay = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let commit_yearly_bonus = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let expected_commit_total_compensation = commitments[2];

    /////////////////////////////////////////////
    // We need dalek scalars to be able to multiply commitments with scalars:
    //     commit_total_compensation = 52 * commit_weekly_pay + commit_yearly_bonus
    //
    // Since dalek `Scalar` types only supports 32 byte arrays
    // we must pass
    /////////////////////////////////////////////
    let mut scalar_bytes: [u8; 32] = [0; 32];
    scalar_bytes[0] = multiplier as u8;

    /////////////////////////////////////////////
    // Construct a Scalar with `from_bytes_mod_order`
    // by reducing a 256-bit little-endian integer
    // modulo the group order ℓ.
    //
    // The Scalar struct holds an integer `ristretto_sc` < 2^{255}
    //
    // see: docs.rs/curve25519-dalek/0.19.1/curve25519_dalek/scalar/index.html
    /////////////////////////////////////////////
    let ristretto_sc = curve25519_dalek::scalar::Scalar::from_bytes_mod_order(scalar_bytes);

    let commit_total_compensation = (ristretto_sc * commit_weekly_pay + commit_yearly_bonus).compress();

    /////////////////////////////////////////////
    // Compares if the `commit_total_compensation`
    // calculated from `52 * commit_weekly_pay + commit_yearly_bonus`
    // by the dalek library is equal to `expected_commit_total_compensation`
    // computed by our gpu / cpu code
    /////////////////////////////////////////////
    if commit_total_compensation == expected_commit_total_compensation {
        println!("Commitments are equal:\n\tComputed - {:?}\n\tExpected - {:?}", 
            commit_total_compensation, expected_commit_total_compensation);
    } else {
        println!("Commitments differ:\n\t{:?}\n\t{:?}",
            commit_total_compensation, expected_commit_total_compensation);
    }
}
