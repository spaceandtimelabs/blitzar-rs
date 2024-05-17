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
        println!("curve_id = {}", T::curve_id());
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
                num_outputs as u32,
                n as u32,
                scalars.as_ptr()
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
