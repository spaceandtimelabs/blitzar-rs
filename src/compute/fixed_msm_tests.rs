use super::*;

use crate::compute::ElementP2;
use ark_bls12_381::G1Affine;
use ark_std::UniformRand;
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;

#[test]
fn we_can_compute_msms_using_a_single_generator() {
    let mut rng = OsRng;

    let mut res = vec![RistrettoPoint::default(); 1];

    // randomly obtain the generator points
    let generators: Vec<RistrettoPoint> =
        (0..1).map(|_| RistrettoPoint::random(&mut rng)).collect();

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

#[test]
fn we_can_compute_msms_using_a_single_generator_bls12_381() {
    let mut rng = ark_std::test_rng();

    let mut res = vec![ElementP2::<ark_bls12_381::g1::Config>::default(); 1];

    // randomly obtain the generator points
    let generators: Vec<ElementP2<ark_bls12_381::g1::Config>> =
        (0..1).map(|_| G1Affine::rand(&mut rng).into()).collect();

    let g: G1Affine = generators[0].clone().into();
    // println!("g = {}", generators[0]);

    // create handle
    let handle = MsmHandle::new(&generators);

    // 1 * g
    let scalars: Vec<u8> = vec![1];
    handle.msm(&mut res, 1, &scalars);
    let r: G1Affine = res[0].clone().into();
    assert_eq!(r, g);

    // 2 * g
    let scalars: Vec<u8> = vec![2];
    handle.msm(&mut res, 1, &scalars);
    let r: G1Affine = res[0].clone().into();
    assert_eq!(r, g + g);
}
