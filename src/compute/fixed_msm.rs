// use super::backend::init_backend;
use super::backend::init_backend;
use crate::compute::Curve;
use std::marker::PhantomData;

/// Handle to compute multi-scalar multiplications (MSMs) with pre-specified generators
pub struct MsmHandle<T: Curve> {
    handle: *mut blitzar_sys::sxt_multiexp_handle,
    phantom: PhantomData<T>,
}

impl<T: Curve> MsmHandle<T> {
    /// new handle from the specified generators.
    ///
    /// Note: any MSMs computed with the handle must have length less than or equal
    /// to the number of generators used to create the handle.
    pub fn new(generators: &[T]) -> Self {
        init_backend();

        unsafe {
            let handle = blitzar_sys::sxt_multiexp_handle_new(
                T::curve_id(),
                generators.as_ptr() as *const std::ffi::c_void,
                generators.len() as u32,
            );
            Self {
                handle,
                phantom: PhantomData,
            }
        }
    }

    /// TODO(rnburn): document me
    pub fn msm(&self, res: &mut [T], element_num_bytes: u32, scalars: &[u8]) {
        let num_outputs = res.len() as u32;
        assert!(scalars.len() as u32 % (num_outputs * element_num_bytes) == 0);
        let n = scalars.len() as u32 / (num_outputs * element_num_bytes);
        unsafe {
            blitzar_sys::sxt_fixed_multiexponentiation(
                res.as_ptr() as *mut std::ffi::c_void,
                self.handle,
                element_num_bytes,
                num_outputs,
                n,
                scalars.as_ptr(),
            );
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
