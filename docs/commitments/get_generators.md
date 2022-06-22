Gets the generators used in the `compute_commitments` function

In total, the function gets `generators.len()` points. These points
are converted from 1280-bit Curve25519 points used in the scalar multiplication
of the commitment computation, to 256-bit Ristretto points. This function
also allows the user to provide an offset so that a shift is applied in the
retrieval. The following operation is applied:

```text
for i in 0..generators.len() {
   generators[i] = randomly_generate_curve25519_point(i + offset).to_compressed_ristretto();
}
```

# Arguments

* `generators` - A sliced view of a CompressedRistretto memory area where the
              256-bit Ristretto Point generators used in the commitment computation will
              be written into.
* `offset_generators` - A value that is used to shift the get generator operation by
                        `offset_generators` values. With this shift, we have
                        generator\[0] holding the value of randomly_generate_curve25519_point(0 + offset),
                        generator\[1] holding the value of randomly_generate_curve25519_point(1 + offset),
                        and so on.

# Panics

If the compute `get_generators` execution in the GPU / CPU fails.

