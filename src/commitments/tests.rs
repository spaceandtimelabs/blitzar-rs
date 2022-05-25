#[cfg(test)]
use super::*;

#[test]
fn compute_commitments_works() {
    // generate input table
    let mut table: Vec<Sequence> = Vec::new();
    
    let data1 = vec![2000, 7500, 5000, 1500];
    table.push(Sequence::Bits16(&data1));

    let mut commitments = vec![Commitment::from_slice(&[0 as u8; 32]); table.len()];
    
    compute_commitments(& mut commitments, &table);

    let commit1 = Commitment::from_slice(
        &([
            4,105,58,131,59,69,150,106,
            120,137,32,225,175,244,82,115,
            216,180,206,150,21,250,240,98,
            251,192,146,244,54,169,199,97
        ] as [u8; 32])
    );

    // verify if commitment results are correct
    assert_eq!(commitments[0], commit1);
}


#[test]
fn commit_a_plus_commit_b_equal_to_commit_c() {
    // generate input table
    let mut table: Vec<Sequence> = Vec::new();
    
    let data_a = vec![2000, 7500, 5000, 1500];
    table.push(Sequence::Bits16(&data_a));

    let data_b = vec![5000, 0, 400000, 10, 0, 0];
    table.push(Sequence::Bits32(&data_b));

    let data_c = vec![2000 + 5000, 7500 + 0, 5000 + 400000, 1500 + 10];
    table.push(Sequence::Bits64(&data_c));

    let mut commitments = vec![Commitment::from_slice(&[0 as u8; 32]); table.len()];
    
    compute_commitments(& mut commitments, &table);

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let expected_commit_c = commitments[2];

    let commit_c = (commit_a + commit_b).compress();

    // checking if commits are non-zero and different from each other
    // we know that all data differ, then all commits must differ too
    for i in 0..commitments.len() {
        for j in (i + 1)..commitments.len() {
            assert_ne!(commitments[i], commitments[j]);
        }
    }

    // verify if commitment results are correct
    assert_eq!(commit_c, expected_commit_c);
}

#[test]
fn commit_1_plus_commit_1_plus_commit_1_equal_to_commit_3() {
    // generate input table
    let mut table: Vec<Sequence> = Vec::new();
    
    let data_a = vec![1];
    table.push(Sequence::Bits16(&data_a));

    let data_b = vec![1];
    table.push(Sequence::Bits32(&data_b));

    let data_c = vec![1];
    table.push(Sequence::Bits64(&data_c));

    let data_d = vec![3];
    table.push(Sequence::Bits64(&data_d));

    let mut commitments = vec![Commitment::from_slice(&[0 as u8; 32]); table.len()];
    
    compute_commitments(& mut commitments, &table);

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let commit_c = match commitments[2].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let expected_commit_d = commitments[3];

    let commit_d = (commit_a + commit_b + commit_c).compress();

    // checking if commits are non-zero and different from each other
    // we know that all data differ, then all commits must differ too
    assert_ne!(commitments[0], commitments[3]);

    // verify if commitment results are correct
    assert_eq!(commit_d, expected_commit_d);
}

#[test]
fn commit_a_times_52_plus_commit_b_equal_to_commit_c() {
    // generate input table
    let sc: u64 = 52;
    let mut table: Vec<Sequence> = Vec::new();
    
    let data_a = vec![2000, 7500, 5000, 1500];
    table.push(Sequence::Bits16(&data_a));

    let data_b = vec![5000, 0, 400000, 10, 0, 0];
    table.push(Sequence::Bits32(&data_b));

    let data_c = vec![sc * 2000 + 5000, 
        sc * 7500 + 0, sc * 5000 + 400000, sc * 1500 + 10];
    table.push(Sequence::Bits64(&data_c));

    let mut commitments = vec![Commitment::from_slice(&[0 as u8; 32]); table.len()];
    
    compute_commitments(& mut commitments, &table);

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let mut scalar_bytes: [u8; 32] = [0; 32];
    scalar_bytes[0] = sc as u8;

    // Construct a Scalar by reducing a 256-bit little-endian integer modulo the group order ℓ.
    let ristretto_sc = curve25519_dalek::scalar::Scalar::from_bytes_mod_order(scalar_bytes);

    let expected_commit_c = commitments[2];

    let commit_c = (ristretto_sc * commit_a + commit_b).compress();

    // checking if commits are non-zero and different from each other
    // we know that all data differ, then all commits must differ too
    for i in 0..commitments.len() {
        for j in (i + 1)..commitments.len() {
            assert_ne!(commitments[i], commitments[j]);
        }
    }

    // verify if commitment results are correct
    assert_eq!(commit_c, expected_commit_c);
}


#[test]
fn commit_negative_a_plus_commit_negative_b_equal_to_commit_c() {
    // generate input table
    let mut table: Vec<Sequence> = Vec::new();

    let a = -128;
    let data_a = vec![a as u16];
    table.push(Sequence::Bits16(&data_a));

    let b = -128;
    let data_b = vec![b as u16];
    table.push(Sequence::Bits16(&data_b));

    let data_c = vec![130816];
    table.push(Sequence::Bits32(&data_c));

    let mut commitments = vec![Commitment::from_slice(&[0 as u8; 32]); table.len()];
    
    compute_commitments(& mut commitments, &table);

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };
    
    let expected_commit_c = commitments[2];

    let commit_c = (commit_a + commit_b).compress();

    // checking if commits are non-zero and different from each other
    // we know that all data differ, then all commits must differ too
    assert_ne!(commitments[0], commitments[2]);

    // verify if commitment results are correct
    assert_eq!(commit_c, expected_commit_c);
}


#[test]
fn different_word_size_and_rows_in_commit_a_plus_commit_b_plus_commit_c_equal_to_commit_d() {
    // generate input table
    let mut table: Vec<Sequence> = Vec::new();

    let data_a = vec![
        6346243789798364141,
        1503914060200516822,
        1,
        1152921504606846976
    ];
    table.push(Sequence::Bits64(&data_a));

    let data_b = vec![123, 733];
    table.push(Sequence::Bits32(&data_b));

    let data_c = vec![121, 200, 135];
    table.push(Sequence::Bits8(&data_c));

    let data_d = vec![
        6346243789798364385,
        1503914060200517755,
        136,
        1152921504606846976
    ];
    table.push(Sequence::Bits64(&data_d));

    let mut commitments = vec![Commitment::from_slice(&[0 as u8; 32]); table.len()];
    
    compute_commitments(& mut commitments, &table);

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };
    
    let commit_c = match commitments[2].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let expected_commit_d = commitments[3];

    let commit_d = (commit_a + commit_b + commit_c).compress();

    // checking if commits are non-zero and different from each other
    // we know that all data differ, then all commits must differ too
    for i in 0..commitments.len() {
        for j in (i + 1)..commitments.len() {
            assert_ne!(commitments[i], commitments[j]);
        }
    }

    // verify if commitment results are correct
    assert_eq!(commit_d, expected_commit_d);
}
