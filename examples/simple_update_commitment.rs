// Copyright 2023-present Space and Time Labs, Inc.
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
extern crate blitzar;
extern crate curve25519_dalek;

use blitzar::compute::*;
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;

fn main() {
    let dense_data: Vec<u32> = vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0];

    /////////////////////////////////////////////
    // This scalars_data will effectively only
    // have information in the positions 2 and 3
    /////////////////////////////////////////////
    let scalar_data: Vec<Scalar> = vec![Scalar::from(5000_u32), Scalar::from(1500_u32)];

    /////////////////////////////////////////////
    // We build the array with the expected results
    // expected_data = dense_data + scalar_data
    /////////////////////////////////////////////
    let expected_data: Vec<u32> = vec![1, 0, 5002, 1500, 3, 4, 0, 0, 0, 9, 0];

    let mut commitment = vec![CompressedRistretto::default(); 1];
    let mut expected_commitment = vec![CompressedRistretto::default(); 1];

    /////////////////////////////////////////////
    // We compute the commitments using the exact
    // data, which stores `dense_data + scalar_data`
    /////////////////////////////////////////////
    compute_commitments(&mut expected_commitment, &[(&expected_data).into()], 0_u64);

    /////////////////////////////////////////////
    // Up to this point, commitment was 0. Then
    // we update it, so that `commitment = dense_data`
    /////////////////////////////////////////////
    update_commitments(&mut commitment, &[(&dense_data).into()], 0_u64);

    /////////////////////////////////////////////
    // We then we update the commiment, so that
    // `commitment = dense_data + scalar_data`
    // Notice that we only pass the scalar values from 2 to 3.
    // Therefore, we need to specify the offsets
    // that will be used do query the correct generators.
    // For instance, the following is aplied:
    // commitment += (generator[0 + 2] * scalar_data[0] +
    //                  + generator[1 + 2] * scalar_data[1])
    /////////////////////////////////////////////
    update_commitments(&mut commitment, &[(&scalar_data).into()], 2_u64);

    /////////////////////////////////////////////
    // We then compare the commitment results
    /////////////////////////////////////////////
    if commitment == expected_commitment {
        println!("Commitments are equal: {:?}", commitment);
    } else {
        println!("Commitments are different:");
        println!("Actual Commitment 1: {:?}", commitment);
        println!("Expected Commitment 1: {:?}", expected_commitment);
    }
}
