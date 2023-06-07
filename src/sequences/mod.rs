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

//! Wrappers for data table

mod dense_sequence;
pub use dense_sequence::DenseSequence;

mod dense_sequence_data;
pub use dense_sequence_data::DenseSequenceData;

mod sequence;
pub use sequence::Sequence;

mod descriptor;
pub(crate) use descriptor::to_sxt_descriptors;
pub use descriptor::Descriptor;
