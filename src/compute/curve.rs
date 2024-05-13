use ark_bls12_381::G1Affine;
use ark_bn254::G1Affine as bn254_g1_affine;
use curve25519_dalek::ristretto::RistrettoPoint;

pub trait Curve {
    fn curve_id() -> u32;
}

impl Curve for RistrettoPoint {
    fn curve_id() -> u32 {
        blitzar_sys::SXT_CURVE_RISTRETTO255
    }
}
