### Examples
---------

All the examples are located in the `examples/` directory. Each one has its own `.rs` file. To run some examples, use the following command:

```
$ cargo run --features <cpu | gpu> --example <example_name>
```

In this command, you can either specify if the computations should proceed in the cpu or in the gpu. Also, you need to specify the example name as it is in the `.rs` files.

---------
#### Example 1 - Simple Commitment Computation
---------

Computes the $j$ commitment of each table column $j$ containing $m[j]$ rows, using for that the random generators $\bf{G}$ given by our blitzar code:

$$
commitment[j] = \displaystyle\sum_{i=0}^{m[j]-1}
{\bf{table}}[i][j] * {\bf{G}}[i].
$$

Use the following command to run the example in the cpu:

```
$ cargo run --features cpu --example simple_commitment
```

---------
#### Example 2 - Adding and Multiplying Commitments
---------

Suppose we have the following table:

<p align="center">

| Weekly Pay | Yearly Bonus |
|---|---|
| 2000 | 50000 |
| 7500 | 0     |
| 5000 | 400000 |
| 1500 | 0     |
</p>

Let $\bf{u}$ = [2000, 7500, 5000, 1500] be the weekly pay vector and and $\bf{v}$ = [50000, 0, 400000, 0] be the yearly bonus vector. We can compute the commitments to each column:

$C_u$ = 2000 * $g_0$ + 7500 * $g_1$ + 5000 * $g_2$ + 1500 * $g_3$

$C_v$ = 50000 * $g_0$ + 0 * $g_1$ + 400000 * $g_2$ + 0 * $g_3$

Recall, that $C_u$, $C_v$ ∈ G (the Ristretto group over curve25519) are both 32-bytes. At some point, we may wish to
compute the total compensation using the formula

<p align="center">
Total Compensation = 52 × Weekly Pay + Yearly Bonus
</p>

Let $\bf{w}$ = [154000, 390000, 660000, 78000] be the total compensation vector. Note, we can write $w = 52u + v$, which means that by the homomorphic property of the commitment,

$C_w$ = Commit(w) = Commit(52u + v) = 52 Commit(u) + Commit(v) = 52$C_u$ + $C_v$

Use the following command to run this example in the gpu:

```
$ cargo run --features gpu --example add_mult_commitments
```

---------
#### Example 3 - Initializing the Backend
---------

During the previous executions, you had to specify the backend where the computation must proceed - either `cpu` or `gpu`. Implicitly, those backends need to be initialized before the commitment computation is called. Inside this commitment function, we call the backend initialization. But this process takes time. So you may want to call this function at the beginning of your program so that you don't pay this price later. The following examples demonstrate this process:

```
$ cargo run --features gpu --example initialize_backend
```

---------
#### Example 4 - Get Generators
---------

This example shows how to fetch the ristretto point generators used in the commitment computation.

```
$ cargo run --features gpu --example get_generators
```

---------
#### Example 5 - Pass generators to Commitment Computation
---------

This example shows how to pass user-defined ristretto point generators to the commitment computation.

```
$ cargo run --features gpu --example pass_curve25519_generators_to_commitment
```

This example shows how to pass user-defined `bls12-381` `G1` point generators to the commitment computation.

```
$ cargo run --features gpu --example pass_bls12_381_g1_generators_to_commitment
```

This example shows how to pass user-defined `bn254` `G1` point generators to the commitment computation.

```
$ cargo run --features gpu --example pass_bn254_g1_generators_to_commitment
```

This example shows how to pass user-defined `grumpkin` point generators to the commitment computation.

```
$ cargo run --features gpu --example pass_grumpkin_generators_to_commitment
```

---------
#### Example 6 - Compute Commitments with Dalek Scalars
---------

This example shows how to compute commitments using Dalek scalars.

```
$ cargo run --features gpu --example simple_scalars_commitment
```

---------
#### Example 7 - Compute Commitments with Dalek Scalars and User Generators
---------

This example shows how to compute commitments using Dalek scalars and user generators.

```
$ cargo run --features gpu --example pass_generators_and_scalars_to_commitment
```

---------
#### Example 8 - Update Commitments Dense Sequences, and Dalek Scalars
---------

This example shows how to update a commitment using all the available data types.

```
$ cargo run --features gpu --example simple_update_commitment
```

---------
#### Example 9 - Get One Commits
---------

This example shows how to fetch the i-th one-commit ristretto point.

```
$ cargo run --features gpu --example get_one_commit
```
