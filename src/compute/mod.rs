// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>
// - Ian Joiner <ian.joiner@spaceandtime.io>

//! High-Level Rust wrapper for the proofs-gpu sys crate.

mod backend;
pub use backend::{init_backend, init_backend_with_config, BackendConfig};

mod commitments;
pub use commitments::{
    compute_commitments, compute_commitments_with_generators, update_commitments,
};

#[cfg(test)]
mod commitments_tests;

mod generators;
pub use generators::{get_generators, get_one_commit};

#[cfg(test)]
mod generators_tests;
