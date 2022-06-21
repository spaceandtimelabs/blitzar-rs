// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

//! Wrappers for data table

use std::ptr;

/// This `DenseSequence` stores the slice view
/// of a contiguous column data table.
/// It doesn't matter how the data is represented.
/// The only thing that matters is that
/// the column table is contiguous in memory,
/// and that the structure holding the memory
/// can be represented as a slice view
/// of u8 elements. Vectors of primitive types
/// automatically can be cast to this kind of 
/// slice view, by using `byte-slice-cast` trait extension.
pub struct DenseSequence<'a> {
    /// Represents a slice
    /// view of any region of memory
    /// converted to a u8 slice view.
    ///
    /// For doing the conversion from
    /// an arbitrary slice array to a
    /// u8 slice array, we use the `byte-slice-cast`
    /// dependency, which implements the traits `as_byte_slice`.
    /// To do this conversion, you can follow the example below:
    ///
    /// ```
    /// use byte_slice_cast::AsByteSlice;
    /// let data: Vec<u16> = vec![2000, 7500, 5000, 1500];
    /// let data_slice = &data.as_byte_slice();
    /// ```
    ///
    /// Be careful when using signed data, since we cast everything to
    /// unsigned bytes.
    pub data_slice: &'a [u8],

    /// Represents the total number of 
    /// bytes of each element encoded in the
    /// `data_slice` view
    ///
    /// During computations, this field is used to infer the total
    /// amount of `rows` in the `data_slice`, using for that the formula:
    ///
    /// ```text
    /// let num_rows = curr_data.data_slice.len() / curr_data.element_size;
    /// ```
    /// So be cautious that the number of bytes captured by the `data_slice`
    /// and the `element_size` value are properly defined. For instance, 
    /// you must secure that for:
    ///
    /// - u8 data slice types: element_size = std::mem::size_of::<u8>()
    /// - u16 data slice types: element_size = std::mem::size_of::<u16>()
    /// - u32 data slice types: element_size = std::mem::size_of::<u32>()
    /// - u64 data slice types: element_size = std::mem::size_of::<u64>()
    /// - u128 data slice types: element_size = std::mem::size_of::<u128>()
    pub element_size: usize
}

impl<'a> DenseSequence<'_> {
    pub(super) fn to_data_properties(& self) -> (u8, usize,  *const u8,  *const u64) {
        if (*self).data_slice.len() % (*self).element_size != 0 {
            panic!("Error: data_slice length is not a multiple of element_size in the dense object");
        }
        
        let num_rows = (*self).data_slice.len() / (*self).element_size;

        ((*self).element_size as u8, num_rows as usize, (*self).data_slice.as_ptr(), ptr::null())
    }
}

///
pub struct SparseSequence<'a> {
    ///
    pub data_slice: &'a [u8],

    ///
    pub element_size: usize,

    ///
    pub data_indices: &'a [u64]
}

impl<'a> SparseSequence<'_> {
    pub(super) fn to_data_properties(& self) -> (u8, usize,  *const u8,  *const u64) {
        if (*self).data_slice.len() % (*self).element_size != 0 {
            panic!("Error: data_slice length is not a multiple of element_size in the sparse object");
        }
        
        let num_rows = (*self).data_slice.len() / (*self).element_size;

        if num_rows != (*self).data_indices.len() {
            panic!("Error: Number of rows differs from the data_indices length in the sparse object");
        }

        ((*self).element_size as u8, num_rows as usize, (*self).data_slice.as_ptr(), (*self).data_indices.as_ptr())
    }
}

/// Defines multiple wrappers so that all of them
/// can be stored in the same vector array.
/// We currently only support `Dense` structures,
/// but in the future, we intend to add `Sparse` structures
/// to our code too.
///
/// To use this sequence, you can have:
///
/// ```
/// use byte_slice_cast::AsByteSlice;
/// let mut table: Vec<pedersen::sequence::Sequence> = Vec::new();
/// let data_a: Vec<u16> = vec![2000, 7500, 5000, 1500];
///
/// table.push(pedersen::sequence::Sequence::Dense(pedersen::sequence::DenseSequence {
///     data_slice: &data_a.as_byte_slice(),
///     element_size: std::mem::size_of::<u16>()
/// }));
/// ```
pub enum Sequence<'a> {
    /// A simple enum wrapper to a DenseSequence structure
    Dense(DenseSequence<'a>),
    ///
    Sparse(SparseSequence<'a>)
}
