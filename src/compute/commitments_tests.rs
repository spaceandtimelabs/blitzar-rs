// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>
// - Ian Joiner <ian.joiner@spaceandtime.io>

use super::*;
use crate::sequences::{DenseSequence, Sequence};
use byte_slice_cast::AsByteSlice;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;

#[test]
fn we_can_compute_commitments_with_a_zero_offset() {
    // generate input table
    let offset_generators = 0_u64;
    let data: Vec<u32> = vec![2000, 7500, 5000, 1500];
    let mut commitments = vec![CompressedRistretto::default(); 1];
    let mut generators = vec![RistrettoPoint::default(); data.len()];
    get_generators(&mut generators, offset_generators);

    compute_commitments(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        })],
        offset_generators,
    );

    let expected_commit = data
        .iter()
        .zip(generators.iter())
        .map(|(x, y)| Scalar::from(*x) * y)
        .sum::<RistrettoPoint>()
        .compress();

    // verify if commitment results are correct
    assert_eq!(commitments[0], expected_commit);
    assert_ne!(CompressedRistretto::default(), commitments[0]);
}

#[test]
fn we_can_compute_commitments_with_a_non_zero_offset() {
    // generate input table
    let offset_generators = 121_u64;
    let data: Vec<u32> = vec![2000, 7500, 5000, 1500];
    let mut commitments = vec![CompressedRistretto::default(); 1];
    let mut generators = vec![RistrettoPoint::default(); data.len()];
    get_generators(&mut generators, offset_generators);

    compute_commitments(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        })],
        offset_generators,
    );

    let expected_commit = data
        .iter()
        .zip(generators.iter())
        .map(|(x, y)| Scalar::from(*x) * y)
        .sum::<RistrettoPoint>()
        .compress();

    // verify if commitment results are correct
    assert_eq!(commitments[0], expected_commit);
    assert_ne!(CompressedRistretto::default(), commitments[0]);
}

#[test]
fn we_can_update_commitments() {
    // generate input table
    let offset_generators = 0_u64;
    let dense_data: Vec<u32> = vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0];
    let scalar_data: Vec<Scalar> = vec![Scalar::from(5000_u32), Scalar::from(1500_u32)];
    let expected_data: Vec<u32> = vec![1, 0, 5002, 1500, 3, 4, 0, 0, 0, 9, 0];
    let sliced_scalar_data: Vec<_> = vec![scalar_data.as_slice(); 1];

    let mut commitments = vec![CompressedRistretto::default(); 1];
    let mut expected_commitments = vec![CompressedRistretto::default(); 1];

    update_commitments(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: dense_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&dense_data[0]),
        })],
        0_u64,
    );

    update_commitments(&mut commitments, &sliced_scalar_data, 2_u64);

    compute_commitments(
        &mut expected_commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: expected_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&expected_data[0]),
        })],
        offset_generators,
    );

    // verify if commitment results are correct
    assert_eq!(commitments, expected_commitments);
    assert_ne!(CompressedRistretto::default(), commitments[0]);
    assert_ne!(CompressedRistretto::default(), expected_commitments[0]);
}

#[test]
fn we_can_update_multiple_commitments() {
    // generate input table
    let offset_generators = 0_u64;
    let dense_data: Vec<Vec<u32>> = vec![
        vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0],
        vec![1, 4, 3, 9, 3, 3, 4, 7, 1232, 32, 32],
    ];
    let scalar_data: Vec<Vec<Scalar>> = vec![
        vec![Scalar::from(5000_u32), Scalar::from(1500_u32)],
        vec![Scalar::from(3000_u32)],
    ];
    let expected_data: Vec<Vec<u32>> = vec![
        vec![1, 0, 2, 0, 3, 5004, 1500, 0, 0, 9, 0],
        vec![1, 4, 3, 9, 3, 3003, 4, 7, 1232, 32, 32],
    ];
    let sliced_scalar_data: Vec<_> = scalar_data.iter().map(|v| v.as_slice()).collect();

    let mut commitments = vec![CompressedRistretto::default(); 2];
    let mut expected_commitments = vec![CompressedRistretto::default(); 2];

    let dense_data_as_sequences: Vec<_> = dense_data
        .iter()
        .map(|v| {
            Sequence::Dense(DenseSequence {
                data_slice: v.as_byte_slice(),
                element_size: std::mem::size_of_val(&v[0]),
            })
        })
        .collect();

    let expected_data_as_sequences: Vec<_> = expected_data
        .iter()
        .map(|v| {
            Sequence::Dense(DenseSequence {
                data_slice: v.as_byte_slice(),
                element_size: std::mem::size_of_val(&v[0]),
            })
        })
        .collect();

    update_commitments(&mut commitments, &dense_data_as_sequences, 0_u64);

    update_commitments(&mut commitments, &sliced_scalar_data, 5_u64);

    compute_commitments(
        &mut expected_commitments,
        &expected_data_as_sequences,
        offset_generators,
    );

    // verify if commitment results are correct
    assert_eq!(commitments, expected_commitments);
    // If the two vectors are equal we only need to verify that one doesn't contain the default
    assert!(commitments
        .iter()
        .all(|&c| c != CompressedRistretto::default()));
}

#[test]
fn compute_commitments_with_scalars_works() {
    // generate input table
    let offset_generators = 0_u64;
    let mut data: Vec<Scalar> = vec![Scalar::zero(); 4];
    let mut generators = vec![RistrettoPoint::default(); data.len()];
    get_generators(&mut generators, offset_generators);

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

    let mut commitments = vec![CompressedRistretto::default(); 1];

    compute_commitments(&mut commitments, &[&data[..]], offset_generators);

    let expected_commit = data
        .iter()
        .zip(generators.iter())
        .map(|(x, y)| *x * y)
        .sum::<RistrettoPoint>()
        .compress();

    // verify if commitment results are correct
    assert_eq!(commitments[0], expected_commit);
    assert_ne!(CompressedRistretto::default(), commitments[0]);
}

#[test]
fn commit_a_plus_commit_b_equal_to_commit_c() {
    // generate input table
    let offset_generators = 0_u64;
    let data_a: Vec<u16> = vec![2000, 7500, 5000, 1500];
    let data_b: Vec<u32> = vec![5000, 0, 400000, 10, 0, 0];
    let data_c: Vec<u64> = vec![2000 + 5000, 7500, 5000 + 400000, 1500 + 10];

    let mut commitments = vec![CompressedRistretto::default(); 3];

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
        offset_generators,
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
    assert_ne!(CompressedRistretto::default(), commit_c);
}

#[test]
fn commit_1_plus_commit_1_plus_commit_1_equal_to_commit_3() {
    // generate input table
    let offset_generators = 0_u64;
    let data_a: Vec<u16> = vec![1];
    let data_b: Vec<u32> = vec![1];
    let data_c: Vec<u64> = vec![1];
    let data_d: Vec<u64> = vec![3];

    let mut commitments = vec![CompressedRistretto::default(); 4];

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
        offset_generators,
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
    assert_ne!(CompressedRistretto::default(), commit_d);
}

#[test]
fn commit_a_times_52_plus_commit_b_equal_to_commit_c() {
    // generate input table
    let scal: u64 = 52;
    let offset_generators = 0_u64;
    let data_a: Vec<u16> = vec![2000, 7500, 5000, 1500];
    let data_b: Vec<u32> = vec![5000, 0, 400000, 10, 0, 0];
    let data_c: Vec<u64> = vec![
        scal * 2000 + 5000,
        scal * 7500,
        scal * 5000 + 400000,
        scal * 1500 + 10,
    ];

    let mut commitments = vec![CompressedRistretto::default(); 3];

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
        offset_generators,
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
    scalar_bytes[0] = scal as u8;

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
    assert_ne!(CompressedRistretto::default(), commit_c);
}

#[test]
fn commit_negative_a_plus_commit_negative_b_equal_to_commit_c() {
    // generate input table
    let a: i8 = -128;
    let b: i8 = -128;
    let offset_generators = 0_u64;
    let data_a: Vec<u16> = vec![a as u16];
    let data_b: Vec<u16> = vec![b as u16];
    let data_c: Vec<u32> = vec![130816];

    let mut commitments = vec![CompressedRistretto::default(); 3];

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
        offset_generators,
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
    assert_ne!(CompressedRistretto::default(), commit_c);
}

#[test]
fn different_word_size_and_rows_in_commit_a_plus_commit_b_plus_commit_c_equal_to_commit_d() {
    // generate input table
    let offset_generators = 0_u64;
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

    let mut commitments = vec![CompressedRistretto::default(); 4];

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
        offset_generators,
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
    assert_ne!(CompressedRistretto::default(), commit_d);
}

#[test]
fn sending_generators_to_gpu_produces_correct_commitment_results() {
    // generate input table
    let data: Vec<u64> = vec![2, 3, 1, 5, 4, 7, 6, 8, 9, 10];

    let mut rng = OsRng;

    // randomly obtain the generator points
    let generator_points: Vec<RistrettoPoint> = (0..data.len())
        .map(|_| RistrettoPoint::random(&mut rng))
        .collect();
    let mut commitments = vec![CompressedRistretto::default(); 1];

    compute_commitments_with_generators(
        &mut commitments,
        &[Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        })],
        &generator_points,
    );

    let mut expected_commit = RistrettoPoint::from_uniform_bytes(&[0_u8; 64]);

    for i in 0..generator_points.len() {
        let mut scalar_bytes: [u8; 32] = [0; 32];
        scalar_bytes[0] = data[i] as u8;

        // Construct a Scalar by reducing a 256-bit little-endian integer modulo the group order ℓ.
        let ristretto_sc = curve25519_dalek::scalar::Scalar::from_bytes_mod_order(scalar_bytes);

        let g_i = generator_points[i];

        expected_commit += ristretto_sc * g_i;
    }

    assert_eq!(commitments[0], expected_commit.compress());
    assert_ne!(CompressedRistretto::default(), commitments[0]);
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
    let generators: Vec<RistrettoPoint> = (0..data.len())
        .map(|_| RistrettoPoint::random(&mut rng))
        .collect();
    let mut commitments = vec![CompressedRistretto::default(); 1];

    compute_commitments_with_generators(&mut commitments, &[&data[..]], &generators);

    let expected_commit = data
        .iter()
        .zip(generators.iter())
        .map(|(x, y)| *x * y)
        .sum::<RistrettoPoint>()
        .compress();

    assert_eq!(commitments[0], expected_commit);
    assert_ne!(CompressedRistretto::default(), commitments[0]);
}
