use super::*;

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

    // try different multiexponentiations
    let scalars: Vec<u8> = vec![1];

    handle.msm(&mut res, 1, &scalars);
    assert_eq!(res[0], generators[0]);
}
