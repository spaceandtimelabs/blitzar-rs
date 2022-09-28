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

2. If the data input is a [crate::sequences::SparseSequence], then:

```text
let partial_commitments = CompressedRistretto::zero();

for i in 0..data.len() {
    partial_commitments = partial_commitments +
                generators[data.data_indices[i]] * data.data_slice[i];
}

commitments = commitments + partial_commitments;
```

The `partial_commitments` is accomplished by the [compute_commitments]
function when the data input is a [crate::sequences::SparseSequence], and by the
[get_generators] and the [compute_commitments_with_generators] when the data input is a
[crate::sequences::DenseSequence] or a [curve25519_dalek::scalar::Scalar]. In this last case, we use the [get_generators]
function, passing to it the `offset_generators`, so that we can get the
exact generators used in the partial commitment computation.

Also, it is important to highlight that the above is an oversimplification to provide
you with a general idea. The real code is more verbose than that and uses the exact fields of the 
struct.

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
        Also, [crate::sequences::Sequence] can be either a [crate::sequences::DenseSequence] or a [crate::sequences::SparseSequence].
        In the case it is a [crate::sequences::SparseSequence], the `offset_generators` value is ignored
        and the indices vector in the [crate::sequences::SparseSequence] is used to query the correct
        generators. Finally, the second accepted data input is a [curve25519_dalek::scalar::Scalar] data,
        which captures the slices of contiguous Dalek Scalar elements.

# Panics

If the compute `compute_commitments` execution fails.
If the compute `compute_commitments_with_generators` execution fails.
If the compute `get_generators` execution in the GPU / CPU fails.
