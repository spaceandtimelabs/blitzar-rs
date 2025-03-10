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
use ark_ff::BigInteger256;
use core::mem;
use halo2curves::{
    bn256::{Fq as Halo2Bn256Fq, G1Affine as Halo2Bn256G1Affine},
    serde::SerdeObject,
};

fn convert_halo2_to_limbs(point: &Halo2Bn256Fq) -> [u64; 4] {
    let limbs: [u64; 4] = unsafe { mem::transmute(*point) };
    limbs
}

/// Converts a Halo2 BN256 G1 Affine point to an Arkworks BN254 G1 Affine point.
pub fn convert_to_ark_bn254_g1_affine(point: &Halo2Bn256G1Affine) -> Bn254G1Affine {
    let x_limbs: [u64; 4] = convert_halo2_to_limbs(&point.x);
    let y_limbs: [u64; 4] = convert_halo2_to_limbs(&point.y);

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

    let x_bytes = bytemuck::cast::<[u64; 4], [u8; 32]>(point.x.0 .0);
    let y_bytes = bytemuck::cast::<[u64; 4], [u8; 32]>(point.y.0 .0);

    Halo2Bn256G1Affine {
        x: Halo2Bn256Fq::from_raw_bytes_unchecked(&x_bytes),
        y: Halo2Bn256Fq::from_raw_bytes_unchecked(&y_bytes),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2curves::bn256::Fq as Halo2Bn256Fq;

    #[test]
    fn test_convert_halo2_modulus_to_limbs() {
        let expected: [u64; 4] = [
            4332616871279656263,
            10917124144477883021,
            13281191951274694749,
            3486998266802970665,
        ];
        let modulus = Halo2Bn256Fq::from_raw(expected);
        let point = convert_halo2_to_limbs(&modulus);
        assert_eq!(point, [0, 0, 0, 0]);
    }

    #[test]
    fn test_convert_halo2_one_to_one_in_montgomery_form_in_limbs() {
        let one: [u64; 4] = [1, 0, 0, 0];
        let one_in_mont = Halo2Bn256Fq::from_raw(one);
        let point = convert_halo2_to_limbs(&one_in_mont);

        let expected: [u64; 4] = [
            15230403791020821917,
            754611498739239741,
            7381016538464732716,
            1011752739694698287,
        ];

        assert_eq!(point, expected);
    }
}
