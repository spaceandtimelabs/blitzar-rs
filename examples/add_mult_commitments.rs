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
use curve25519_dalek::scalar::Scalar;

fn main() {
    // generate input table
    let num_weeks: u64 = 52;

    /////////////////////////////////////////////
    // Define the data slices that will be used in the computation.
    /////////////////////////////////////////////
    let weekly_pay_data: &[u16] = &[2000, 7500, 5000, 1500];

    let yearly_bonus_data: &[u32] = &[5000, 0, 400000, 10, 0, 0];

    let total_compensation_data: &[u64] = &[
        num_weeks * 2000 + 5000,
        num_weeks * 7500,
        num_weeks * 5000 + 400000,
        num_weeks * 1500 + 10,
    ];

    /////////////////////////////////////////////
    // Do the actual commitment computation (either in cpu / gpu)
    /////////////////////////////////////////////
    let mut commitments = vec![Default::default(); 3];
    compute_curve25519_commitments(
        &mut commitments,
        &[
            weekly_pay_data.into(),
            yearly_bonus_data.into(),
            total_compensation_data.into(),
        ],
        0,
    );

    /////////////////////////////////////////////
    // Converts the commitment results to uncompressed form
    /////////////////////////////////////////////
    let commit_weekly_pay = commitments[0].decompress().unwrap();
    let commit_yearly_bonus = commitments[1].decompress().unwrap();
    let commit_total_compensation = commitments[2].decompress().unwrap();

    /////////////////////////////////////////////
    // Compares if the `commit_total_compensation`
    // is equal to `52 * commit_weekly_pay + commit_yearly_bonus`
    // This is true because the commitments are homomorphic
    /////////////////////////////////////////////
    assert_eq!(
        Scalar::from(num_weeks) * commit_weekly_pay + commit_yearly_bonus,
        commit_total_compensation
    );
}
