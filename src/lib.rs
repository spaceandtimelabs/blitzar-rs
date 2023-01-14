// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

// Refuse to compile if documentation is missing.
#![deny(missing_docs)]

//! <h1 align="center" style="border-bottom: none;">Proofs-GPU 🦀🔑</h1>
//! <p align="center">
//!   <a href="">
//!     <img alt="Build states" src="">
//!   </a>
//!   <a href="#badge">
//!     <img alt="semantic-release: conventional-commits" src="https://img.shields.io/badge/semantic--release-conventional--commits-blueviolet">
//!   </a>
//!   <a href="#badge">
//!     <img alt="docs" src="https://img.shields.io/badge/docs-passing-green">
//!   </a>
//! </p>
//!
//! High-Level Rust wrapper for the proofs-gpu sys crate
//! For the rust sys-crate and the C++ repo, check
//! [here](https://github.com/spaceandtimelabs/proofs-gpu).
//!
//!## Considerations:
//!
//!1. The current library only supports `x86_64` architectures and only the linux operating system.
//!2. Until this point, the library was tested only in the `ubuntu20.04` linux environment.
//!3. Consider using `docker` or a virtual machine.
//!4. You must have the latest rust environment installed in your linux machine. Download [here](https://www.rust-lang.org/tools/install).
//!

//! ## Use

//! Add the following two lines to your `Cargo.toml` file:

//! ```text
//! [dependencies.proofs_gpu]
//! path = "/path/to/directory/proofs_gpu/"
//! ```

//! Don't forget to substitute this `path` with the correct location of your `proofs_gpu` directory.

//! Import the necessary modules to your rust code:

//! ```text
//! extern crate proofs_gpu;

//! use proofs_gpu::sequences::*;
//! use proofs_gpu::compute::*;
//! ```

//!## Examples
//!
//!All the examples are located in the `examples/` directory. Each one has its own `.rs` file. To run some example, use the following command:
//!
//!```text
//! cargo run --features <cpu | gpu> --example <example_name>
//!```
//!
//! ## Tests
//! ```text
//!  cargo test
//! ```
//! ## Benchmarks
//! Benchmarks are run using [criterion.rs](https://github.com/bheisler/criterion.rs):
//! ```text
//!  cargo bench --features gpu
//! ```

//------------------------------------------------------------------------
// proofs-gpu public modules
//------------------------------------------------------------------------

// Wrappers for data table
pub mod sequences;

// Responsible for all computations (commitments and generator computation)
pub mod compute;

// Responsible for all proof primitives
pub mod proof;
