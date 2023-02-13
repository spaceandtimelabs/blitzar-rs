Updates given commitments

This function updates the `commitments` data according to:

1. If the data input is either a slice view of a [crate::sequences::DenseSequence] or a slice view of a [curve25519_dalek::scalar::Scalar], then:

```text
let partial_commitments = vec![CompressedRistretto::zero(); data.len()];

for i in 0..data.len() {
    for j in 0..data[i].len() {
        partial_commitments[i] = partial_commitments[i] +
            generators[j + offset_generators] * data[i][j];
    }
    commitments[i] = commitments[i] + partial_commitments[i];
}
```

The `partial_commitments` is computed by [compute_commitments].

# Arguments

* `commitments` - A sliced view of a CompressedRistretto memory area where the 
               256-bit Ristretto point results will be written to. Please,
               you need to guarantee that this slice captures exactly
               data.len() element positions.

* `data` - A generic slice view T. Currently, we support
        two different types of slices. First, a slice view of a [crate::sequences::Sequence], 
        which captures the slices of contiguous u8 memory elements.
        In this case, you need to guarantee that the contiguous u8 slice view
        captures the correct amount of bytes that can reflect
        your desired amount of `num_rows` in the sequence. After all,
        we infer the `num_rows` from data\[i].data_slice.len() / data\[i].element_size.
        The second accepted data input is a slice view of a [curve25519_dalek::scalar::Scalar] memory area,
        which captures the slices of contiguous Dalek Scalar elements.

* `offset_generators` - A value that is used to shift the get generator operation by
                        `offset_generators` values. With this shift, we have
                        generator\[0] holding the value of randomly_generate_curve25519_point(0 + offset),
                        generator\[1] holding the value of randomly_generate_curve25519_point(1 + offset),
                        and so on.

# Asserts

If the data.len() value is different from the commitments.len() value.

# Panics

If the compute `compute_commitments` execution fails.
If the compute `compute_commitments_with_generators` execution fails.
If the compute `get_generators` execution in the GPU / CPU fails.
