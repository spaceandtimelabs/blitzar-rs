// use super::backend::init_backend;
use crate::compute::Curve;
use std::marker::PhantomData;

/// TODO(rnburn): document me
pub struct MsmHandle<T: Curve> {
    handle: *mut blitzar_sys::sxt_multiexp_handle,
    phantom: PhantomData<T>,
}

impl<T: Curve> MsmHandle<T> {
    /// TODO(rnburn): document me
    pub fn new(generators: &[T] ) -> Self {
        unsafe {
          let handle =
            blitzar_sys::sxt_multiexp_handle_new(
                T::curve_id(),
                generators.as_ptr() as *const std::ffi::c_void,
                generators.len() as u32);
            Self{
                handle: handle,
                phantom: PhantomData,
            }
        }
    }
}

impl<T: Curve> Drop for MsmHandle<T> {
    fn drop(&mut self) {
        unsafe {
          blitzar_sys::sxt_multiexp_handle_free(self.handle);
        }
    }
}
