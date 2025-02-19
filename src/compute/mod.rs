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
pub use backend::{BackendConfig, init_backend, init_backend_with_config};

mod curve;
use curve::CurveId;

mod commitments;
pub use commitments::{
    compute_bls12_381_g1_commitments_with_generators,
    compute_bn254_g1_uncompressed_commitments_with_generators,
    compute_bn254_g1_uncompressed_commitments_with_halo2_generators,
    compute_curve25519_commitments, compute_curve25519_commitments_with_generators,
    compute_grumpkin_uncompressed_commitments_with_generators, update_curve25519_commitments,
};

#[cfg(test)]
mod commitments_tests;

mod conversion;
pub use conversion::{
    convert_bn254_g1_affine_generators_from_halo2_to_ark, convert_commitments_from_ark_to_halo2,
    convert_commitments_from_halo2_to_arkworks,
};
#[cfg(test)]
mod conversion_tests;

mod element_p2;
pub use element_p2::ElementP2;
#[cfg(test)]
mod element_p2_test;

mod fixed_msm;
pub use fixed_msm::{MsmHandle, SwMsmHandle};
#[cfg(test)]
mod fixed_msm_tests;

mod generators;
pub use generators::{get_curve25519_generators, get_one_curve25519_commit};

#[cfg(test)]
mod generators_tests;
