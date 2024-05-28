use super::backend::init_backend;
use crate::compute::Curve;
use std::marker::PhantomData;

/// Handle to compute multi-scalar multiplications (MSMs) with pre-specified generators
pub struct MsmHandle<T: Curve> {
    handle: *mut blitzar_sys::sxt_multiexp_handle,
    phantom: PhantomData<T>,
}

impl<T: Curve> MsmHandle<T> {
    /// New handle from the specified generators.
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

    /// Compute an MSM using pre-specified generators.
    ///
    /// Suppose g_1, ..., g_n are pre-specified generators and
    ///
    ///    s_11, s_12, ..., s_1n
    ///    s_21, s_22, ..., s_2n
    ///    .
    ///    .   .
    ///    .       .
    ///    s_m1, sm2, ..., s_mn
    ///
    /// is an array of scalars of element_num_bytes each.
    ///
    /// If msm is called with the slice of scalars of size element_num_bytes * m * n
    /// defined by
    ///
    ///    scalars = [s_11, s_21, ..., s_m1, s_12, s_22, ..., s_m2, ..., s_mn ]
    ///
    /// then res will contain the MSM result
    ///
    ///    res[0] = s_11 * g_1 + s_12 * g_2 + ... + s_1n * g_n
    ///       .
    ///       .
    ///       .
    ///    res[m-1] = s_m1 * g_1 + s_12 * g_2 + ... + s_mn * g_n
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
