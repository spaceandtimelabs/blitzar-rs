// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

//! Computes Pedersen Commitments in the CPU / GPU

mod backend;
pub use backend::{init_backend, init_backend_with_config, BackendConfig};

mod commitments;
pub use commitments::{
    compute_commitments, compute_commitments_with_generators, update_commitment,
};

#[cfg(test)]
mod commitments_tests;

mod generators;
pub use generators::{get_generators, get_one_commit};

#[cfg(test)]
mod generators_tests;
