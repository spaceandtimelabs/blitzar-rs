// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

//! Computes Pedersen Commitments in the CPU / GPU

extern crate proofs_gpu;
extern crate curve25519_dalek;

use std::ptr;
use std::cmp;
use std::sync::Once;

use crate::sequence::Sequence;

pub use byte_slice_cast::AsByteSlice;
pub use curve25519_dalek::scalar::Scalar;
pub use curve25519_dalek::ristretto::RistrettoPoint;
pub use curve25519_dalek::ristretto::CompressedRistretto;

// holds the state of the backend initalization (0 for success, non-zero otherwise)
static mut INIT_STATE: i32 = 0; 

// static variable used to secure that the backend initialization is triggered only
static INIT: Once = Once::new();

#[doc = include_str!("../../docs/commitments/init_backend.md")]
///
/// # Example - Initializing the Backend
///
/// During the previous executions, you had to specify the backend where the computation
/// must proceed - either `cpu` or `gpu`. Implicitly, those backends need to be initialized
/// before the commitment computation is called. You may want to call this
/// function at the beginning of your program to prevent later initialization overhead.
///
/// ```no_run
#[doc = include_str!("../../examples/initialize_backend.rs")]
/// ```
pub fn init_backend() {
    unsafe {
        INIT.call_once(|| {
            let curr_backend;
            
            // verify which feature backend was passed to the build
            if cfg!(feature = "cpu") {
                curr_backend = proofs_gpu::SXT_BACKEND_CPU;
            } else {
                curr_backend = proofs_gpu::SXT_BACKEND_GPU;
            }

            // initializes the backend using the lower-level rust sys crate
            let config: proofs_gpu::sxt_config = proofs_gpu::sxt_config {
                backend: curr_backend as i32
            };
        
            INIT_STATE = proofs_gpu::sxt_init(&config);
        });
        
        if INIT_STATE != 0 {
            panic!("Error during backend initialization");
        }
    };
}

/// `to_sxt_ristretto_elements` generates a lower-level
/// sys crate `sxt_ristretto_element` vector struct from
/// a given `num_sequences`
fn to_sxt_ristretto_elements(num_sequences: usize)
    -> Vec<proofs_gpu::sxt_ristretto_element> {

    let mut cbinding_commitments: Vec<proofs_gpu::
            sxt_ristretto_element> = Vec::with_capacity(num_sequences);

    unsafe {
        // sets the correct size for the `cbinding_commitments` vector
        cbinding_commitments.set_len(num_sequences);
    }

    return cbinding_commitments;
}

/// `to_sxt_ristretto_generators` generates a lower-level
/// sys crate `sxt_ristretto_element` vector struct from
/// a given `CompressedRistretto` slice
fn to_sxt_ristretto_generators(generators: &[CompressedRistretto])
    -> Vec<proofs_gpu::sxt_ristretto_element> {

    let mut cbinding_generators: Vec<proofs_gpu::
            sxt_ristretto_element> = Vec::with_capacity(generators.len());
        
    unsafe {
        // sets the correct size for the `cbinding_generators` vector
        cbinding_generators.set_len(generators.len());
    }

    for i in 0..generators.len() {
        cbinding_generators[i].ristretto_bytes = generators[i].to_bytes();
    }

    return cbinding_generators;
}

// Sealed Trait from the outside world
mod private {
    // This descriptor is only used to implement the `to_sxt_descriptor` method
    // for the `Sequence<'a>` and the `&[Scalar]` elements
    pub trait Descriptor {
        // returns the number of elements referenced by the descriptor
        fn len(& self) -> usize;
        
        // returns true if the descriptor is a Sequence:Sparse, and false otherwise
        fn is_sparse(& self) -> bool;

        // converts the descriptor to a sxt_descriptor
        fn to_sxt_descriptor(& self) -> (usize, proofs_gpu::sxt_sequence_descriptor);
    }
}

/// Implement the `to_sxt_descriptor` method for the `Sequence<'a>` datatype
impl<'a> private::Descriptor for Sequence<'a> {
    fn len(& self) -> usize {
        match self {
            Sequence::Dense(x) => return x.len(),
            Sequence::Sparse(y) => return y.len()
        }
    }

    fn is_sparse(& self) -> bool {
        match self {
            Sequence::Dense(_x) => return false,
            Sequence::Sparse(_y) => return true
        }
    }
    
    fn to_sxt_descriptor(& self) -> (usize, proofs_gpu::sxt_sequence_descriptor) {
        let (element_nbytes, num_rows, data, indices) = match self {
            Sequence::Dense(x) => x.to_data_properties(),
            Sequence::Sparse(y) => y.to_data_properties()
        };

        let descriptor = proofs_gpu::sxt_sequence_descriptor {
            element_nbytes: element_nbytes,
            n: num_rows as u64,
            data: data,
            indices: indices
        };

        return (num_rows, descriptor);
    }
}

/// Implement the `to_sxt_descriptor` method for the `&[Scalar]` datatype
impl private::Descriptor for &[Scalar] {
    fn len(& self) -> usize {
        return (*self).len();
    }

    fn is_sparse(& self) -> bool {
        return false;
    }

    fn to_sxt_descriptor(& self) -> (usize, proofs_gpu::sxt_sequence_descriptor) {
        let num_rows = (*self).len();

        let descriptor = proofs_gpu::sxt_sequence_descriptor {
            element_nbytes: 32,  // number bytes
            n: num_rows as u64,            // number rows
            data: (*self).as_ptr() as *const u8,  // data pointer
            indices: ptr::null()
        };

        return (num_rows, descriptor);
    }
}

/// `to_sxt_descriptors` converts the data table from the 
/// sequence slice to the lower-level sys crate 
/// `sxt_sequence_descriptor` vector struct.
///
/// # Panics
///
/// If some data_slice has a `data_slice.len()` that is not a multiple of `element_size`
fn to_sxt_descriptors<T: private::Descriptor>(data: & [T])
     -> (Vec<proofs_gpu::sxt_sequence_descriptor>, usize) {

    let mut longest_row: usize = 0;
    let num_sequences = data.len();
    let mut cbinding_descriptors: Vec<proofs_gpu::
        sxt_sequence_descriptor> = Vec::with_capacity(num_sequences);

    unsafe {
        // sets the correct size for the `cbinding_descriptors` vector
        cbinding_descriptors.set_len(num_sequences);
    }

    // populate the `cbinding_descriptors` vector array
    for i in 0..num_sequences {
        let (num_rows, descriptor) = data[i].to_sxt_descriptor();

        longest_row = cmp::max(longest_row, num_rows);

        cbinding_descriptors[i] = descriptor;
    }

    return (cbinding_descriptors, longest_row);
}

/// Converts the computed `sxt_ristretto_elements` commitments from the
/// lower-level sys crate structure to the higher-level `CompressedRistretto` structure.
///
/// # Panics
///
/// If `commitments` and `sxt_ristretto_elements` mismatch in size
fn to_pedersen_commitments(commitments: & mut[CompressedRistretto],
    sxt_ristretto_elements: &[proofs_gpu::sxt_ristretto_element]) {

    let num_sequences = sxt_ristretto_elements.len();

    // vectors mismatch in size
    if num_sequences != commitments.len() {
        panic!("Error writing the results to commitments vector");
    }
    
    // copy results back to commitments vector
    for i in 0..num_sequences {
        commitments[i] = CompressedRistretto::
                from_slice(&sxt_ristretto_elements[i].ristretto_bytes);
    }
}

/// Process the commitment computation
///
/// # Panics
///
/// If the commitment computation fails executing.
fn process_compute_commitments(
    commitments: & mut[CompressedRistretto],
    sxt_descriptors: &[proofs_gpu::sxt_sequence_descriptor]) {

    let num_sequences = sxt_descriptors.len();
    let mut sxt_ristretto_elements = to_sxt_ristretto_elements(num_sequences);

    init_backend();

    unsafe {
        // computes the commitments using the lower-level rust sys crate
        let ret_compute = proofs_gpu::sxt_compute_pedersen_commitments(
            sxt_ristretto_elements.as_mut_ptr(),
            num_sequences as u32,
            sxt_descriptors.as_ptr(),
        );

        if ret_compute != 0 {
            panic!("Error during commitments computation");
        }
    }

    to_pedersen_commitments(commitments, &sxt_ristretto_elements);
}

/// Process the commitment computation using user-generators
///
/// # Panics
///
/// If the commitment computation fails executing.
fn process_compute_commitments_with_generators(
    commitments: & mut[CompressedRistretto], 
    sxt_descriptors: &[proofs_gpu::sxt_sequence_descriptor],
    longest_row: usize,
    generators: &[CompressedRistretto]) {

    let num_sequences = sxt_descriptors.len();
    let mut sxt_ristretto_elements = to_sxt_ristretto_elements(num_sequences);
    let mut sxt_ristretto_generators = to_sxt_ristretto_generators(generators);

    if longest_row > generators.len() {
        panic!("Generator slice has a length smaller than the longest sequence in the input data.");
    }

    init_backend();

    unsafe {
        // computes the commitments using the lower-level rust sys crate
        let ret_compute = proofs_gpu::sxt_compute_pedersen_commitments_with_generators(
            sxt_ristretto_elements.as_mut_ptr(),
            num_sequences as u32,
            sxt_descriptors.as_ptr(),
            sxt_ristretto_generators.as_mut_ptr()
        );

        if ret_compute != 0 {
            panic!("Error during commitments computation");
        }
    }

    to_pedersen_commitments(commitments, &sxt_ristretto_elements);
}

#[doc = include_str!("../../docs/commitments/compute_commitments.md")]
///
/// # Example 1 - Simple Commitment Computation
///```no_run
#[doc = include_str!("../../examples/simple_commitment.rs")]
///```
///
/// # Example 2 - Adding and Multiplying Commitments
///```no_run
#[doc = include_str!("../../examples/add_mult_commitments.rs")]
///```
///
/// # Example 3 - Compute Commitments with Dalek Scalars
///```no_run
#[doc = include_str!("../../examples/simple_scalars_commitment.rs")]
///```
///
/// # Example 4 - Compute Commitments with Sparse Sequences
///```no_run
#[doc = include_str!("../../examples/simple_sparse_commitment.rs")]
///```
pub fn compute_commitments<T: private::Descriptor>(
    commitments: & mut[CompressedRistretto], data: & [T])  {

    let (sxt_descriptors, _longest_row) = to_sxt_descriptors(data);

    process_compute_commitments(
        commitments,
        &sxt_descriptors
    );
}


#[doc = include_str!("../../docs/commitments/compute_commitments_with_generators.md")]
///
///# Example 1 - Pass generators to Commitment Computation
///```no_run
#[doc = include_str!("../../examples/pass_generators_to_commitment.rs")]
///```
///
/// Example 2 - Compute Commitments with Dalek Scalars and User Generators
///```no_run
#[doc = include_str!("../../examples/pass_generators_and_scalars_to_commitment.rs")]
///```
pub fn compute_commitments_with_generators<T: private::Descriptor>(
    commitments: & mut[CompressedRistretto], 
    data: & [T], generators: &[CompressedRistretto])  {

    let (sxt_descriptors, longest_row) = to_sxt_descriptors(data);

    process_compute_commitments_with_generators(
        commitments,
        &sxt_descriptors,
        longest_row,
        generators
    );
}

#[doc = include_str!("../../docs/commitments/get_generators.md")]
///
/// # Example - Getting the Generators used in the `compute_commitments` function
//
/// ```no_run
#[doc = include_str!("../../examples/get_generators.rs")]
/// ```
pub fn get_generators(generators: & mut[CompressedRistretto], offset_generators: u64) {
    let mut sxt_ristretto_generators = to_sxt_ristretto_elements(generators.len());

    init_backend();

    unsafe {
        let ret_get_generators = proofs_gpu::sxt_get_generators(
            sxt_ristretto_generators.as_mut_ptr(),
            generators.len() as u64,
            offset_generators
        );

        if ret_get_generators != 0 {
            panic!("Error during access generators");
        }
    }

    to_pedersen_commitments(generators, &sxt_ristretto_generators);
}

#[doc = include_str!("../../docs/commitments/update_commitments.md")]
///
/// # Example - Update commitments with dense, sparse, and dalek scalars
//
/// ```no_run
#[doc = include_str!("../../examples/simple_update_commitment.rs")]
/// ```
pub fn update_commitment<T: private::Descriptor>(
    commitment: & mut CompressedRistretto, offset_generators: u64, data: T) {

    let mut partial_commitment = [CompressedRistretto::from_slice(&[0 as u8; 32]); 1];

    // When the data is a sparse sequence, 
    // we don't use the offset_generators,
    // because each data element is already
    // tied with its own row
    if data.is_sparse() {
        compute_commitments(
            &mut partial_commitment,
            &[data]
        );
    } else {
        // Otherwise, we fetch the generators from our proofs_gpu sys crate
        // and then we use them to compute the partial commitment out of the given data
        let mut generators = vec![CompressedRistretto::from_slice(&[0 as u8; 32]); data.len()];

        get_generators(
            &mut generators,
            offset_generators
        );
    
        compute_commitments_with_generators(
            &mut partial_commitment,
            &[data],
            &generators
        );
    }

    // using the A = `partial_commitment` and the B = `commitment`
    // given by the user, we compute a new commitment as B = A + B,
    // and then we write the result back to the `commitment` variable
    let c_a = match (*commitment).decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    let c_b = match partial_commitment[0].decompress() {
        Some(pt) => pt,
        None => panic!("Invalid ristretto point decompression")
    };

    (*commitment) = (c_a + c_b).compress();
}

#[cfg(test)]
mod tests;
