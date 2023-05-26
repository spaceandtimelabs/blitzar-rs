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
use crate::sequences::DenseSequence;
use ark_ff::{Fp, FpConfig, PrimeField};

/// This `DenseSequenceData` stores the data
/// of a contiguous column data table.
/// This is in contrast to `DenseSequence` which
/// simply stores the slice view. This gives the ability
/// to convert to a `DenseSequence` where `T` cannot simply
/// be cast via something like `byte-slice-cast`.
pub struct DenseSequenceData {
    data: Vec<u8>,
    element_size: usize,
}

impl<'a> From<&'a DenseSequenceData> for DenseSequence<'a> {
    fn from(value: &'a DenseSequenceData) -> Self {
        DenseSequence {
            data_slice: &value.data,
            element_size: value.element_size,
        }
    }
}

#[cfg(target_endian = "little")]
impl<P: FpConfig<N>, const N: usize> From<&[Fp<P, N>]> for DenseSequenceData {
    fn from(value: &[Fp<P, N>]) -> Self {
        let data = value
            .iter()
            .flat_map(|s| {
                s.into_bigint()
                    .0
                    .into_iter()
                    .flat_map(|limb| limb.to_le_bytes())
            })
            .collect();
        DenseSequenceData {
            data,
            element_size: N * 8,
        }
    }
}

#[cfg(test)]
#[cfg(target_endian = "little")]
mod test {
    use crate::sequences::{DenseSequence, DenseSequenceData};
    use ark_curve25519::Fr;
    use ark_ff::{BigInteger, PrimeField};
    use curve25519_dalek::scalar::Scalar;
    #[test]
    fn we_can_convert_an_empty_slice_of_ark_scalars_to_a_dense_sequence() {
        let s = Vec::<Fr>::new();
        let dsd = DenseSequenceData::from(&s[..]);
        let d = DenseSequence::from(&dsd);
        assert_eq!(d.element_size, 4 * 8);
        assert!(d.is_empty());
    }

    #[test]
    fn we_can_convert_a_slice_of_ark_scalars_to_a_dense_sequence_with_correct_data() {
        let s = vec![Fr::from(123), Fr::from(-456), Fr::from(789)];
        let dsd = DenseSequenceData::from(&s[..]);
        let d = DenseSequence::from(&dsd);
        assert_eq!(d.element_size, 4 * 8);
        assert_eq!(d.len(), 3);

        assert_eq!(
            d.data_slice[0..d.element_size],
            Fr::from(123).into_bigint().to_bytes_le()
        );
        assert_eq!(
            d.data_slice[d.element_size..2 * d.element_size],
            Fr::from(-456).into_bigint().to_bytes_le()
        );
        assert_eq!(
            d.data_slice[2 * d.element_size..3 * d.element_size],
            Fr::from(789).into_bigint().to_bytes_le()
        );
    }

    #[test]
    fn we_can_convert_slices_of_ark_and_dalek_scalars_to_dense_sequences_with_the_same_data() {
        let s_ark = vec![Fr::from(123), Fr::from(-456), Fr::from(789)];
        let dsd_ark = DenseSequenceData::from(&s_ark[..]);
        let d_ark = DenseSequence::from(&dsd_ark);

        let s_dalek = vec![
            Scalar::from(123u32),
            -Scalar::from(456u32),
            Scalar::from(789u32),
        ];
        let d_dalek = DenseSequence::from(&s_dalek[..]);

        assert_eq!(d_ark.element_size, d_dalek.element_size);
        assert_eq!(d_ark.len(), d_dalek.len());
        assert_eq!(d_ark.data_slice, d_dalek.data_slice);
    }
}
