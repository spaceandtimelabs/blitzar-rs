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

/// Implement the `to_sxt_descriptor` method for the `&[Scalar]` and Vec<Scalar> datatype
macro_rules! impl_descriptor {
    ($tt:ty) => {
        impl Descriptor for $tt {
            fn len(&self) -> usize {
                (*self).len()
            }

            fn to_sxt_descriptor(&self) -> (usize, proofs_gpu_sys::sxt_sequence_descriptor) {
                let num_rows = (*self).len();

                let descriptor = proofs_gpu_sys::sxt_sequence_descriptor {
                    element_nbytes: 32,
                    n: num_rows as u64,
                    data: self.as_ptr() as *const u8,
                };

                (num_rows, descriptor)
            }
        }
    };
}

impl_descriptor!(&[Scalar]);
impl_descriptor!(Vec<Scalar>);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequences::DenseSequence;
    use byte_slice_cast::AsByteSlice;

    #[test]
    fn we_can_convert_a_sequence_to_a_descriptor() {
        let data = vec![123, 456, 777];
        let seq = Sequence::Dense(DenseSequence {
            data_slice: data.as_byte_slice(),
            element_size: std::mem::size_of_val(&data[0]),
        });
        let (len, descr) = seq.to_sxt_descriptor();

        assert_eq!(seq.len(), len);
        assert_eq!(descr.n, 3);
        assert_eq!(descr.element_nbytes, std::mem::size_of_val(&data[0]) as u8);
        assert_eq!(descr.data, data.as_ptr() as *const u8);
    }

    #[test]
    fn we_can_convert_a_scalar_slice_to_a_descriptor() {
        let data = vec![Scalar::default(); 3];
        let (len, descr) = (&data[..]).to_sxt_descriptor();
        assert_eq!(len, data.len());
        assert_eq!(descr.n, data.len() as u64);
        assert_eq!(descr.element_nbytes, std::mem::size_of_val(&data[0]) as u8);
        assert_eq!(descr.data, data.as_ptr() as *const u8);
    }

    #[test]
    fn we_can_convert_a_vec_scalar_to_a_descriptor() {
        let data = vec![Scalar::default(); 3];
        let (len, descr) = data.to_sxt_descriptor();
        assert_eq!(len, data.len());
        assert_eq!(descr.n, data.len() as u64);
        assert_eq!(descr.element_nbytes, std::mem::size_of_val(&data[0]) as u8);
        assert_eq!(descr.data, data.as_ptr() as *const u8);
    }

    #[test]
    fn we_can_convert_sequences_to_descriptors() {
        let data1 = vec![123, 456];
        let data2 = vec![123, 456, 777];
        let table: Vec<_> = vec![
            Sequence::Dense(DenseSequence {
                data_slice: data1.as_byte_slice(),
                element_size: std::mem::size_of_val(&data1[0]),
            }),
            Sequence::Dense(DenseSequence {
                data_slice: data2.as_byte_slice(),
                element_size: std::mem::size_of_val(&data2[0]),
            }),
        ];

        let (descr, descr_len) = to_sxt_descriptors(&table[..]);
        assert_eq!(descr_len, 3);
        assert_eq!(descr.len(), 2);
    }

    #[test]
    fn we_can_convert_scalar_slices_to_descriptors() {
        let table: Vec<_> = vec![Scalar::default(), Scalar::default()];
        let (descr, descr_len) = to_sxt_descriptors(&[&table[..], &table[..]]);
        assert_eq!(descr_len, 2);
        assert_eq!(descr.len(), 2);
    }

    #[test]
    fn we_can_convert_scalar_vec_to_descriptors() {
        let table1: Vec<_> = vec![Scalar::default(); 4];
        let table2: Vec<_> = vec![Scalar::default(); 3];
        let (descr, descr_len) = to_sxt_descriptors(&[table1, table2]);
        assert_eq!(descr_len, 4);
        assert_eq!(descr.len(), 2);
    }
}
