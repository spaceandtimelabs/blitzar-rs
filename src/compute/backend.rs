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
use std::sync::Once;

/// struct to hold configuration values about the chosen backend
pub struct BackendConfig {
    /// The total number of precomputed values to be generated
    pub num_precomputed_generators: u64,
}

// holds the state of the backend initalization (0 for success, non-zero otherwise)
static mut INIT_STATE: i32 = 0;

// static variable used to assure that the backend initialization is triggered only once
static INIT: Once = Once::new();

/// verify which feature backend was passed to the build
fn get_backend() -> i32 {
    if cfg!(feature = "cpu") {
        blitzar_sys::SXT_CPU_BACKEND as i32
    } else if cfg!(feature = "gpu") {
        blitzar_sys::SXT_GPU_BACKEND as i32
    } else {
        panic!("Incorrect backend specified");
    }
}

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
            let backend = get_backend();

            // initializes the backend using the lower-level rust sys crate
            let config: blitzar_sys::sxt_config = blitzar_sys::sxt_config {
                backend,
                num_precomputed_generators,
            };

            INIT_STATE = blitzar_sys::sxt_init(&config);
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
            let backend = get_backend();

            // initializes the backend using the lower-level rust sys crate
            let config: blitzar_sys::sxt_config = blitzar_sys::sxt_config {
                backend,
                num_precomputed_generators: config.num_precomputed_generators,
            };

            INIT_STATE = blitzar_sys::sxt_init(&config);
        });

        if INIT_STATE != 0 {
            panic!("Error during backend initialization");
        }
    };
}
