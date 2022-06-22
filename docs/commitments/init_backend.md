Responsible for initializing only once the C++ commitment backend.

This function initializes the backend so that computations
can proceed either in the CPU or in the GPU. The backend
the type must be specified during `build` time using the following flags:

```text
cargo build --features gpu # only the GPU is used
cargo build --features cpu # only the CPU is used
```

Once the backend is initialized, it is not possible to change to
another one. Therefore, if the GPU feature is specified during build time,
then it is not possible to use the CPU to do the computations. 
In the case, no feature is specified during build time, 
the backend will initialize with the GPU as default.
//
Also, any `compute` function will call this `init_backend`
securing that the GPU is always in a proper state. However,
this last case may introduce some initialization overhead that could
have been overlapped with CPU computation (such as reading data from a database).
 
Finally, to guarantee that the code inside this function is not
initialized multiple times, we use the `std::sync::Once` scheme.

# Panics

If the backend initialization fails.
