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

use ark_bn254::{Fq as Bn254Fq, G1Affine as Bn254G1Affine, G1Projective as Bn254G1Projective};
use ark_ff::PrimeField;
use halo2curves::bn256::{
    Fq as Halo2Bn256Fq, G1Affine as Halo2Bn256G1Affine, G1 as Halo2Bn256G1Projective,
};

/// Convert a Halo2 Bn256 G1 affine point to an Arkworks Bn254 G1 affine point
pub fn convert_halo2_to_ark_bn254_g1_affine(point: &Halo2Bn256G1Affine) -> Bn254G1Affine {
    if *point == Halo2Bn256G1Affine::default() {
        return Bn254G1Affine::default();
    }
    Bn254G1Affine::new(
        Bn254Fq::from_le_bytes_mod_order(&point.x.to_bytes()),
        Bn254Fq::from_le_bytes_mod_order(&point.y.to_bytes()),
    )
}

/// Convert a Halo2 Bn256 G1 projective point to an Arkworks Bn254 G1 projective point
pub fn convert_halo2_to_ark_bn254_g1_projective(
    point: &Halo2Bn256G1Projective,
) -> Bn254G1Projective {
    Bn254G1Projective::new(
        Bn254Fq::from_le_bytes_mod_order(&point.x.to_bytes()),
        Bn254Fq::from_le_bytes_mod_order(&point.y.to_bytes()),
        Bn254Fq::from_le_bytes_mod_order(&point.z.to_bytes()),
    )
}

/// Convert an Arkworks Bn254 G1 affine point to a Halo2 Bn256 G1 projective point
pub fn convert_ark_affine_to_halo2_projective_bn254_g1(
    point: &Bn254G1Affine,
) -> Halo2Bn256G1Projective {
    let x_bytes: [u8; 32] = bytemuck::cast(point.x.into_bigint().0);
    let y_bytes: [u8; 32] = bytemuck::cast(point.y.into_bigint().0);
    let z = if point.infinity {
        Halo2Bn256Fq::zero()
    } else {
        Halo2Bn256Fq::one()
    };
    Halo2Bn256G1Projective {
        x: Halo2Bn256Fq::from_bytes(&x_bytes).unwrap(),
        y: Halo2Bn256Fq::from_bytes(&y_bytes).unwrap(),
        z,
    }
}
