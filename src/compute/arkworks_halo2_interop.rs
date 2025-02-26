// Copyright 2025-present Space and Time Labs, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ark_bn254::{Fq as Bn254Fq, G1Affine as Bn254G1Affine};
use ark_ff::{BigInteger256, PrimeField};
use halo2curves::{
    bn256::{Fq as Halo2Bn256Fq, G1Affine as Halo2Bn256G1Affine},
    serde::SerdeObject,
};

/// Converts a Halo2 BN256 G1 Affine point to an Arkworks BN254 G1 Affine point.
pub fn convert_to_ark_bn254_g1_affine(point: &Halo2Bn256G1Affine) -> Bn254G1Affine {
    let x_bytes: [u8; 32] = point.x.to_raw_bytes().try_into().unwrap();
    let y_bytes: [u8; 32] = point.y.to_raw_bytes().try_into().unwrap();

    let x_limbs = bytemuck::cast::<[u8; 32], [u64; 4]>(x_bytes);
    let y_limbs = bytemuck::cast::<[u8; 32], [u64; 4]>(y_bytes);

    Bn254G1Affine {
        x: Bn254Fq::new_unchecked(BigInteger256::new(x_limbs)),
        y: Bn254Fq::new_unchecked(BigInteger256::new(y_limbs)),
        infinity: *point == Halo2Bn256G1Affine::default(),
    }
}

/// Converts an Arkworks BN254 G1 Affine point to a Halo2 BN256 G1 Affine point.
pub fn convert_to_halo2_bn256_g1_affine(point: &Bn254G1Affine) -> Halo2Bn256G1Affine {
    if point.infinity {
        return Halo2Bn256G1Affine::default();
    }

    let x_bytes = bytemuck::cast::<[u64; 4], [u8; 32]>(point.x.into_bigint().0);
    let y_bytes = bytemuck::cast::<[u64; 4], [u8; 32]>(point.y.into_bigint().0);

    Halo2Bn256G1Affine {
        x: Halo2Bn256Fq::from_bytes(&x_bytes).unwrap(),
        y: Halo2Bn256Fq::from_bytes(&y_bytes).unwrap(),
    }
}
