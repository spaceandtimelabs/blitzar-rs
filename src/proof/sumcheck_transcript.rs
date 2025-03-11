// Copyright 2024-present Space and Time Labs, Inc.
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

/// Provide random challenges for sumcheck proof.
pub trait SumcheckTranscript<T> {
    /// Provides dimensions of sumcheck proof.
    fn init(&mut self, num_variables: usize, round_degree: usize);

    /// Produce the challenge field element for a sumcheck
    /// round given the round polynomial
    ///   polynomial[0] + polynomial[1] X^1 + ... + polynomial[d] X^d
    fn round_challenge(&mut self, polynomial: &[T]) -> T;
}
