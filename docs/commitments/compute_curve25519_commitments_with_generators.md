Computes the Pedersen commitment for a given input data using `curve25519` elements.

In total, the function computes `data.len()` commitments,
which is related to the total number of columns in the data table. The commitment
results are stored as 256-bit Ristretto points in the `commitments` variable.

The `j`-th Pedersen commitment is a 256-bit Ristretto point `C_j` over the
`curve25519` elliptic curve that is cryptographically binded to a data message vector `M_j`. This `M_j` vector is populated according to the type of the `data` given.

For instance, for an input data table specified as a [crate::sequence::Sequence] slice view, we populate `M_j` as follows:

```text
let el_size = data[j].element_size; // sizeof of each element in the current j-th column
let num_rows = data[j].data_slice.len() / el_size; // number of rows in the j-th column

let M_j = [
   data[j].data_slice[0:el_size],
   data[j].data_slice[el_size:2*el_size],
   data[j].data_slice[2*el_size:3*el_size],
   .,
   .,
   .,
   data[j].data_slice[(num_rows-1)*el_size:num_rows*el_size]
];
```

Now, for an input data table specified as a [curve25519_dalek::scalar::Scalar] view, we populate `M_j` as follows:

```text
let el_size = 32; // sizeof of each element in the current j-th column
let num_rows = data[j].len(); // number of rows in the j-th column

let M_j = [
   data[j][0],
   data[j][1],
   data[j][2],
   .,
   .,
   .,
   data[j][num_rows - 1]
];
```

This message `M_j` cannot be decrypted from `C_j`. The curve point `C_j`
is generated in a unique way using `M_j` and a
set of 1280-bit `curve25519` points `G_i`, called row generators.
Although our GPU code uses 1280-bit generators during the scalar 
multiplication, these generators are passed as 256-bit Ristretto points
and only converted to 1280-bit points inside the GPU/CPU.
The total number of generators used to compute `C_j` is equal to 
the number of `num_rows` in the `data[j]` sequence. The following formula
is specified to obtain the `C_j` commitment when the input table is a 
[crate::sequence::Sequence] view:

```text
let C_j_temp = 0; // this is a 1280-bit curve25519 point

for j in 0..num_rows {
    let G_i = generators[j].decompress(); // we decompress to convert 256-bit to 1280-bit points
    let curr_data_ji = data[j].data_slice[i*el_size:(i + 1)*el_size];
    C_j_temp = C_j_temp + curr_data_ji * G_i;
}

let C_j = convert_to_ristretto(C_j_temp); // this is a 256-bit Ristretto point
```

When we have a [curve25519_dalek::scalar::Scalar] view for the input table, we use the following formula:

```text
let C_j_temp = 0; // this is a 1280-bit curve25519 point

for j in 0..num_rows {
    let G_i = get_random_ristretto_point(j);
    let curr_data_ji = data[j][i];
    C_j_temp = C_j_temp + curr_data_ji * G_i;
}

let C_j = convert_to_ristretto(C_j_temp); // this is a 256-bit Ristretto point
```

Ps: the above is only illustrative code. It will not compile.

Here `curr_data_ji` are simply 256-bit scalars, `C_j_temp` and `G_i` are
1280-bit `curve25519` points and `C_j` is a 256-bit Ristretto point.

Given `M_j` and `G_i`, it is easy to verify that the Pedersen
commitment `C_j` is the correctly generated output. However,
the Pedersen commitment generated from `M_j` and `G_i` is cryptographically
binded to the message `M_j` because finding alternative inputs `M_j*` and 
`G_i*` for which the Pedersen commitment generates the same point `C_j`
requires an infeasible amount of computation.

To guarantee proper execution, so that the backend is correctly set,
this `compute_curve25519_commitments` always calls the `init_backend()` function.

Portions of this documentation were extracted from
[here](https://findora.org/faq/crypto/pedersen-commitment-with-elliptic-curves/)

# Arguments

* `commitments` - A sliced view of a `CompressedRistretto` memory area where the 
               256-bit Ristretto point results will be written to. Please,
               you need to guarantee that this slice captures exactly
               data.len() element positions.

* `data` - A generic sliced view `T`. Currently, we support
        two different types of slices. First, is a slice view of a [crate::sequence::Sequence], 
        which captures the slices of contiguous `u8` memory elements.
        In this case, you need to guarantee that the contiguous `u8` slice view
        captures the correct amount of bytes that can reflect
        your desired amount of `num_rows` in the sequence. After all,
        we infer the `num_rows` from `data[i].data_slice.len() / data[i].element_size`.
        The second accepted data input is a slice view of a [curve25519_dalek::scalar::Scalar] memory area,
        which captures the slices of contiguous Dalek Scalar elements.

* `generators` - A sliced view of a Ristretto memory area where the
              256-bit Ristretto Point generators used in the commitment computation are
              stored. Bear in mind that the size of this slice must always be greater
              or equal to the longest sequence, in terms of rows, in the table.

# Asserts

If the longest sequence in the input data is bigger than the generators length, or if
the `data.len()` value is different from the `commitments.len()` value.

# Panics

If the compute commitments execution in the GPU / CPU fails.
