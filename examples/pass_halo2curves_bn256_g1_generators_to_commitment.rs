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

use ark_bn254::{
    Fr as ArkBn254Fr, G1Affine as ArkBn254G1Affine, G1Projective as ArkBn254G1Projective,
};
use ark_ec::{CurveGroup, VariableBaseMSM};
use halo2curves::{
    bn256::{G1Affine as Halo2Bn256G1Affine, G1 as Halo2Bn256G1Projective},
    group::Curve,
};

extern crate blitzar;
use blitzar::compute::{
    compute_bn254_g1_uncompressed_commitments_with_halo2_generators, convert_to_ark_bn254_g1_affine,
};

fn main() {
    /////////////////////////////////////////////
    // For the following data, we have:
    //     commitment[0] = gs[0]*data[0] + gs[1]*data[1] + gs[2]*data[2] + gs[3]*data[3]
    //
    // Those generators `gs` are automatically generated by our CPU/GPU code.
    // So we provide an interface to access them. We use the offset to get only
    // a subset of the generators used in the gpu/cpu code.
    //
    // Alternatively, in this example, we provide a generator vector `gs`.
    /////////////////////////////////////////////
    let data: Vec<u16> = vec![2, 3, 1, 5, 4, 7, 6, 8, 9, 10];

    /////////////////////////////////////////////
    // randomly obtain the generator points
    /////////////////////////////////////////////
    let mut rng = rand::thread_rng();
    let generator_points: Vec<Halo2Bn256G1Affine> = (0..data.len())
        .map(|_| Halo2Bn256G1Affine::random(&mut rng))
        .collect();

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    let mut commitments = vec![Halo2Bn256G1Projective::default(); 1];
    compute_bn254_g1_uncompressed_commitments_with_halo2_generators(
        &mut commitments,
        &[(&data).into()],
        &generator_points,
    );

    /////////////////////////////////////////////
    // Then we use the above generators `gs`,
    // as well as the data as scalars
    // to verify that those generators `gs`
    // are indeed the ones used during the
    // commitment computation
    /////////////////////////////////////////////
    let scalar_data: Vec<ArkBn254Fr> = data.iter().map(|&d| ArkBn254Fr::from(d)).collect();

    let ark_generator_points: Vec<ArkBn254G1Affine> = generator_points
        .iter()
        .map(convert_to_ark_bn254_g1_affine)
        .collect();

    let ark_commitment = ArkBn254G1Projective::msm(&ark_generator_points, &scalar_data).unwrap();

    /////////////////////////////////////////////
    // Compare Arkworks and our CPU/GPU commitment
    /////////////////////////////////////////////
    let result_commitments: Vec<ArkBn254G1Affine> = commitments
        .iter()
        .map(|proj| proj.to_affine())
        .map(|affine| convert_to_ark_bn254_g1_affine(&affine))
        .collect();

    println!("Computed Commitment: {:?}\n", result_commitments[0]);
    println!("Expected Commitment: {:?}\n", ark_commitment.into_affine());
}
