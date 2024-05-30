use super::backend::init_backend;
use crate::compute::curve::SwCurveConfig;
use crate::compute::{CurveId, ElementP2};
use ark_ec::short_weierstrass::Affine;
use rayon::prelude::*;
use std::marker::PhantomData;

/// Handle to compute multi-scalar multiplications (MSMs) with pre-specified generators
///
/// # Example 1 - compute an MSM using the handle
///```no_run
#[doc = include_str!("../../examples/simple_fixed_msm.rs")]
///```
pub struct MsmHandle<T: CurveId> {
    handle: *mut blitzar_sys::sxt_multiexp_handle,
    phantom: PhantomData<T>,
}

impl<T: CurveId> MsmHandle<T> {
    /// New handle from the specified generators.
    ///
    /// Note: any MSMs computed with the handle must have length less than or equal
    /// to the number of generators used to create the handle.
    pub fn new(generators: &[T]) -> Self {
        init_backend();

        unsafe {
            let handle = blitzar_sys::sxt_multiexp_handle_new(
                T::CURVE_ID,
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

impl<T: CurveId> Drop for MsmHandle<T> {
    fn drop(&mut self) {
        unsafe {
            blitzar_sys::sxt_multiexp_handle_free(self.handle);
        }
    }
}

/// Extend MsmHandle to work with affine coordinates for short Weierstrass curve elements
pub trait SwMsmHandle {
    /// Type of an Affine curve element
    type AffineElement;

    /// Create a handle from affine generators
    fn new_with_affine(generators: &[Self::AffineElement]) -> Self;

    /// Compute a MSM with the result given as affine elements
    fn affine_msm(&self, res: &mut [Self::AffineElement], element_num_bytes: u32, scalars: &[u8]);
}

impl<C: SwCurveConfig + Clone> SwMsmHandle for MsmHandle<ElementP2<C>> {
    type AffineElement = Affine<C>;

    fn new_with_affine(generators: &[Self::AffineElement]) -> Self {
        let generators: Vec<ElementP2<C>> = generators.iter().map(|e| e.into()).collect();
        MsmHandle::new(&generators)
    }

    fn affine_msm(&self, res: &mut [Self::AffineElement], element_num_bytes: u32, scalars: &[u8]) {
        let mut res_p: Vec<ElementP2<C>> = vec![ElementP2::<C>::default(); res.len()];
        self.msm(&mut res_p, element_num_bytes, scalars);
        res.par_iter_mut().zip(res_p).for_each(|(resi, resi_p)| {
            *resi = resi_p.into();
        });
    }
}
