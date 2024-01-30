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

//! commitment and generator computation

mod backend;
pub use backend::{init_backend, init_backend_with_config, BackendConfig};

mod commitments;
pub use commitments::{
    compute_bls12_381_g1_commitments_with_generators, compute_curve25519_commitments,
    compute_curve25519_commitments_with_generators, update_curve25519_commitments,
};

#[cfg(test)]
mod commitments_tests;

mod generators;
pub use generators::{get_curve25519_generators, get_one_curve25519_commit};

#[cfg(test)]
mod generators_tests;
