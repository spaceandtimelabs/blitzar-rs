use crate::compute::Curve;
use std::marker::PhantomData;

pub struct SwMsmHandle<T: Curve> {
    phantom: PhantomData<T>,
}

