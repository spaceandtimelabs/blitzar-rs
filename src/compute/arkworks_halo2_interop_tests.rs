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

use super::*;
use ark_bn254::{Fq as Bn254Fq, G1Affine as Bn254G1Affine};
use ark_ec::AffineRepr;
use halo2curves::{
    bn256::{Fq as Halo2Bn256Fq, G1Affine as Halo2Bn256G1Affine},
    group::cofactor::CofactorCurveAffine,
};

// Modulus taken from https://github.com/privacy-scaling-explorations/halo2curves/blob/3bfa6562f0ddcbac941091ba3c7c9b6c322efac1/src/bn256/fq.rs#L12
const MODULUS: [u64; 4] = [
    4332616871279656263,
    10917124144477883021,
    13281191951274694749,
    3486998266802970665,
];

#[test]
fn test_convert_points_from_halo2_bn256_g1_affine_to_ark_bn254_g1_affine() {
    let halo2_affine = [
        Halo2Bn256G1Affine::default(),
        Halo2Bn256G1Affine::generator(),
        Halo2Bn256G1Affine::identity(),
        Halo2Bn256G1Affine {
            x: Halo2Bn256Fq::from_raw(MODULUS).sub(&Halo2Bn256Fq::one()),
            y: Halo2Bn256Fq::from_raw(MODULUS).sub(&Halo2Bn256Fq::one()),
        },
        Halo2Bn256G1Affine {
            x: Halo2Bn256Fq::from_raw(MODULUS),
            y: Halo2Bn256Fq::from_raw(MODULUS),
        },
        Halo2Bn256G1Affine {
            x: Halo2Bn256Fq::from_raw(MODULUS).add(&Halo2Bn256Fq::one()),
            y: Halo2Bn256Fq::from_raw(MODULUS).add(&Halo2Bn256Fq::one()),
        },
    ];

    let expected = [
        Bn254G1Affine::default(),
        Bn254G1Affine::generator(),
        Bn254G1Affine::identity(),
        Bn254G1Affine {
            x: Bn254Fq::from(-1),
            y: Bn254Fq::from(-1),
            infinity: false,
        },
        Bn254G1Affine {
            x: Bn254Fq::from(0),
            y: Bn254Fq::from(0),
            infinity: true,
        },
        Bn254G1Affine {
            x: Bn254Fq::from(1),
            y: Bn254Fq::from(1),
            infinity: false,
        },
    ];

    for (halo2, ark) in halo2_affine.iter().zip(expected.iter()) {
        let converted = convert_to_ark_bn254_g1_affine(halo2);
        assert_eq!(converted, *ark);
    }
}

#[test]
fn test_convert_ark_bn254_g1_affine_to_halo2_bn256_g1_affine() {
    let ark_affine = [
        Bn254G1Affine::default(),
        Bn254G1Affine::generator(),
        Bn254G1Affine::identity(),
        Bn254G1Affine {
            x: Bn254Fq::from(-1),
            y: Bn254Fq::from(-1),
            infinity: false,
        },
        Bn254G1Affine {
            x: Bn254Fq::from(0),
            y: Bn254Fq::from(0),
            infinity: true,
        },
        Bn254G1Affine {
            x: Bn254Fq::from(1),
            y: Bn254Fq::from(1),
            infinity: false,
        },
    ];

    let expected = [
        Halo2Bn256G1Affine::default(),
        Halo2Bn256G1Affine::generator(),
        Halo2Bn256G1Affine::identity(),
        Halo2Bn256G1Affine {
            x: Halo2Bn256Fq::from_raw(MODULUS).sub(&Halo2Bn256Fq::one()),
            y: Halo2Bn256Fq::from_raw(MODULUS).sub(&Halo2Bn256Fq::one()),
        },
        Halo2Bn256G1Affine {
            x: Halo2Bn256Fq::from_raw(MODULUS),
            y: Halo2Bn256Fq::from_raw(MODULUS),
        },
        Halo2Bn256G1Affine {
            x: Halo2Bn256Fq::from_raw(MODULUS).add(&Halo2Bn256Fq::one()),
            y: Halo2Bn256Fq::from_raw(MODULUS).add(&Halo2Bn256Fq::one()),
        },
    ];

    for (ark, halo2) in ark_affine.iter().zip(expected.iter()) {
        let converted = convert_to_halo2_bn256_g1_affine(ark);
        assert_eq!(converted, *halo2);
    }
}
