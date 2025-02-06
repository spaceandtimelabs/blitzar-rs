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
use ark_bn254::{G1Affine as Bn254G1Affine, G1Projective as Bn254G1Projective};
use ark_ec::{AffineRepr, PrimeGroup};
use halo2curves::{
    bn256::{G1Affine as Halo2Bn256G1Affine, G1 as Halo2Bn256G1Projective},
    group::{cofactor::CofactorCurveAffine, Group},
};

#[test]
fn test_convert_halo2_to_ark_bn254_g1_affine() {
    let halo2_affine = Halo2Bn256G1Affine::default();
    let halo2_to_ark = convert_halo2_to_ark_bn254_g1_affine(&halo2_affine);
    assert_eq!(
        halo2_to_ark,
        Bn254G1Affine::default(),
        "Default affine points should be equal"
    );

    let halo2_affine = Halo2Bn256G1Affine::generator();
    let halo2_to_ark = convert_halo2_to_ark_bn254_g1_affine(&halo2_affine);
    assert_eq!(
        halo2_to_ark,
        Bn254G1Affine::generator(),
        "Generator affine points should be equal"
    );

    let halo2_affine = Halo2Bn256G1Affine::identity();
    let halo2_to_ark = convert_halo2_to_ark_bn254_g1_affine(&halo2_affine);
    assert_eq!(
        halo2_to_ark,
        Bn254G1Affine::identity(),
        "Identity affine points should be equal"
    );
}

#[test]
fn test_convert_halo2_to_ark_bn254_g1_projective() {
    let halo2_projective = Halo2Bn256G1Projective::default();
    let halo2_to_ark = convert_halo2_to_ark_bn254_g1_projective(&halo2_projective);
    assert_eq!(
        halo2_to_ark,
        Bn254G1Projective::default(),
        "Default projective points should be equal"
    );

    let halo2_projective = Halo2Bn256G1Projective::generator();
    let halo2_to_ark = convert_halo2_to_ark_bn254_g1_projective(&halo2_projective);
    assert_eq!(
        halo2_to_ark,
        Bn254G1Projective::generator(),
        "Generator projective points should be equal"
    );

    let halo2_projective = Halo2Bn256G1Projective::identity();
    let halo2_to_ark = convert_halo2_to_ark_bn254_g1_projective(&halo2_projective);
    let ark_identity_projective: Bn254G1Projective = Bn254G1Affine::identity().into();
    assert_eq!(
        halo2_to_ark, ark_identity_projective,
        "Identity projective points should be equal"
    );
}

#[test]
fn test_convert_ark_affine_to_halo2_projective_bn254_g1() {
    let ark_default = Bn254G1Affine::default();
    let ark_to_halo2 = convert_ark_affine_to_halo2_projective_bn254_g1(&ark_default);
    assert_eq!(
        ark_to_halo2,
        Halo2Bn256G1Projective::default(),
        "Default affine to projective point should be equal"
    );

    let ark_generator = Bn254G1Affine::generator();
    let ark_to_halo2 = convert_ark_affine_to_halo2_projective_bn254_g1(&ark_generator);
    assert_eq!(
        ark_to_halo2,
        Halo2Bn256G1Projective::generator(),
        "Generator affine to projective point should be equal"
    );

    let ark_identity = Bn254G1Affine::identity();
    let ark_to_halo2 = convert_ark_affine_to_halo2_projective_bn254_g1(&ark_identity);
    assert_eq!(
        ark_to_halo2,
        Halo2Bn256G1Projective::identity(),
        "Identity affine to projective point should be equal"
    );
}
