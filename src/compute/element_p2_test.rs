use super::*;
use ark_bls12_381::G1Affine;
use ark_std::UniformRand;

#[test]
fn we_can_convert_between_different_point_representations() {
    // we handle zero
    let e1 = G1Affine::identity();
    let e2 = ElementP2::from(e1);
    let e1p = G1Affine::from(e2);
    assert_eq!(e1, e1p);

    // we handle a random point
    let mut rng = ark_std::test_rng();
    let e1 = G1Affine::rand(&mut rng);
    let e2 = ElementP2::from(e1);
    let e1p = G1Affine::from(e2);
    assert_eq!(e1, e1p);
}
