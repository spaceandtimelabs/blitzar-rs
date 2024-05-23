use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ff::fields::Field;
use ark_std::{One, Zero};
use std::convert::From;

/// TODO(rnburn): doc me
#[derive(Clone)]
pub struct ElementP2<P: SWCurveConfig> {
    /// TODO(rnburn): doc me
    pub x: P::BaseField,

    /// TODO(rnburn): doc me
    pub y: P::BaseField,

    /// TODO(rnburn): doc me
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
