// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

use super::dense_sequence::DenseSequence;

/// Defines multiple wrappers so that all of them
/// can be stored in the same vector array.
/// We currently support `Dense` structures only.
///
/// To use this sequence, you can have:
///
/// ```no_run
/// use blitzar::sequences::*;
/// use byte_slice_cast::AsByteSlice;
///
/// let dense_data: Vec<u32> = vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0];
/// let mut table: Vec<Sequence> = Vec::new();
///
/// table.push(Sequence::Dense(DenseSequence {
///     data_slice: &dense_data.as_byte_slice(),
///     element_size: std::mem::size_of_val(&dense_data[0])
/// }));
/// ```
pub enum Sequence<'a> {
    /// A simple enum wrapper to a DenseSequence structure
    Dense(DenseSequence<'a>),
}
