use std::convert::From;
use ark_ec::short_weierstrass::{Affine, SWCurveConfig, Projective};
use ark_std::{One, Zero};

/// TODO(rnburn): doc me
pub struct ElementP2<P: SWCurveConfig> {
    /// TODO(rnburn): doc me
    pub x: P::BaseField,

    /// TODO(rnburn): doc me
    pub y: P::BaseField,

    /// TODO(rnburn): doc me
    pub z: P::BaseField,
}

impl<P: SWCurveConfig> From<Projective<P>> for ElementP2<P> {
    fn from(pt: Projective<P>) -> Self {
        let pt = Affine::<P>::from(pt);
        Self{
            x: pt.x,
            y: pt.y,
            z: if pt.infinity { P::BaseField::zero() } else { P::BaseField::one() },
        }
    }
}
