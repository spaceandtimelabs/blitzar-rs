use ark_bls12_381::G1Projective;
use ark_bn254::G1Projective as bn254_g1_projective;
use ark_ec::CurveGroup;
use curve25519_dalek::ristretto::RistrettoPoint;

pub trait Curve {
    fn curve_id() -> u32;
}

impl Curve for RistrettoPoint {
    fn curve_id() -> u32 {
        blitzar_sys::SXT_CURVE_RISTRETTO255
    }
}

impl Curve for G1Projective {
    fn curve_id() -> u32 {
        blitzar_sys::SXT_CURVE_BLS_381
    }
}

// impl Curve for bn254_g1_projective {
//     fn curve_id() -> u32 {
//         blitzar_sys::SXT_CURVE_BN_254
//     }
// }
