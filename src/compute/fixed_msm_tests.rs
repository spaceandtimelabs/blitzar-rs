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
fn we_can_compute_msms_using_multiple_generator() {
    let mut rng = OsRng;

    let mut res = vec![RistrettoPoint::default(); 1];

    // randomly obtain the generator points
    let generators: Vec<RistrettoPoint> =
        (0..2).map(|_| RistrettoPoint::random(&mut rng)).collect();

    // create handle
    let handle = MsmHandle::new(&generators);

    // g[0] + 2 * g[1]
    let scalars: Vec<u8> = vec![1, 2];
    handle.msm(&mut res, 1, &scalars);
    assert_eq!(res[0], generators[0] + generators[1] + generators[1]);
}

#[test]
fn we_can_compute_msms_using_multiple_outputs() {
    let mut rng = OsRng;

    let mut res = vec![RistrettoPoint::default(); 2];

    // randomly obtain the generator points
    let generators: Vec<RistrettoPoint> =
        (0..2).map(|_| RistrettoPoint::random(&mut rng)).collect();

    // create handle
    let handle = MsmHandle::new(&generators);

    // g[0] + 2 * g[1]
    // 3 * g[0] + g[1]
    let scalars: Vec<u8> = vec![1, 3, 2, 1];
    handle.msm(&mut res, 1, &scalars);
    assert_eq!(res[0], generators[0] + generators[1] + generators[1]);
    assert_eq!(
        res[1],
        generators[0] + generators[0] + generators[0] + generators[1]
    );
}

#[test]
fn we_can_compute_msms_using_a_single_generator_bls12_381() {
    let mut rng = ark_std::test_rng();

    let mut res = vec![ElementP2::<ark_bls12_381::g1::Config>::default(); 1];

    // randomly obtain the generator points
    let generators: Vec<ElementP2<ark_bls12_381::g1::Config>> =
        (0..1).map(|_| G1Affine::rand(&mut rng).into()).collect();

    let g: G1Affine = generators[0].clone().into();

    // create handle
    let handle = MsmHandle::new(&generators);

    // 2 * g
    let scalars: Vec<u8> = vec![2];
    handle.msm(&mut res, 1, &scalars);
    let r: G1Affine = res[0].clone().into();
    assert_eq!(r, g + g);
}

#[test]
fn for_short_weierstrass_curvs_we_can_compute_msms_with_affine_elements() {
    let mut rng = ark_std::test_rng();

    let mut res = vec![G1Affine::default(); 1];

    // randomly obtain the generator points
    let generators: Vec<G1Affine> = (0..1).map(|_| G1Affine::rand(&mut rng)).collect();

    let g = generators[0];

    // create handle
    let handle: MsmHandle<ElementP2<ark_bls12_381::g1::Config>> =
        MsmHandle::new_with_affine(&generators);

    // 2 * g
    let scalars: Vec<u8> = vec![2];
    handle.affine_msm(&mut res, 1, &scalars);
    assert_eq!(res[0], g + g);
}
