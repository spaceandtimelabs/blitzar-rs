Updates a given commitment

This function updates the `commitments` data according to:

1. If the data input is either a [crate::sequences::DenseSequence] or a [curve25519_dalek::scalar::Scalar], then:

```text
let partial_commitments = CompressedRistretto::zero();

for i in 0..data.len() {
    partial_commitments = partial_commitments +
                generators[i + offset_generators] * data[i];
}

commitments = commitments + partial_commitments;
```

The `partial_commitments` is computed by [compute_commitments].

# Arguments

* `commitments` - A reference of a CompressedRistretto data where the 
               256-bit Ristretto point results will be read from and then
               written to.

* `offset_generators` - A value that is used to shift the get generator operation by
                        `offset_generators` values. With this shift, we have
                        generator\[0] holding the value of randomly_generate_curve25519_point(0 + offset),
                        generator\[1] holding the value of randomly_generate_curve25519_point(1 + offset),
                        and so on.

* `data` - A generic slice view T. Currently, we support
        two different types of data types. First, a [crate::sequences::Sequence], 
        which captures the slices of contiguous u8 memory elements.
        In this case, you need to guarantee that the contiguous u8 slice view
        captures the correct amount of bytes that can reflect
        your desired amount of `num_rows` in the sequence. After all,
        we infer the `num_rows` from data\[i].data_slice.len() / data\[i].element_size.
        Finally, the second accepted data input is a [curve25519_dalek::scalar::Scalar] data,
        which captures the slices of contiguous Dalek Scalar elements.

# Panics

If the compute `compute_commitments` execution fails.
If the compute `compute_commitments_with_generators` execution fails.
If the compute `get_generators` execution in the GPU / CPU fails.
