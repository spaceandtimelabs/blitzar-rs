// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

#![allow(unused_imports)]

use super::*;
use crate::sequence::DenseSequence;
use crate::sequence::SparseSequence;

extern crate rand_core;

use rand_core::OsRng;

#[test]
fn compute_commitments_works() {
    // generate input table
    let data: Vec<u32> = vec![2000, 7500, 5000, 1500];
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    compute_commitments(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        })],
    );

    let expected_commit = CompressedRistretto::from_slice(
        &([
            4, 105, 58, 131, 59, 69, 150, 106, 120, 137, 32, 225, 175, 244, 82, 115, 216, 180, 206,
            150, 21, 250, 240, 98, 251, 192, 146, 244, 54, 169, 199, 97,
        ] as [u8; 32]),
    );

    // verify if commitment results are correct
    assert_eq!(commitments[0], expected_commit);
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[0]);
}

#[test]
fn compute_sparse_commitments_works() {
    // generate input table
    let sparse_data: Vec<u32> = vec![1, 2, 3, 4, 9];
    let sparse_indices: Vec<u64> = vec![0, 2, 4, 5, 9];
    let dense_data: Vec<u32> = vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0];

    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 2];

    compute_commitments(
        &mut commitments,
        &[
            Sequence::Dense(DenseSequence {
                data_slice: dense_data.as_byte_slice(),
                element_size: std::mem::size_of_val(&dense_data[0]),
            }),
            Sequence::Sparse(SparseSequence {
                data_slice: sparse_data.as_byte_slice(),
                element_size: std::mem::size_of_val(&sparse_data[0]),
                data_indices: &sparse_indices,
            }),
        ],
    );

    // verify if commitment results are correct
    assert_eq!(commitments[0], commitments[1]);
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[0]);
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[1]);
}

#[test]
fn compute_update_commitment_works() {
    // generate input table
    let sparse_data: Vec<u32> = vec![1, 2, 3, 4, 9];
    let sparse_indices: Vec<u64> = vec![0, 2, 4, 5, 9];
    let dense_data: Vec<u32> = vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0];
    let mut scalar_data: Vec<Scalar> = vec![Scalar::zero(); 4];
    let expected_data: Vec<u32> = vec![2, 0, 5004, 1500, 6, 8, 0, 0, 0, 18, 0];

    for _i in 0..5000 {
        scalar_data[2] += Scalar::one();
    }
    for _i in 0..1500 {
        scalar_data[3] += Scalar::one();
    }

    let mut commitment = CompressedRistretto::from_slice(&[0_u8; 32]);
    let mut expected_commitment = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    update_commitment(
        &mut commitment,
        0_u64,
        Sequence::Dense(DenseSequence {
            data_slice: dense_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&dense_data[0]),
        }),
    );

    update_commitment(
        &mut commitment,
        0_u64,
        Sequence::Sparse(SparseSequence {
            data_slice: sparse_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&sparse_data[0]),
            data_indices: &sparse_indices,
        }),
    );

    update_commitment(&mut commitment, 2_u64, &scalar_data[2..]);

    compute_commitments(
        &mut expected_commitment,
        &[Sequence::Dense(DenseSequence {
            data_slice: expected_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&expected_data[0]),
        })],
    );

    // verify if commitment results are correct
    assert_eq!(commitment, expected_commitment[0]);
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitment);
    assert_ne!(
        CompressedRistretto::from_slice(&[0_u8; 32]),
        expected_commitment[0]
    );
}

#[test]
fn compute_commitments_with_scalars_works() {
    // generate input table
    let mut data: Vec<Scalar> = vec![Scalar::zero(); 4];

    for _i in 0..2000 {
        data[0] += Scalar::one();
    }
    for _i in 0..7500 {
        data[1] += Scalar::one();
    }
    for _i in 0..5000 {
        data[2] += Scalar::one();
    }
    for _i in 0..1500 {
        data[3] += Scalar::one();
    }

    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    compute_commitments(&mut commitments, &[&data[..]]);

    let expected_commit = CompressedRistretto::from_slice(
        &([
            4, 105, 58, 131, 59, 69, 150, 106, 120, 137, 32, 225, 175, 244, 82, 115, 216, 180, 206,
            150, 21, 250, 240, 98, 251, 192, 146, 244, 54, 169, 199, 97,
        ] as [u8; 32]),
    );

    // verify if commitment results are correct
    assert_eq!(commitments[0], expected_commit);
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[0]);
}

#[test]
fn commit_a_plus_commit_b_equal_to_commit_c() {
    // generate input table
    let data_a: Vec<u16> = vec![2000, 7500, 5000, 1500];
    let data_b: Vec<u32> = vec![5000, 0, 400000, 10, 0, 0];
    let data_c: Vec<u64> = vec![2000 + 5000, 7500, 5000 + 400000, 1500 + 10];

    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 3];

    compute_commitments(
        &mut commitments,
        &[
            Sequence::Dense(DenseSequence {
                data_slice: data_a.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_a[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_b.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_b[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_c.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_c[0]),
            }),
        ],
    );

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
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
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commit_c);
}

#[test]
fn commit_1_plus_commit_1_plus_commit_1_equal_to_commit_3() {
    // generate input table
    let data_a: Vec<u16> = vec![1];
    let data_b: Vec<u32> = vec![1];
    let data_c: Vec<u64> = vec![1];
    let data_d: Vec<u64> = vec![3];

    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 4];

    compute_commitments(
        &mut commitments,
        &[
            Sequence::Dense(DenseSequence {
                data_slice: data_a.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_a[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_b.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_b[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_c.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_c[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_d.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_d[0]),
            }),
        ],
    );

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    let commit_c = match commitments[2].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    let expected_commit_d = commitments[3];

    let commit_d = (commit_a + commit_b + commit_c).compress();

    // checking if commits are non-zero and different from each other
    // we know that all data differ, then all commits must differ too
    assert_ne!(commitments[0], commitments[3]);

    // verify if commitment results are correct
    assert_eq!(commit_d, expected_commit_d);
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commit_d);
}

#[test]
fn commit_a_times_52_plus_commit_b_equal_to_commit_c() {
    // generate input table
    let sc: u64 = 52;

    let data_a: Vec<u16> = vec![2000, 7500, 5000, 1500];
    let data_b: Vec<u32> = vec![5000, 0, 400000, 10, 0, 0];
    let data_c: Vec<u64> = vec![
        sc * 2000 + 5000,
        sc * 7500,
        sc * 5000 + 400000,
        sc * 1500 + 10,
    ];

    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 3];

    compute_commitments(
        &mut commitments,
        &[
            Sequence::Dense(DenseSequence {
                data_slice: data_a.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_a[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_b.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_b[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_c.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_c[0]),
            }),
        ],
    );

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
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
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commit_c);
}

#[test]
fn commit_negative_a_plus_commit_negative_b_equal_to_commit_c() {
    // generate input table
    let a: i8 = -128;
    let b: i8 = -128;
    let data_a: Vec<u16> = vec![a as u16];
    let data_b: Vec<u16> = vec![b as u16];
    let data_c: Vec<u32> = vec![130816];

    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 3];

    compute_commitments(
        &mut commitments,
        &[
            Sequence::Dense(DenseSequence {
                data_slice: data_a.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_a[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_b.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_b[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_c.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_c[0]),
            }),
        ],
    );

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    let expected_commit_c = commitments[2];

    let commit_c = (commit_a + commit_b).compress();

    // checking if commits are non-zero and different from each other
    // we know that all data differ, then all commits must differ too
    assert_ne!(commitments[0], commitments[2]);

    // verify if commitment results are correct
    assert_eq!(commit_c, expected_commit_c);
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commit_c);
}

#[test]
fn different_word_size_and_rows_in_commit_a_plus_commit_b_plus_commit_c_equal_to_commit_d() {
    // generate input table
    let data_a: Vec<u64> = vec![
        6346243789798364141,
        1503914060200516822,
        1,
        1152921504606846976,
    ];

    let data_b: Vec<u32> = vec![123, 733];

    let data_c: Vec<u8> = vec![121, 200, 135];

    let data_d: Vec<u64> = vec![
        6346243789798364385,
        1503914060200517755,
        136,
        1152921504606846976,
    ];

    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 4];

    compute_commitments(
        &mut commitments,
        &[
            Sequence::Dense(DenseSequence {
                data_slice: data_a.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_a[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_b.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_b[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_c.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_c[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data_d.as_byte_slice(),
                element_size: std::mem::size_of_val(&data_d[0]),
            }),
        ],
    );

    let commit_a = match commitments[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    let commit_b = match commitments[1].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    let commit_c = match commitments[2].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
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
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commit_d);
}

#[test]
fn get_generators_is_the_same_used_in_commitment_computation() {
    // generate input table
    let data: Vec<u16> = vec![2, 3, 1, 5, 4, 7, 6, 8, 9, 10];

    let mut generators = vec![CompressedRistretto::from_slice(&[0_u8; 32]); data.len()];
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    // convert the generator points to compressed ristretto
    get_generators(&mut generators, 0_u64);

    compute_commitments(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        })],
    );

    let mut expected_commit = match CompressedRistretto::from_slice(&[0_u8; 32]).decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    for i in 0..generators.len() {
        let mut scalar_bytes: [u8; 32] = [0; 32];
        scalar_bytes[0] = data[i] as u8;

        // Construct a Scalar by reducing a 256-bit little-endian integer modulo the group order ℓ.
        let ristretto_sc = curve25519_dalek::scalar::Scalar::from_bytes_mod_order(scalar_bytes);

        let g_i = match generators[i].decompress() {
            Some(pt) => pt,
            None => panic!("Invalid ristretto point decompression"),
        };

        expected_commit += ristretto_sc * g_i;
    }

    assert_eq!(commitments[0], expected_commit.compress());
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[0]);
}

#[test]
fn get_generators_with_offset_is_the_same_used_in_commitment_computation() {
    // generate input table
    let data: Vec<u32> = vec![0, 0, 0, 0, 4, 7, 6, 8, 9, 10, 0, 0, 0];

    let offset_generators: usize = 4;
    let generators_len = data.len() - offset_generators - 3;
    let mut generators = vec![CompressedRistretto::from_slice(&[0_u8; 32]); generators_len];
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    // convert the generator points to compressed ristretto

    get_generators(&mut generators, offset_generators as u64);

    compute_commitments(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        })],
    );

    let mut expected_commit = match CompressedRistretto::from_slice(&[0_u8; 32]).decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    for i in 0..generators.len() {
        let mut scalar_bytes: [u8; 32] = [0; 32];
        scalar_bytes[0] = data[i + offset_generators] as u8;

        // Construct a Scalar by reducing a 256-bit little-endian integer modulo the group order ℓ.
        let ristretto_sc = curve25519_dalek::scalar::Scalar::from_bytes_mod_order(scalar_bytes);

        let g_i = match generators[i].decompress() {
            Some(pt) => pt,
            None => panic!("Invalid ristretto point decompression"),
        };

        expected_commit += ristretto_sc * g_i;
    }

    assert_eq!(commitments[0], expected_commit.compress());
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[0]);
}

#[test]
fn sending_generators_to_gpu_produces_correct_commitment_results() {
    // generate input table
    let data: Vec<u64> = vec![2, 3, 1, 5, 4, 7, 6, 8, 9, 10];

    let mut rng = OsRng;

    // randomly obtain the generator points
    let generator_points: Vec<CompressedRistretto> = (0..data.len())
        .map(|_| RistrettoPoint::random(&mut rng).compress())
        .collect();
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    compute_commitments_with_generators(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        })],
        &generator_points,
    );

    let mut expected_commit = match CompressedRistretto::from_slice(&[0_u8; 32]).decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    for i in 0..generator_points.len() {
        let mut scalar_bytes: [u8; 32] = [0; 32];
        scalar_bytes[0] = data[i] as u8;

        // Construct a Scalar by reducing a 256-bit little-endian integer modulo the group order ℓ.
        let ristretto_sc = curve25519_dalek::scalar::Scalar::from_bytes_mod_order(scalar_bytes);

        let g_i = match generator_points[i].decompress() {
            Some(pt) => pt,
            None => panic!("Invalid ristretto point decompression"),
        };

        expected_commit += ristretto_sc * g_i;
    }

    assert_eq!(commitments[0], expected_commit.compress());
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[0]);
}

#[test]
fn sending_generators_and_scalars_to_gpu_produces_correct_commitment_results() {
    // generate input table
    let data: Vec<Scalar> = vec![
        curve25519_dalek::scalar::Scalar::from_bytes_mod_order([1; 32]),
        curve25519_dalek::scalar::Scalar::from_bytes_mod_order([2; 32]),
        curve25519_dalek::scalar::Scalar::from_bytes_mod_order([3; 32]),
        curve25519_dalek::scalar::Scalar::from_bytes_mod_order([4; 32]),
    ];

    let mut rng = OsRng;

    // randomly obtain the generator points
    let generator_points: Vec<CompressedRistretto> = (0..data.len())
        .map(|_| RistrettoPoint::random(&mut rng).compress())
        .collect();
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); 1];

    compute_commitments_with_generators(&mut commitments, &[&data[..]], &generator_points);

    let mut expected_commit = match CompressedRistretto::from_slice(&[0_u8; 32]).decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression"),
    };

    for i in 0..generator_points.len() {
        let g_i = match generator_points[i].decompress() {
            Some(pt) => pt,
            None => panic!("Invalid ristretto point decompression"),
        };

        expected_commit += data[i] * g_i;
    }

    assert_eq!(commitments[0], expected_commit.compress());
    assert_ne!(CompressedRistretto::from_slice(&[0_u8; 32]), commitments[0]);
}
