use crate::compute::ElementP2;
use curve25519_dalek::ristretto::RistrettoPoint;

pub trait SwCurveConfig: ark_ec::short_weierstrass::SWCurveConfig {
    const CURVE_ID: u32;
}

impl SwCurveConfig for ark_bls12_381::g1::Config {
    const CURVE_ID: u32 = blitzar_sys::SXT_CURVE_BLS_381;
}

impl SwCurveConfig for ark_bn254::g1::Config {
    const CURVE_ID: u32 = blitzar_sys::SXT_CURVE_BN_254;
}

pub trait CurveId {
    const CURVE_ID: u32;
}

impl CurveId for RistrettoPoint {
    const CURVE_ID: u32 = blitzar_sys::SXT_CURVE_RISTRETTO255;
}

impl<C: SwCurveConfig> CurveId for ElementP2<C> {
    const CURVE_ID: u32 = C::CURVE_ID;
}
