// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

// Refuse to compile if documentation is missing.
#![deny(missing_docs)]

//! <h1 align="center" style="border-bottom: none;">Pedersen 🦀🔑</h1>
//! <p align="center">
//!   <a href="https://github.com/spaceandtimelabs/pedersen/actions?query=workflow%3ATest+branch%3Amain">
//!     <img alt="Build states" src="https://img.shields.io/badge/tests-passing-green">
//!   </a>
//!   <a href="#badge">
//!     <img alt="semantic-release: conventional-commits" src="https://img.shields.io/badge/semantic--release-conventional--commits-blueviolet">
//!   </a>
//!   <a href="#badge">
//!     <img alt="docs" src="https://img.shields.io/badge/docs-passing-green">
//!   </a>
//! </p>
//!
//! Pedersen Commitment Library. 
//! A high-level rust wrapper for low-level
//! rust sys-crate, which wrappers a
//! c++ GPU / CPU implementation of 
//! group operations on Ristretto and Curve25519.
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
//! [dependencies.pedersen]
//! path = "/path/to/directory/pedersen/"
//! ```

//! Don't forget to substitute this `path` with the correct location of your `pedersen` directory.

//! Import the necessary modules to your rust code:

//! ```
//! extern crate pedersen;

//! use pedersen::sequence::*;
//! use pedersen::commitments::*;
//! ```

//!## Examples
//!
//!All the examples are located in the `examples/` directory. Each one has its own `.rs` file. To run some example, use the following command:
//!
//!```text
//! cargo run --features <cpu | gpu> --example <example_name>
//!```
//!
//!In this command, you can either specify if the computations should proceed in the cpu or in the gpu. Also, you need to specify the example name as it is in the `.rs` files.
//!
//! ### Example 1: Simple Commitment Computation
//! ```text
//! cargo run --features cpu --example simple_commitment
//! ```
//!
//! ### Example 2 - Adding and Multiplying Commitments
//!
//!```text
//!cargo run --features gpu --example add_mult_commitments
//!```
//!
//! ### Example 3: Initializing the Backend
//! ```text
//! $ cargo run --features gpu --example initialize_backend
//! ```
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
// pedersen public modules
//------------------------------------------------------------------------

// Wrappers for data table 
pub mod sequence;

// Compute commitments from data sequences
pub mod commitments;
