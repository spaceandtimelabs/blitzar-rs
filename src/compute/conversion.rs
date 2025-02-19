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

use ark_bn254::{G1Affine as Bn254G1Affine, G1Projective as Bn254G1Projective};
use ark_ec::CurveGroup;
use ark_ff::{BigInt, PrimeField};
use halo2curves::bn256::{
    Fq as Halo2Bn256Fq, G1Affine as Halo2Bn256G1Affine, G1 as Halo2Bn256G1Projective,
};

fn convert_bn254_g1_affine_point_from_halo2_to_ark(point: &Halo2Bn256G1Affine) -> Bn254G1Affine {
    if *point == Halo2Bn256G1Affine::default() {
        return Bn254G1Affine::default();
    }

    Bn254G1Affine {
        x: BigInt::<4>::new(bytemuck::cast(point.x.to_bytes())).into(),
        y: BigInt::<4>::new(bytemuck::cast(point.y.to_bytes())).into(),
        infinity: false,
    }
}

/// Converts a slice of Halo2 bn256 G1 affine points to a vector of Arkworks bn254 G1 affine points
#[tracing::instrument(level = "debug", skip_all)]
pub fn convert_bn254_g1_affine_generators_from_halo2_to_ark(
    generators: &[Halo2Bn256G1Affine],
) -> Vec<Bn254G1Affine> {
    generators
        .iter()
        .map(convert_bn254_g1_affine_point_from_halo2_to_ark)
        .collect::<Vec<Bn254G1Affine>>()
}

fn convert_bn254_g1_projective_from_halo2_to_ark(
    point: &Halo2Bn256G1Projective,
) -> Bn254G1Projective {
    Bn254G1Projective {
        x: BigInt::<4>::new(bytemuck::cast(point.x.to_bytes())).into(),
        y: BigInt::<4>::new(bytemuck::cast(point.y.to_bytes())).into(),
        z: BigInt::<4>::new(bytemuck::cast(point.z.to_bytes())).into(),
    }
}

/// Converts a slice of Halo2 bn256 G1 projective points to a vector of Arkworks bn254 G1 affine points
#[tracing::instrument(level = "debug", skip_all)]
pub fn convert_commitments_from_halo2_to_arkworks(
    commitments: &[Halo2Bn256G1Projective],
) -> Vec<Bn254G1Affine> {
    commitments
        .iter()
        .map(convert_bn254_g1_projective_from_halo2_to_ark)
        .map(|proj| proj.into_affine())
        .collect::<Vec<Bn254G1Affine>>()
}

fn convert_bn254_g1_point_from_ark_affine_to_halo2_projective(
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

/// Maps a slice of Arkworks bn254 G1 affine points to a mutable slice of Halo2 bn256 G1 projective points
#[tracing::instrument(level = "debug", skip_all)]
pub fn convert_commitments_from_ark_to_halo2(
    commitments: &mut [Halo2Bn256G1Projective],
    ark_commitments: &[Bn254G1Affine],
) {
    commitments
        .iter_mut()
        .zip(ark_commitments)
        .for_each(|(c_a, c_b)| {
            *c_a = convert_bn254_g1_point_from_ark_affine_to_halo2_projective(c_b);
        });
}
