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

/// This `SparseSequence` stores the slice view
/// of a sparse column data table.
/// The structure holding the memory
/// must be represented as a sliced view
/// of u8 elements. Vectors of primitive types
/// automatically can be cast to this kind of 
/// slice view, by using `byte-slice-cast` trait extension.
/// This structure is relevant in case the column data table
/// has many zeros. In this case, the sequence will
/// capture only non-zero elements. In order to keep
/// track of the rows associated with each non-zero element,
/// the field `data_indices` is used to hold
/// the row index associated with each data element.
pub struct SparseSequence<'a> {
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
    pub element_size: usize,

    /// `indices[i]` holds the actual row_i in which data\[i]
    /// is tied with. These `indices` must capture
    /// exactly `num_rows` elements.
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
/// We currently support `Dense` and `Sparse` structures.
///
/// To use this sequence, you can have:
///
/// ```
/// use pedersen::sequence::*;
/// use byte_slice_cast::AsByteSlice;
///
/// let sparse_data: Vec<u32> = vec![1, 2, 3, 4, 9];
/// let sparse_indices: Vec<u64> = vec![0, 2, 4, 5, 9];
/// let dense_data: Vec<u32> = vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0];
/// let mut table: Vec<Sequence> = Vec::new();
///
/// table.push(Sequence::Dense(DenseSequence {
///     data_slice: &dense_data.as_byte_slice(),
///     element_size: std::mem::size_of_val(&dense_data[0])
/// }));
///
/// table.push(Sequence::Sparse(SparseSequence {
///     data_slice: &sparse_data.as_byte_slice(),
///     element_size: std::mem::size_of_val(&sparse_data[0]),
///     data_indices: &sparse_indices
/// }));
/// ```
pub enum Sequence<'a> {
    /// A simple enum wrapper to a DenseSequence structure
    Dense(DenseSequence<'a>),
    /// A simple enum wrapper to a SparseSequence structure
    Sparse(SparseSequence<'a>)
}
