use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ff::fields::Field;
use ark_std::{One, Zero};
use std::convert::From;

/// Projective form for a short Weierstrass curve element.
///
/// A point (x, y, z) represents the affine point (x / z, y / z) or
/// the identity if z == 0
#[derive(Clone)]
pub struct ElementP2<P: SWCurveConfig> {
    /// (x, z) maps to the affine point x / z
    pub x: P::BaseField,

    /// (y, z) maps to the affine point y / z
    pub y: P::BaseField,

    /// divisor to convert to affine points
    ///
    /// if z == 0, the point represents the identity element
    pub z: P::BaseField,
}

impl<P: SWCurveConfig> Default for ElementP2<P> {
    fn default() -> Self {
        Self {
            x: P::BaseField::zero(),
            y: P::BaseField::zero(),
            z: P::BaseField::zero(),
        }
    }
}

impl<P: SWCurveConfig> From<Affine<P>> for ElementP2<P> {
    fn from(pt: Affine<P>) -> Self {
        Self {
            x: pt.x,
            y: pt.y,
            z: if pt.infinity {
                P::BaseField::zero()
            } else {
                P::BaseField::one()
            },
        }
    }
}

impl<P: SWCurveConfig> From<&Affine<P>> for ElementP2<P> {
    fn from(pt: &Affine<P>) -> Self {
        Self {
            x: pt.x,
            y: pt.y,
            z: if pt.infinity {
                P::BaseField::zero()
            } else {
                P::BaseField::one()
            },
        }
    }
}

impl<P: SWCurveConfig> From<ElementP2<P>> for Affine<P> {
    fn from(pt: ElementP2<P>) -> Self {
        if pt.z.is_zero() {
            return Affine::<P> {
                x: P::BaseField::zero(),
                y: P::BaseField::zero(),
                infinity: true,
            };
        }
        let z_inv = pt.z.inverse().unwrap();
        Affine::<P> {
            x: pt.x * z_inv,
            y: pt.y * z_inv,
            infinity: false,
        }
    }
}
