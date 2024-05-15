use ark_ec::short_weierstrass::Projective;
use curve25519_dalek::ristretto::RistrettoPoint;

pub trait Curve {
    fn curve_id() -> u32;
}

impl Curve for RistrettoPoint {
    fn curve_id() -> u32 {
        blitzar_sys::SXT_CURVE_RISTRETTO255
    }
}

impl Curve for Projective<ark_bls12_381::g1::Config> {
    fn curve_id() -> u32 {
        blitzar_sys::SXT_CURVE_BLS_381
    }
}

impl Curve for Projective<ark_bn254::g1::Config> {
    fn curve_id() -> u32 {
        blitzar_sys::SXT_CURVE_BN_254
    }
}
