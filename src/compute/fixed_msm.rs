use super::backend::init_backend;
use crate::compute::{curve::SwCurveConfig, CurveId, ElementP2};
use ark_ec::short_weierstrass::Affine;
use rayon::prelude::*;
use std::marker::PhantomData;
use std::ffi::CString;

fn count_scalars_per_output(scalars_len: usize, output_bit_table: &[u32]) -> u32 {
    let bit_sum: usize = output_bit_table.iter().map(|s| *s as usize).sum();
    let num_output_bytes = (bit_sum + 7) / 8;
    assert!(scalars_len % num_output_bytes == 0);
    (scalars_len / num_output_bytes).try_into().unwrap()
}

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

unsafe impl<T: CurveId> Send for MsmHandle<T> {}
unsafe impl<T: CurveId> Sync for MsmHandle<T> {}

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

    /// New handle from a serialized file.
    ///
    /// Note: any MSMs computed with the handle must have length less than or equal
    /// to the number of generators used to create the handle.
    pub fn new_from_file(filename: &str) -> Self {
        init_backend();
        let filename = CString::new(filename).expect("filename cannot have null bytes");
        unsafe {
            let handle = blitzar_sys::sxt_multiexp_handle_new_from_file(
                T::CURVE_ID,
                filename.as_ptr(),
            );
            Self {
                handle,
                phantom: PhantomData,
            }
        }
    }

    /// Serialize the handle to a file.
    ///
    /// This function can be used together with new_from_file to reduce
    /// the cost of creating a handle.
    pub fn write(&self, filename: &str) {
        let filename = CString::new(filename).expect("filename cannot have null bytes");
        unsafe {
            blitzar_sys::sxt_multiexp_handle_write_to_file(
                self.handle,
                filename.as_ptr(),
            );
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

    /// Compute an MSM in packed format using pre-specified generators.
    ///
    /// On completion `res` contains an array of size `num_outputs` for the multiexponentiation
    /// of the given `scalars` array.
    ///
    /// An entry output_bit_table[output_index] specifies the number of scalar bits used for
    /// output_index.
    ///
    /// Put
    ///     bit_sum = sum_{output_index} output_bit_table[output_index]
    /// and let num_bytes denote the smallest integer greater than or equal to bit_sum that is a
    /// multiple of 8.
    ///
    ///
    /// `scalars` specifies a contiguous multi-dimension `num_bytes` by `n` array laid out in
    /// a packed column-major order as specified by output_bit_table. A given row determines the scalar
    /// exponents for generator g_i with the output scalars packed contiguously and padded with zeros.
    pub fn packed_msm(&self, res: &mut [T], output_bit_table: &[u32], scalars: &[u8]) {
        let num_outputs = res.len() as u32;
        let n = count_scalars_per_output(scalars.len(), output_bit_table);
        unsafe {
            blitzar_sys::sxt_fixed_packed_multiexponentiation(
                res.as_ptr() as *mut std::ffi::c_void,
                self.handle,
                output_bit_table.as_ptr(),
                num_outputs,
                n,
                scalars.as_ptr(),
            );
        }
    }

    /// Compute a varying lengthing multiexponentiation of scalars in packed format using a handle to
    /// pre-specified generators.
    ///
    /// On completion `res` contains an array of size `num_outputs` for the multiexponentiation
    /// of the given `scalars` array.
    ///
    /// An entry output_bit_table[output_index] specifies the number of scalar bits used for
    /// output_index and output_lengths[output_index] specifies the length used for output_index.
    ///
    /// Note: output_lengths must be sorted in ascending order
    ///
    /// Put
    ///     bit_sum = sum_{output_index} output_bit_table[output_index]
    /// and let num_bytes denote the smallest integer greater than or equal to bit_sum that is a
    /// multiple of 8.
    ///
    /// Let n denote the length of the longest output. Then `scalars` specifies a contiguous
    /// multi-dimension `num_bytes` by `n` array laid out in a packed column-major order as specified by
    /// output_bit_table. A given row determines the scalar exponents for generator g_i with the output
    /// scalars packed contiguously and padded with zeros.
    pub fn vlen_msm(
        &self,
        res: &mut [T],
        output_bit_table: &[u32],
        output_lengths: &[u32],
        scalars: &[u8],
    ) {
        let num_outputs = res.len() as u32;
        assert_eq!(output_bit_table.len(), num_outputs as usize);
        assert_eq!(output_lengths.len(), num_outputs as usize);
        unsafe {
            blitzar_sys::sxt_fixed_vlen_multiexponentiation(
                res.as_ptr() as *mut std::ffi::c_void,
                self.handle,
                output_bit_table.as_ptr(),
                output_lengths.as_ptr(),
                num_outputs,
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

    /// Compute a packed MSM with the result given as affine elements
    fn affine_packed_msm(
        &self,
        res: &mut [Self::AffineElement],
        output_bit_table: &[u32],
        scalars: &[u8],
    );

    /// Compute a variable length MSM with the result given as affine elements
    fn affine_vlen_msm(
        &self,
        res: &mut [Self::AffineElement],
        output_bit_table: &[u32],
        output_lengths: &[u32],
        scalars: &[u8],
    );
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

    fn affine_packed_msm(
        &self,
        res: &mut [Self::AffineElement],
        output_bit_table: &[u32],
        scalars: &[u8],
    ) {
        let mut res_p: Vec<ElementP2<C>> = vec![ElementP2::<C>::default(); res.len()];
        self.packed_msm(&mut res_p, output_bit_table, scalars);
        res.par_iter_mut().zip(res_p).for_each(|(resi, resi_p)| {
            *resi = resi_p.into();
        });
    }

    fn affine_vlen_msm(
        &self,
        res: &mut [Self::AffineElement],
        output_bit_table: &[u32],
        output_lengths: &[u32],
        scalars: &[u8],
    ) {
        let mut res_p: Vec<ElementP2<C>> = vec![ElementP2::<C>::default(); res.len()];
        self.vlen_msm(&mut res_p, output_bit_table, output_lengths, scalars);
        res.par_iter_mut().zip(res_p).for_each(|(resi, resi_p)| {
            *resi = resi_p.into();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn we_can_count_the_number_of_scalars_per_output() {
        let output_bit_table = [1];
        let n = count_scalars_per_output(1, &output_bit_table);
        assert_eq!(n, 1);

        let output_bit_table = [14, 2];
        let n = count_scalars_per_output(10, &output_bit_table);
        assert_eq!(n, 5);

        // we handle cases that overflow
        let output_bit_table = [u32::MAX, 1];
        let n = count_scalars_per_output((u32::MAX as usize) + 1, &output_bit_table);
        assert_eq!(n, 8);
    }
}
