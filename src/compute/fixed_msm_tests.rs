use super::*;

use ark_std::UniformRand;
use ark_serialize::CanonicalSerialize;
use ark_bls12_381::{G1Projective};
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;

#[test]
fn we_can_compute_msms_using_a_single_generator() {
    let mut rng = OsRng;

    let mut res = vec![RistrettoPoint::default(); 1];

    // randomly obtain the generator points
    let generators: Vec<RistrettoPoint> = (0..1)
        .map(|_| RistrettoPoint::random(&mut rng))
        .collect();

    // create handle
    let handle = MsmHandle::new(&generators);

    // 1 * g
    let scalars: Vec<u8> = vec![1];
    handle.msm(&mut res, 1, &scalars);
    assert_eq!(res[0], generators[0]);

    // 2 * g
    let scalars: Vec<u8> = vec![2];
    handle.msm(&mut res, 1, &scalars);
    assert_eq!(res[0], generators[0] + generators[0]);
}

/*
#[test]
fn we_can_compute_msms_using_a_single_generator_bls12_381() {
    let mut rng = ark_std::test_rng();

    let mut res = vec![G1Projective::default(); 1];

    // randomly obtain the generator points
    let generators: Vec<G1Projective> =
        (0..1).map(|_| G1Projective::rand(&mut rng)).collect();
    println!("g = {}", generators[0]);

    // create handle
    let handle = MsmHandle::new(&generators);

    // 1 * g
    let scalars: Vec<u8> = vec![1];
    println!("E size = {}", std::mem::size_of::<G1Projective>());
    println!("res = {}", res[0]);
    handle.msm(&mut res, 1, &scalars);
    println!("res = {}", res[0]);

    // let mut bytes1 = Vec::new();
    // res[0]
    //     .serialize_compressed(&mut bytes1)
    //     .unwrap();
    //
    // let mut bytes2 = Vec::new();
    // generators[0]
    //     .serialize_compressed(&mut bytes2)
    //     .unwrap();

    // assert_eq!(bytes1, bytes2);
    assert_eq!(res[0], generators[0]);

    // 2 * g
    // let scalars: Vec<u8> = vec![2];
    // handle.msm(&mut res, 1, &scalars);
    // assert_eq!(res[0], generators[0] + generators[0]);
}*/
