<div id="top"></div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h1 align="center">Blitzar Crate</h1>

<p align="center">
<img
 alt="blitzar logo"
 width="200px"
 src="https://raw.githubusercontent.com/spaceandtimelabs/blitzar-rs/assets/logo.png"/>
</p>

  <a href="https://github.com/spaceandtimelabs/blitzar-rs/actions/workflows/release.yml">
    <img alt="Build State" src="https://github.com/spaceandtimelabs/blitzar-rs/actions/workflows/release.yml/badge.svg">
  </a>

  <a href="https://twitter.com/intent/follow?screen_name=spaceandtimedb">
    <img alt="Twitter" src="https://img.shields.io/twitter/follow/spaceandtimedb.svg?style=social&label=Follow">
  </a>

  <a href="http://discord.gg/SpaceandTimeDB">
    <img alt="Discord Server" src="https://img.shields.io/discord/953025874154893342?logo=discord">
  </a>
  
  <a href="https://github.com/spaceandtimelabs/blitzar-rs/blob/main/LICENSE">
    <img alt="License" src="https://img.shields.io/badge/License-Apache_2.0-blue.svg">
    </a>
  </a>

  <a href="https://en.cppreference.com/w/cpp/20">
    <img alt="C++ Logo" src="https://img.shields.io/badge/C%2B%2B-20-blue?style=flat&logo=c%2B%2B">
    </a>
  </a>

  <a href="https://www.linux.org/">
    <img alt="OS" src="https://img.shields.io/badge/OS-Linux-blue?logo=linux">
    </a>
  </a>

  <a href="https://www.linux.org/">
    <img alt="CPU" src="https://img.shields.io/badge/CPU-x86-red">
    </a>
  </a>

  <a href="https://developer.nvidia.com/cuda-downloads">
    <img alt="CUDA" src="https://img.shields.io/badge/CUDA-12.1-green?style=flat&logo=nvidia">
    </a>
  </a>

  <p align="center">
    High-Level Rust wrapper for the blitzar-sys crate.
    <br />
    <a href="https://github.com/spaceandtimelabs/blitzar-rs/issues">Report Bug</a>
    |
    <a href="https://github.com/spaceandtimelabs/blitzar-rs/issues">Request a Feature</a>
  </p>
</div>

#### Background
Blitzar was created by the core cryptography team at [Space and Time](https://www.spaceandtime.io/) to accelerate Proof of SQL, a novel zero-knowledge proof for SQL operations. After surveying our options for a GPU acceleration framework, we realized that Proof of SQL needed something better… so we built Blitzar. Now, Proof of SQL runs with a 3.2 second proving time against a million-row table on a single GPU, and it’s only getting faster.

We’ve open-sourced Blitzar to provide the Web3 community with a faster and more
robust framework for building GPU-accelerated zk-proofs. We’re excited to open
the project to community contributions to expand the scope of Blitzar and lay
the foundation for the next wave of lightning fast zk-proofs.

#### Overview
Blitzar-rs is a High-Level rust wrapper for the [blitzar-sys crate](https://github.com/spaceandtimelabs/blitzar/tree/main/rust/blitzar-sys) for accelerating cryptographic zero-knowledge proof algorithms on the CPU and GPU.
> **Note**
> This repo contains the high-Level rust wrapper for the blitzar-sys crate. If you are using C++, use the code from the companion repo here: https://github.com/spaceandtimelabs/blitzar.

The crate provides

* Functions for doing group operations on [Curve-25519](https://en.wikipedia.org/wiki/Curve25519) and [Ristretto25519](https://ristretto.group/) elements.
* An implementation of [Inner Product Argument Protocol](https://eprint.iacr.org/2017/1066.pdf) for producing and verifying a compact proof of the inner product of two vectors.

**WARNING**: This project has not undergone a security audit and is NOT ready
for production use.

#### Computational Backends
Although the primary goal of this library is to provide GPU acceleration for cryptographic zk-proof algorithms, the library also provides CPU support for the sake of testing. The following backends are supported:

| Backend            | Implementation                                             | Target Hardware             |
| :---               | :---                                                       | :---                        |
| `cpu`             | Serial      | x86 capable CPUs |
| `gpu`             | Parallel   | Nvidia CUDA capable GPUs


## Cryptographic Primitives

#### Multi-Scalar Multiplication (MSM) / Generalized Pedersen Commitment / Multiexponentiation

Blitzar provides an implementation of Multi-Scalar Multiplication (i.e. generalized Pedersen commitments). Mathematical details behind MSM are outlined in the [Blitzar Github repository](https://github.com/spaceandtimelabs/blitzar#multi-scalar-multiplication-msm--generalized-pedersen-commitment--multiexponentiation).

Note: we interchangeably use the terms "multi-scalar multiplication" and "multiexponentiation" to refer to the this operation because when the group is written additively, the operation is a multi-scalar multiplication, and when the group is written multiplicatively, the operation is a multiexponentiation.

The Blitzar implementation allows for computation of multiple, potentially different length, MSMs simultaneously. Additionally, either built-in, precomputed, generators can be used, or they can be provided as needed.

Currently, Blitzar supports Curve25519 as the group. We're always working to expand the curves that we support, so check back for updates.

#### Inner Product Argument

Blitzar provides a modified implementation of an inner product argument (e.g. [Bulletproofs](https://eprint.iacr.org/2017/1066.pdf) and [Halo2](https://zcash.github.io/halo2/background/pc-ipa.html)). Mathematical details of the modified inner product argument are outlined in the [Blitzar Github repository](https://github.com/spaceandtimelabs/blitzar#inner-product-argument).

#### Other Features to Come

If there is a particular feature that you would like to see, please [reach out](https://github.com/spaceandtimelabs/blitzar/issues). Blitzar is a community-first project, and we want to hear from you.

## Getting Started

To get a local copy up and running, consider the following steps.

### Prerequisites to build from source

<details open>
<summary>GPU backend prerequisites:</summary>

* [Rust 1.69.0](https://www.rust-lang.org/tools/install)
* `x86_64` Linux instance.
* Nvidia Toolkit Driver Version: >= 530.30.02 (check the [compatibility list here](https://docs.nvidia.com/cuda/cuda-toolkit-release-notes/index.html)).

</details>

<details>
<summary>CPU backend prerequisites:</summary>

You'll need the following requirements to run the environment:

* [Rust 1.69.0](https://www.rust-lang.org/tools/install)
* `x86_64` Linux instance.

</details>

## Usage

### Add to your project

To add this library to your project, update your `Cargo.toml` file with the following line:

```
[dependencies]
blitzar = "0.2.0"
```

### Feature Flags

| Feature            | Default? | Description |
| :---               |  :---:   | :---        |
| `cpu`          |    x     | Enables the CPU backend. |
| `gpu`            |    ✓     | Enables the GPU Backend. |

### Tests

```bash
cargo test
```

### Documentation

```bash
cargo doc --no-deps --open
```

### Examples

Check [EXAMPLES](docs/EXAMPLES.md) file.

### Running benchmarks:

Benchmarks are run using [criterion.rs](https://github.com/bheisler/criterion.rs):

```
$ cargo bench --features <cpu | gpu>
```

## Development Process

The main branch is regularly built and tested, being the only source of truth. [Tags](https://github.com/spaceandtimelabs/blitzar-rs/tags) are created regularly from automated semantic release executions. 

## Contributing

We're excited to open Blitzar-rs to the community, but are not accepting community Pull Requests yet due to logistic reasons. However, feel free to contribute with any suggestion, idea, or bugfix on our [Issues](https://github.com/spaceandtimelabs/blitzar-rs/issues) panel. Also, see [contribution guide](CONTRIBUTING.md).

## Community & support

Join our [Discord server](https://discord.com/invite/SpaceandTimeDB) to ask questions, discuss features, and for general chat.

## License

This project is released under the [Apache 2 License](LICENSE).
  
## C++ code

This repo contains the high-Level rust wrapper for the blitzar-sys crate. If you are using C++, use the code from the companion repo here: https://github.com/spaceandtimelabs/blitzar.
