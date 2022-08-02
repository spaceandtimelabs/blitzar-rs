// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

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
    pub data_indices: &'a [u64],
}

impl SparseSequence<'_> {
    /// Returns the number of elements in the Sparse Sequence
    pub fn len(&self) -> usize {
        (*self).data_slice.len() / (*self).element_size
    }

    /// Returns true if the sequence is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        (*self).len() == 0
    }

    pub(super) fn to_data_properties(&self) -> (u8, usize, *const u8, *const u64) {
        assert!(
            (*self).data_slice.len() % (*self).element_size == 0,
            "data_slice length is not a multiple of element_size in the sparse object"
        );

        let num_rows = (*self).len();

        assert!(
            num_rows == (*self).data_indices.len(),
            "number of rows differs from the data_indices length in the sparse object"
        );

        (
            (*self).element_size as u8,
            num_rows as usize,
            (*self).data_slice.as_ptr(),
            (*self).data_indices.as_ptr(),
        )
    }
}
