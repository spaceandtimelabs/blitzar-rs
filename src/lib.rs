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

// Refuse to compile if documentation is missing.
#![deny(missing_docs)]

//! <p align="center">
//!   <picture>
//!     <source media="(prefers-color-scheme: dark)" width="200px" srcset="https://raw.githubusercontent.com/spaceandtimelabs/blitzar-rs/assets/logo_dark_background.png">
//!     <source media="(prefers-color-scheme: light)" width="200px" srcset="https://raw.githubusercontent.com/spaceandtimelabs/blitzar-rs/assets/logo_light_background.png">
//!     <img alt="Blitzar" width="200px" src="https://raw.githubusercontent.com/spaceandtimelabs/blitzar-rs/assets/logo_light_background.png">
//!   </picture>
//! </p>
//! <p align="center">
//!   <a href="https://crates.io/crates/blitzar">
//!     <img alt="crates.io version" src="https://img.shields.io/crates/v/blitzar.svg">
//!   </a>
//!   <a href="https://github.com/spaceandtimelabs/blitzar-rs">
//!     <img alt="Build states" src="https://github.com/spaceandtimelabs/blitzar-rs/actions/workflows/release.yml/badge.svg">
//!   </a>
//!   <a href="https://docs.rs/crate/blitzar/">
//!     <img alt="docs.rs" src="https://img.shields.io/docsrs/blitzar">
//!   </a>
//!   <a href="#badge">
//!     <img alt="semantic-release: conventional-commits" src="https://img.shields.io/badge/semantic--release-conventional--commits-blueviolet">
//!   </a>
//! </p>
//!
//! High-Level Rust wrapper for the `blitzar-sys` crate.
//! For the Rust sys-crate and the C++ repo, check
//! [here](https://github.com/spaceandtimelabs/blitzar).
//!
//! ## Considerations:
//!
//! 1. The current library only supports `x86_64` architectures and only the Linux operating system.
//! 2. The library was tested in the `ubuntu22.04` Linux environment.
//! 3. Consider using `docker` or a virtual machine.
//! 4. You must have the latest Rust environment installed in your Linux machine. Download [here](https://www.rust-lang.org/tools/install).
//!

//! ## Use
//!
//! Add the following two lines to your `Cargo.toml` file:
//! ```text
//! [dependencies]
//! blitzar = <version_number>
//! ```
//!
//! Import the necessary modules to your rust code:
//! ```text
//! extern crate blitzar;
//!
//! use blitzar::sequence::*;
//! use blitzar::compute::*;
//! ```

//!## Examples
//!
//!All the examples are located in the `examples/` directory. Each one has its own `.rs` file. To run some example, use the following command:
//!
//!```text
//! cargo run --features <cpu|gpu> --example <example_name>
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
// blitzar public modules
//------------------------------------------------------------------------

// Wrappers for data table
pub mod sequence;

// Responsible for all computations (commitments and generator computation)
pub mod compute;

// Responsible for all proof primitives
pub mod proof;
