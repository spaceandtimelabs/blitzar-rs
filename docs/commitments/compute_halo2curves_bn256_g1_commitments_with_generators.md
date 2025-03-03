Computes the Pedersen commitment for a given input data using `Halo2 curves bn256 G1` curve elements.

In total, the function computes `data.len()` commitments,
which is related to the total number of columns in the data table. The commitment
results are stored as 768-bit `Halo2 curves bn256 G1` curve point in projective form in the `commitments` variable.

The `compute_bn254_g1_uncompressed_commitments_with_halo2_generators` function wraps the `compute_bn254_g1_uncompressed_commitments_with_generators` function. 
Internally the `Halo2 curve` commitment and generator points are converted to and from `Arkworks` points before the Pedersen commitment computation.

For more information about how Pedersen commitments are computed, see [compute_bn254_g1_commitments_with_generators.md](compute_bn254_g1_commitments_with_generators.md).


# Arguments

* `commitments` - A sliced view of an uncompressed `Halo2 curves bn256 G1` curve element memory area where the 
               768-bit point results will be written to. Please,
               you need to guarantee that this slice captures exactly
               `data.len()` element positions.

* `data` - A generic sliced view `T` of a [crate::sequence::Sequence], 
        which captures the slices of contiguous `u8` memory elements.
        You need to guarantee that the contiguous `u8` slice view
        captures the correct amount of bytes that can reflect
        your desired amount of `num_rows` in the sequence. After all,
        we infer the `num_rows` from `data[i].data_slice.len() / data[i].element_size`.

* `generators` - A sliced view of a `Halo2 curves bn256 G1` curve affine element memory area where the
              512-bit point generators used in the commitment computation are
              stored. Bear in mind that the size of this slice must always be greater
              or equal to the longest sequence, in terms of rows, in the table.

# Asserts

If the longest sequence in the input data is bigger than the generators length, or if
the `data.len()` value is different from the `commitments.len()` value.

# Panics

If the compute commitments execution in the GPU / CPU fails.
