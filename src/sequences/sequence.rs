// Copyright 2023-present Space and Time Labs, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
