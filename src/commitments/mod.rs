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

/// Responsible for initialize only once the C++ commitment backend.
///
/// This function initializes the backend so that computations
/// can proceed either in the CPU or in the GPU. The backend
/// type must be specified during `build` time using the following flags:
///
/// ```text
/// cargo build --features gpu # only the GPU is used
/// cargo build --features cpu # only the CPU is used
/// ```
///
/// Once the backend is initialized, it is not possible to change to
/// another one. Therefore, if the GPU feature is specified during build time,
/// then it is not possible to use the CPU to do the computations. 
/// In the case no feature is specified during build time, 
/// the backend will initialize with the GPU as default.
//
/// Also, any `compute` function will call this `init_backend`
/// securing that the GPU is always in a proper state. However,
/// this last case may introduce some initialization overhead that could
/// have been overlapped with CPU computation (such as reading data from database).
///  
/// Finally, to guarantee that the code inside this function is not
/// initiliazed multiple times, we use the `std::sync::Once` scheme.
///
/// # Panics
/// 
/// If the backend initialization fails.
///
/// # Example 1 - Initializing the Backend
///
/// During the previous executions, you had to specify the backend where the computation
/// must proceed - either `cpu` or `gpu`. Implicitly, those backends need to be initialized
/// before the commitment computation is called. Inside this commitment function, we call
/// the backend initialization. But this process takes time. So you may want to call this
/// function at the beginning of your program so that you don't pay this price later.
///
/// Follows the code:
///
/// ```no_run
#[doc = include_str!("../../examples/initialize_backend.rs")]
/// ```
///
/// Run the example:
/// ```text
/// $ cargo run --features gpu --example initialize_backend
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

// sealed trait from the outside package
mod private {
    pub trait Descriptor {
        fn to_sxt_descriptor(& self) -> (usize, proofs_gpu::sxt_sequence_descriptor);
    }
}

impl<'a> private::Descriptor for Sequence<'a> {
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

impl private::Descriptor for &[Scalar] {
    fn to_sxt_descriptor(& self) -> (usize, proofs_gpu::sxt_sequence_descriptor) {
        println!("Hellow world");
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

/// Computes the Pedersen commitment for a given input data.
///
/// In total, the function computes `data.len()` commitments,
/// which is related with the total number of columns in the data table. The commitment
/// results are stored as 256-bit Ristretto points in the `commitments` variable.
///
/// The j-th Pedersen commitment is a 256-bit Ristretto point C_j over the
/// curve25519 elliptic curve that is cryptographically binded to a data message vector M_j:
/// 
/// ```text
/// let el_size = data[j].element_size; // sizeof of each element in the current j-th column
/// let num_rows = data[j].data_slice.len() / el_size; // number of rows in the j-th column
///
/// let M_j = [
///    data[j].data_slice[0:el_size],
///    data[j].data_slice[el_size:2*el_size],
///    data[j].data_slice[2*el_size:3*el_size],
///    .,
///    .,
///    .,
///    data[j].data_slice[(num_rows-1)*el_size:num_rows*el_size]
/// ];
/// ```
///
/// This message M_j cannot be decrypted from C_j. The curve point C_j
/// is generated in a unique way using M_j and a
/// set of 1280-bit curve25519 points G_i, called row generators.
/// Although our gpu code uses 1280-bit generators during the scalar 
/// multiplication, these generators are passed as 256-bit Ristretto points
/// and only converted to 1280-bit points inside the GPU/CPU.
/// The total number of generators used to compute C_j is equal to 
/// the number of `num_rows` in the data\[j] sequence. The following formula
/// is specified to obtain the C_j commitment:
///
/// ```text
/// let C_j_temp = 0; // this is a 1280-bit curve25519 point
///
/// for j in 0..num_rows {
///     let G_i = generators[j].decompress(); // we decompress to convert 256-bit to 1280-bit points
///     let curr_data_ji = data[j].data_slice[i*el_size:(i + 1)*el_size];
///     C_j_temp = C_j_temp + curr_data_ji * G_i;
/// }
///
/// let C_j = convert_to_ristretto(C_j_temp); // this is a 256-bit Ristretto point
/// ```
///
/// Ps: the above is only illustrative code. It will not compile.
///
/// Here `curr_data_ji` are simply 256-bit scalars, C_j_temp and G_i are
/// 1280-bit curve25519 points, and C_j is a 256-bit Ristretto point.
/// 
/// Given M_j and G_i, it is easy to verify that the Pedersen
/// commitment C_j is the correctly generated output. However,
/// the Pedersen commitment generated from M_j and G_i is cryptographically
/// binded to the message M_j because finding alternative inputs M_j* and 
/// G_i* for which the Pedersen commitment generates the same point C_j
/// requires an infeasible amount of computation.
///
/// To guarantee proper execution, so that the backend is correctly setted,
/// this `compute_commitments` always calls the `init_backend()` function.
/// 
/// Portions of this documentations was extracted from
/// [here](findora.org/faq/crypto/pedersen-commitment-with-elliptic-curves/)
///
/// # Arguments
///
/// * `commitments` - A slice view of a CompressedRistretto memory area where the 
///                256-bit Ristretto point results will be written to. Please,
///                you need to guarantee that this slice captures exactly
///                data.len() element positions.
///
/// * `data` - A slice view of a [Sequence] memory area, which captures the
///         slices of contiguous u8 memory elements. Given that each sequence
///         data\[i] captures an unsigned char slice view, you need to guarantee
///         that it captures the correct amount of bytes that can reflect
///         your desired amount of `num_rows` in the sequence. After all,
///         we infer the `num_rows` from data\[i].data_slice.len() / data\[i].element_size
///
/// * `generators` - A sliced view of a CompressedRistretto memory area where the
///               256-bit Ristretto Point generators used in the commitment computation are
///               stored into. Bear in mind that the size of this slice must always be greater
///               or equal to the longest sequence, in terms of rows, in the table.
///
/// # Panics
///
/// If the compute commitments execution in the GPU / CPU fails, If the longest sequence
/// in the input data is bigger than the generators` length, or If
/// the data.len() value is different from the commitments.len() value.
///
/// # Example - Computing Commitments with user specified generators
///
/// Computes the j commitment of each table column j containing m\[j].len() rows, 
/// using for that the random generators G given by our proofs-gpu code:
///
/// ```no_run
#[doc = include_str!("../../examples/pass_generators_to_commitment.rs")]
/// ```
///
/// Run the example:
/// ```text
/// cargo run --features cpu --example pass_generators_to_commitment
/// ```
pub fn compute_commitments<T: private::Descriptor>(
    commitments: & mut[CompressedRistretto], data: & [T])  {

    let (sxt_descriptors, _longest_row) = to_sxt_descriptors(data);

    process_compute_commitments(
        commitments,
        &sxt_descriptors
    );
}

///
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

/// Gets the generators used in the `compute_commitments` function
///
/// In total, the function gets `generators.len()` points. These points
/// are converted from 1280-bit Curve25519 points used in the scalar multiplication
/// of the commitment computation, to 256-bit Ristretto points. This function
/// also allows the user to provide an offset so that a shift is applied in the
/// retrieval. The following operation is applied:
///
///```text
///for i in 0..generators.len() {
///    generators[i] = randomly_generate_curve25519_point(i + offset).to_compressed_ristretto();
///}
///```
///
/// # Arguments
///
/// * `generators` - A sliced view of a CompressedRistretto memory area where the
///               256-bit Ristretto Point generators used in the commitment computation will
///               be written into.
/// * `offset_generators` - A value that is used to shift the get generator operation by
///                         `offset_generators` values. With this shift, we have
///                         generator\[0] holding the value of randomly_generate_curve25519_point(0 + offset),
///                         generator\[1] holding the value of randomly_generate_curve25519_point(1 + offset),
///                         and so on.
///
/// # Panics
///
/// If the compute `get_generators` execution in the GPU / CPU fails.
///
/// # Example - Getting the Generators used in the `compute_commitments` function
//
/// ```no_run
#[doc = include_str!("../../examples/get_generators.rs")]
/// ```
///
/// Run the example:
///
///```text
///cargo run --features gpu --example get_generators
///```
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

#[cfg(test)]
mod tests;
