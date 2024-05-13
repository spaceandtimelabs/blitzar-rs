// use super::backend::init_backend;
use crate::compute::Curve;
use std::marker::PhantomData;

/// TODO(rnburn): document me
pub struct MsmHandle<T: Curve> {
    phantom: PhantomData<T>,
}

impl<T: Curve> MsmHandle<T> {
    fn new(generators: &[T] ) -> Self {
        Self{
            phantom: PhantomData,
        }
    }
}
