// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

use super::dense_sequence::DenseSequence;
use super::sparse_sequence::SparseSequence;

/// Defines multiple wrappers so that all of them
/// can be stored in the same vector array.
/// We currently support `Dense` and `Sparse` structures.
///
/// To use this sequence, you can have:
///
/// ```no_run
/// use pedersen::sequences::*;
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
    Sparse(SparseSequence<'a>),
}
