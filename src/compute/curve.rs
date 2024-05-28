use crate::compute::ElementP2;
use curve25519_dalek::ristretto::RistrettoPoint;

pub trait SwCurveConfig: ark_ec::short_weierstrass::SWCurveConfig {
    fn curve_id() -> u32;
}

impl SwCurveConfig for ark_bls12_381::g1::Config {
    fn curve_id() -> u32 {
        blitzar_sys::SXT_CURVE_BLS_381
    }
}

impl SwCurveConfig for ark_bn254::g1::Config {
    fn curve_id() -> u32 {
        blitzar_sys::SXT_CURVE_BN_254
    }
}

pub trait Curve {
    fn curve_id() -> u32;
}

impl Curve for RistrettoPoint {
    fn curve_id() -> u32 {
        blitzar_sys::SXT_CURVE_RISTRETTO255
    }
}

impl<C: SwCurveConfig> Curve for ElementP2<C> {
    fn curve_id() -> u32 {
        C::curve_id()
    }
}
