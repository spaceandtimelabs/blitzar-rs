//! High-Level Rust wrapper for the blitzar-sys crate proof primitives.

mod error;
pub use error::ProofError;

mod inner_product;
pub use inner_product::InnerProductProof;

#[cfg(test)]
mod inner_product_tests;
