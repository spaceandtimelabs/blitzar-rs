Gets the n-th ristretto point defined as:

```text
if n == 0 {
    one_commit = curve25519_dalek::ristretto::RistrettoPoint::identity();
} else {
    one_commit[0] = g[0] + g[1] + ... + g[n - 1];
}
```

where `g[i]` is the i-th generator provided by `get_generators` function at the offset 0.

# Arguments:

* `n` - the n-th ristretto point to be fetched.

# Return:

The n-th ristretto point defined as above.

# Panics

If the compute `get_one_commit` execution in the GPU / CPU fails.

