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
use ark_bn254::{Fq as Bn254Fq, G1Affine as Bn254G1Affine, G1Projective as Bn254G1Projective};
use ark_ec::{AffineRepr, PrimeGroup};
use halo2curves::{
    bn256::{Fq as Halo2Bn256Fq, G1Affine as Halo2Bn256G1Affine, G1 as Halo2Bn256G1Projective},
    group::{cofactor::CofactorCurveAffine, Group},
};

#[test]
fn test_convert_bn254_g1_affine_generators_from_halo2_to_ark() {
    // Modulus taken from https://github.com/privacy-scaling-explorations/halo2curves/blob/3bfa6562f0ddcbac941091ba3c7c9b6c322efac1/src/bn256/fq.rs#L12
    let modulus: [u64; 4] = [
        4332616871279656263,
        10917124144477883021,
        13281191951274694749,
        3486998266802970665,
    ];

    let halo2_affine = [
        Halo2Bn256G1Affine::default(),
        Halo2Bn256G1Affine::generator(),
        Halo2Bn256G1Affine::identity(),
        Halo2Bn256G1Affine {
            x: Halo2Bn256Fq::from_raw(modulus).sub(&Halo2Bn256Fq::one()),
            y: Halo2Bn256Fq::from_raw(modulus).sub(&Halo2Bn256Fq::one()),
        },
        Halo2Bn256G1Affine {
            x: Halo2Bn256Fq::from_raw(modulus),
            y: Halo2Bn256Fq::from_raw(modulus),
        },
        Halo2Bn256G1Affine {
            x: Halo2Bn256Fq::from_raw(modulus).add(&Halo2Bn256Fq::one()),
            y: Halo2Bn256Fq::from_raw(modulus).add(&Halo2Bn256Fq::one()),
        },
    ];

    let halo2_to_ark = convert_bn254_g1_affine_generators_from_halo2_to_ark(&halo2_affine);

    assert_eq!(
        halo2_to_ark,
        [
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
        ],
    );
}

#[test]
fn test_convert_commitments_from_halo2_to_arkworks() {
    let halo2_projective = [Halo2Bn256G1Projective::default(); 1];
    let halo2_to_ark = convert_commitments_from_halo2_to_arkworks(&halo2_projective);
    assert_eq!(
        halo2_to_ark[0],
        Bn254G1Projective::default(),
        "Default projective points should be equal"
    );

    let halo2_projective = [Halo2Bn256G1Projective::generator(); 1];
    let halo2_to_ark = convert_commitments_from_halo2_to_arkworks(&halo2_projective);
    assert_eq!(
        halo2_to_ark[0],
        Bn254G1Projective::generator(),
        "Generator projective points should be equal"
    );

    let halo2_projective = [Halo2Bn256G1Projective::identity(); 1];
    let halo2_to_ark = convert_commitments_from_halo2_to_arkworks(&halo2_projective);
    let ark_identity_projective: Bn254G1Projective = Bn254G1Affine::identity().into();
    assert_eq!(
        halo2_to_ark[0], ark_identity_projective,
        "Identity projective points should be equal"
    );

    let halo2_projective = [
        Halo2Bn256G1Projective::default(),
        Halo2Bn256G1Projective::generator(),
        Halo2Bn256G1Projective::identity(),
    ];
    let halo2_to_ark = convert_commitments_from_halo2_to_arkworks(&halo2_projective);
    assert_eq!(
        halo2_to_ark,
        [
            Bn254G1Projective::default(),
            Bn254G1Projective::generator(),
            ark_identity_projective
        ],
        "Projective points should be equal"
    );
}

#[test]
fn test_convert_commitments_from_ark_to_halo2() {
    let mut ark_to_halo2 = [Halo2Bn256G1Projective::default(); 3];

    let ark_default = [Bn254G1Affine::default(); 1];
    convert_commitments_from_ark_to_halo2(&mut ark_to_halo2, &ark_default);
    assert_eq!(
        ark_to_halo2[0],
        Halo2Bn256G1Projective::default(),
        "Default affine to projective point should be equal"
    );

    let ark_generator = [Bn254G1Affine::generator(); 1];
    convert_commitments_from_ark_to_halo2(&mut ark_to_halo2, &ark_generator);
    assert_eq!(
        ark_to_halo2[0],
        Halo2Bn256G1Projective::generator(),
        "Generator affine to projective point should be equal"
    );

    let ark_identity = [Bn254G1Affine::identity(); 1];
    convert_commitments_from_ark_to_halo2(&mut ark_to_halo2, &ark_identity);
    assert_eq!(
        ark_to_halo2[0],
        Halo2Bn256G1Projective::identity(),
        "Identity affine to projective point should be equal"
    );

    let ark_points = [
        Bn254G1Affine::default(),
        Bn254G1Affine::generator(),
        Bn254G1Affine::identity(),
    ];
    convert_commitments_from_ark_to_halo2(&mut ark_to_halo2, &ark_points);
    assert_eq!(
        ark_to_halo2,
        [
            Halo2Bn256G1Projective::default(),
            Halo2Bn256G1Projective::generator(),
            Halo2Bn256G1Projective::identity()
        ],
        "Affine to projective points should be equal"
    );
}
