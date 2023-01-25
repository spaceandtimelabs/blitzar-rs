// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

//! Wrappers for data table

mod dense_sequence;
pub use dense_sequence::DenseSequence;

mod sequence;
pub use sequence::Sequence;

mod descriptor;
pub(crate) use descriptor::{to_sxt_descriptors, Descriptor};
