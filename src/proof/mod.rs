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

//! proof primitives

mod error;
pub use error::ProofError;

mod inner_product;
pub use inner_product::InnerProductProof;

#[cfg(test)]
mod inner_product_tests;

mod field;
mod sumcheck_transcript;
pub use sumcheck_transcript::SumcheckTranscript;

mod sumcheck;
pub use sumcheck::SumcheckProof;

#[cfg(test)]
mod sumcheck_tests;
