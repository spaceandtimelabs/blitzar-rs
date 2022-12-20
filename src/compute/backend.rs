// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>
use std::sync::Once;

/// struct to hold configuration values about the chosen backend
pub struct BackendConfig {
    /// The total number of precomputed values to be generated
    pub num_precomputed_generators: u64,
}

// holds the state of the backend initalization (0 for success, non-zero otherwise)
static mut INIT_STATE: i32 = 0;

// static variable used to secure that the backend initialization is triggered only once
static INIT: Once = Once::new();

#[doc = include_str!("../../docs/commitments/init_backend.md")]
///
/// # Example - Initializing the Backend
///
/// Backends need to be initialized
/// before the commitment computation is called. You may want to call this
/// function at the beginning of your program to prevent later initialization overhead.
///
/// ```no_run
#[doc = include_str!("../../examples/initialize_backend.rs")]
/// ```
pub fn init_backend() {
    unsafe {
        let num_precomputed_generators: u64 = 20;

        INIT.call_once(|| {
            // verify which feature backend was passed to the build
            let backend = if cfg!(feature = "naive-cpu") {
                proofs_gpu_sys::SXT_NAIVE_BACKEND_CPU
            } else if cfg!(feature = "naive-gpu") {
                proofs_gpu_sys::SXT_NAIVE_BACKEND_GPU
            } else {
                proofs_gpu_sys::SXT_PIPPENGER_BACKEND_CPU
            } as i32;

            // initializes the backend using the lower-level rust sys crate
            let config: proofs_gpu_sys::sxt_config = proofs_gpu_sys::sxt_config {
                backend,
                num_precomputed_generators,
            };

            INIT_STATE = proofs_gpu_sys::sxt_init(&config);
        });

        if INIT_STATE != 0 {
            panic!("Error during backend initialization");
        }
    };
}

#[doc = include_str!("../../docs/commitments/init_backend_with_config.md")]
///
/// # Example - Initializing the Backend with provided Configuration values
///
/// Backends need to be initialized
/// before the commitment computation is called. You may want to call this
/// function at the beginning of your program to prevent later initialization overhead.
/// Specifying a `config.num_precomputed_generators` > 0 forces the `config.num_precomputed_generators`
/// generators to be computed and stored at the CPU memory. Later, those are used
/// with the commitment computation.
///
/// ```no_run
#[doc = include_str!("../../examples/initialize_backend_with_config.rs")]
/// ```
pub fn init_backend_with_config(config: BackendConfig) {
    unsafe {
        INIT.call_once(|| {
            // verify which feature backend was passed to the build
            let backend = if cfg!(feature = "naive-cpu") {
                proofs_gpu_sys::SXT_NAIVE_BACKEND_CPU
            } else if cfg!(feature = "naive-gpu") {
                proofs_gpu_sys::SXT_NAIVE_BACKEND_GPU
            } else {
                proofs_gpu_sys::SXT_PIPPENGER_BACKEND_CPU
            } as i32;

            // initializes the backend using the lower-level rust sys crate
            let config: proofs_gpu_sys::sxt_config = proofs_gpu_sys::sxt_config {
                backend,
                num_precomputed_generators: config.num_precomputed_generators,
            };

            INIT_STATE = proofs_gpu_sys::sxt_init(&config);
        });

        if INIT_STATE != 0 {
            panic!("Error during backend initialization");
        }
    };
}
