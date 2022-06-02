// -*- mode: rust; -*-
//
// Authors:
// - Joe <jose@spaceandtime.io>
// - Ryan Burn <ryan@spaceandtime.io>

//! Computes Pedersen Commitments in the CPU / GPU

extern crate proofs_gpu;
extern crate curve25519_dalek;

use std::sync::Once;

use crate::sequence::Sequence;
pub use byte_slice_cast::AsByteSlice;

// holds the state of the backend initalization (0 for success, non-zero otherwise)
static mut INIT_STATE: i32 = 0; 

// static variable used to secure that the backend initialization is triggered only
static INIT: Once = Once::new();  

/// A simple alias to a dalek CompressedRistretto point
pub type Commitment = curve25519_dalek::ristretto::CompressedRistretto;

/// `init_backend` is responsible for initialize only once the C++ commitment backend.
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
/// During the previous executions, you had to specify the backend where the computation must proceed - either `cpu` or `gpu`. Implicitly, those backends need to be initialized before the commitment computation is called. Inside this commitment function, we call the backend initialization. But this process takes time. So you may want to call this function at the beginning of your program so that you don't pay this price later.
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

/// `to_sxt_commitments` generates a lower-level
/// sys crate `sxt_commitment` vector struct from
/// a given `num_sequences`
fn to_sxt_commitments(num_sequences: usize)
    -> Vec<proofs_gpu::sxt_commitment> {

    let mut cbinding_commitments: Vec<proofs_gpu::
            sxt_commitment> = Vec::with_capacity(num_sequences);

    unsafe {
        // sets the correct size for the `cbinding_commitments` vector
        cbinding_commitments.set_len(num_sequences);
    }

    return cbinding_commitments;
}

/// `to_sxt_descriptors` converts the data table from the 
/// sequence slice to the lower-level sys crate 
/// `sxt_sequence_descriptor` vector struct.
///
/// # Panics
///
/// If some data_slice has a `data_slice.len()` that is not a multiple of `element_size`
fn to_sxt_descriptors(data: & [Sequence])
     -> Vec<proofs_gpu::sxt_sequence_descriptor> {

    let num_sequences = data.len();
    let mut cbinding_descriptors: Vec<proofs_gpu::
        sxt_sequence_descriptor> = Vec::with_capacity(num_sequences);

    unsafe {
        // sets the correct size for the `cbinding_descriptors` vector
        cbinding_descriptors.set_len(num_sequences);
    }

    // populate the `cbinding_descriptors` vector array
    for i in 0..num_sequences {
        let curr_data = match &data[i] {
            Sequence::Dense(x) => x
        };

        if curr_data.data_slice.len() % curr_data.element_size != 0 {
            panic!("Error computing the number of rows in the data_slice");
        }
        
        let num_rows = curr_data.data_slice.len() / curr_data.element_size;

        let descriptor = proofs_gpu::sxt_dense_sequence_descriptor {
            element_nbytes: curr_data.element_size as u8,  // number bytes
            n: num_rows as u64,            // number rows
            data: curr_data.data_slice.as_ptr()   // data pointer
        };

        cbinding_descriptors[i] = proofs_gpu::sxt_sequence_descriptor {
            sequence_type: proofs_gpu::SXT_DENSE_SEQUENCE_TYPE as u8,
            __bindgen_anon_1: proofs_gpu::sxt_sequence_descriptor__bindgen_ty_1 {
                dense: descriptor
            }
        };
    }

    return cbinding_descriptors;
}

/// `to_commitments` converts the computed `sxt_commitments` commitments from the
/// lower-level sys crate structure to the higher-level `Commitment` structure.
///
/// # Panics
///
/// If `commitments` and `sxt_commitments` mismatch in size
fn to_commitments(commitments: & mut[Commitment], sxt_commitments: &[proofs_gpu::sxt_commitment]) {
    let num_sequences = sxt_commitments.len();

    // vectors mismatch in size
    if num_sequences != commitments.len() {
        panic!("Error writing the results to commitments vector");
    }
    
    // copy results back to commitments vector
    for i in 0..num_sequences {
        commitments[i] = Commitment::
                from_slice(&sxt_commitments[i].ristretto_bytes);
    }
}

/// `compute_commitments` computes the Pedersen commitment for a given input data.
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
/// set of random 1280-bit curve25519 points G_i, called row generators.
/// The total number of generators used to compute C_j is equal to 
/// the number of `num_rows` in the data\[j] sequence. The following formula
/// is specified to obtain the C_j commitment:
///
/// ```text
/// let C_j_temp = 0; // this is a 1280-bit curve25519 point
///
/// for j in 0..num_rows {
///     let G_i = get_random_ristretto_point(j);
///     let curr_data_ji = data[i].data_slice[j*el_size:(j + 1)*el_size];
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
/// * `commitments` - A slice view of a [Commitment][Commitment] memory area where the 
///                256-bit Ristretto point results will be written to. Please,
///                you need to guarantee that this slice captures exactly
///                data.len() element positions.
///
/// * `data` - A slice view of a [Sequence][Sequence] memory area, which captures the
///         slices of contiguous u8 memory elements. Given that each sequence
///         data\[i] captures an unsigned char slice view, you need to guarantee
///         that it captures the correct amount of bytes that can reflect
///         your desired amount of `num_rows` in the sequence. After all,
///         we infer the `num_rows` from data\[i].data_slice.len() / data\[i].element_size
///
///
/// # Panics
///
/// If the compute commitments execution in the GPU fails.
///
/// # Example 1 - Simple Commitment Computation
///
/// Computes the j commitment of each table column j containing m\[j].len() rows, using for that the random generators G given by our proofs-gpu code:
///
/// ```no_run
#[doc = include_str!("../../examples/simple_commitment.rs")]
/// ```
///
/// Run the example:
/// ```text
/// cargo run --features cpu --example simple_commitment
/// ```
///
/// # Example 2 - Adding and Multiplying Commitments
///
/// Let u = \[2000, 7500, 5000, 1500] be the 
/// weekly pay vector and and v = \[50000, 0, 400000, 0]
// be the yearly bonus vector. We can compute the commitments to each column:
///
/// ```text
/// C_u = 2000 * g_0 + 7500 * g_1 + 5000 * g_2 + 1500 * g_3
/// C_v = 50000 * g_0 + 0 * g_1 + 400000 * g_2 + 0 * g_3
/// ```
/// Recall, that C_u, C_v ∈ G (the Ristretto group over curve25519) are both 32-bytes. At some point, we may wish to
/// compute the total compensation using the formula
/// 
/// ```Total Compensation = 52 × Weekly Pay + Yearly Bonus```
///
/// Let w = \[154000, 390000, 660000, 78000] be
/// the total compensation vector. Note, we can write w = 52u + v, which means that by the homomorphic property of the commitment,
///
/// ```C_w = Commit(w) = Commit(52u + v) = 52 Commit(u) + Commit(v) = 52C_u + C_v```
///
/// ```no_run
#[doc = include_str!("../../examples/add_mult_commitments.rs")]
/// ```
///
/// Run the example:
///
///```text
///cargo run --features gpu --example add_mult_commitments
///```
pub fn compute_commitments(commitments: & mut[Commitment], data: & [Sequence])  {
    let num_sequences = data.len();
    let mut sxt_descriptors = to_sxt_descriptors(data);
    let mut sxt_commitments = to_sxt_commitments(num_sequences);

    init_backend();

    unsafe {
        // computes the commitments using the lower-level rust sys crate
        let ret_compute = proofs_gpu::sxt_compute_pedersen_commitments(
            sxt_commitments.as_mut_ptr(),
            num_sequences as u32,
            sxt_descriptors.as_mut_ptr(),
        );

        if ret_compute != 0 {
            panic!("Error during commitments computation");
        }
    }

    to_commitments(commitments, &sxt_commitments);
}

#[cfg(test)]
mod tests;
