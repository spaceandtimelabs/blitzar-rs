Responsible for initializing only once the C++ commitment backend.

This function initializes the backend so that computations
can proceed either in the CPU or in the GPU. The backend
type must be specified during `build` time using the following flags:

```text
cargo build --features pip-cpu # only the pippenger commitment CPU is used
cargo build --features naive-gpu # only the naive commitment GPU is used
cargo build --features naive-cpu # only the naive commitment CPU is used
```

Once the backend is initialized, it is not possible to change to
another one. Therefore, if the GPU feature is specified during build time,
then it is not possible to use the CPU to do the computations. 
In the case, no feature is specified during build time, 
the backend will initialize with the pip-cpu as default.

During this initialization process, the user can also specify a
`num_precomputed_generators` value, which is used to pre-generate
some generators. Those are later used in the commitment computation,
preventing the generators from being created over and over again.

//
Also, any `compute` function will call this `init_backend_with_precomputation`
securing that the backend is always in a proper state.
 
Finally, to guarantee that the code inside this function is not
initialized multiple times, we use the `std::sync::Once` scheme.

# Arguments

* `num_precomputed_generators` - The total number of generators to be precomputed.
Those are used later during the commitment computation. Pre-computing may be beneficial,
as it can save computational time.

# Panics

If the backend initialization fails.
