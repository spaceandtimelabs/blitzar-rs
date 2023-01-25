// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

use super::Sequence;
use curve25519_dalek::scalar::Scalar;
use std::cmp;

/// This descriptor is only used to implement the `to_sxt_descriptor` method
/// for the `Sequence<'a>` and the `&[Scalar]` elements
pub trait Descriptor {
    /// returns the number of elements referenced by the descriptor
    fn len(&self) -> usize;

    /// converts the descriptor to a sxt_descriptor
    fn to_sxt_descriptor(&self) -> (usize, proofs_gpu_sys::sxt_sequence_descriptor);
}

/// Implement the `to_sxt_descriptor` method for the `Sequence<'a>` datatype
impl<'a> Descriptor for Sequence<'a> {
    fn len(&self) -> usize {
        match self {
            Sequence::Dense(x) => x.len(),
        }
    }

    fn to_sxt_descriptor(&self) -> (usize, proofs_gpu_sys::sxt_sequence_descriptor) {
        let (element_nbytes, num_rows, data) = match self {
            Sequence::Dense(x) => x.to_data_properties(),
        };

        let descriptor = proofs_gpu_sys::sxt_sequence_descriptor {
            element_nbytes,
            n: num_rows as u64,
            data,
        };

        (num_rows, descriptor)
    }
}

/// Implement the `to_sxt_descriptor` method for the `&[Scalar]` datatype
impl Descriptor for &[Scalar] {
    fn len(&self) -> usize {
        (*self).len()
    }

    fn to_sxt_descriptor(&self) -> (usize, proofs_gpu_sys::sxt_sequence_descriptor) {
        let num_rows = (*self).len();

        let descriptor = proofs_gpu_sys::sxt_sequence_descriptor {
            element_nbytes: 32,
            n: num_rows as u64,
            data: (*self).as_ptr() as *const u8,
        };

        (num_rows, descriptor)
    }
}

/// `to_sxt_descriptors` converts the data table from the
/// sequence slice to the lower-level sys crate
/// `sxt_sequence_descriptor` vector struct.
///
/// # Errors
///
/// If some data_slice has a `data_slice.len()` that is not a multiple of `element_size`
pub fn to_sxt_descriptors<T: Descriptor>(
    data: &[T],
) -> (Vec<proofs_gpu_sys::sxt_sequence_descriptor>, usize) {
    let mut longest_row: usize = 0;
    let num_sequences = data.len();
    let cbinding_descriptors: Vec<proofs_gpu_sys::sxt_sequence_descriptor> = (0..num_sequences)
        .map(|i| {
            let (num_rows, descriptor) = data[i].to_sxt_descriptor();

            longest_row = cmp::max(longest_row, num_rows);

            descriptor
        })
        .collect();

    (cbinding_descriptors, longest_row)
}
