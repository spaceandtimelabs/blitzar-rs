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

//! data and scalar field elements for data table

use blitzar_sys::sxt_sequence_descriptor;

/// Stores the slice view of a contiguous column data table.
///
/// It doesn't matter how the data is represented.
/// The only thing that matters is that
/// the column table is contiguous in memory,
/// and that the structure holding the memory
/// can be represented as a slice view
/// of `u8` elements. Slices of primitive types
/// automatically can be cast to this kind of
/// slice view, by using `From` trait.
#[derive(Copy, Clone)]
pub struct Sequence<'a> {
    /// Represents a slice
    /// view of any region of memory
    /// converted to a `u8` slice view.
    ///
    /// For doing the conversion from
    /// an arbitrary slice array to a
    /// `u8` slice array, we use the `unsafe from_raw_parts`.
    data_slice: &'a [u8],

    /// Represents the total number of
    /// bytes of each element encoded in the
    /// `data_slice` view.
    ///
    /// During computations, this field is used to infer the total
    /// amount of `rows` in the `data_slice`, using for that the formula:
    ///
    /// ```text
    /// let num_rows = curr_data.data_slice.len() / curr_data.element_size;
    /// ```
    /// So be cautious that the number of bytes captured by the `data_slice`
    /// and the `element_size` value are properly defined. For instance,
    /// you must secure that for:
    ///
    /// - `u8` data slice types: `element_size = std::mem::size_of::<u8>()`
    /// - `u16` data slice types: `element_size = std::mem::size_of::<u16>()`
    /// - `u32` data slice types: `element_size = std::mem::size_of::<u32>()`
    /// - `u64` data slice types: `element_size = std::mem::size_of::<u64>()`
    /// - `u128` data slice types: `element_size = std::mem::size_of::<u128>()`
    element_size: usize,

    /// Represents whether the data slice should be interpreted
    /// as a sequence of signed or unsigned values.
    is_signed: bool,
}

impl<'a> Sequence<'a> {
    /// Returns the number of elements in the Dense Sequence.
    pub fn len(&self) -> usize {
        self.data_slice.len() / self.element_size
    }

    /// Returns `true` if the sequence is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Converts a slice of any type to a Sequence by calling `from_raw_parts` on it.
    /// The `is_signed` parameter is used to determine whether the data is interpreted as a signed value or not.
    /// Several types are also supported via the `From` trait, which is preferred over this method.
    ///
    /// The size of the elements in the slice must be between `1` and `16` bytes (inclusive) if `is_signed` is true,
    /// and between `1` and `32` bytes (inclusive) if `is_signed` is `false`.
    pub fn from_raw_parts<T>(slice: &'a [T], is_signed: bool) -> Self {
        let element_size = core::mem::size_of::<T>();
        Self::from_raw_parts_with_size(slice, element_size, is_signed)
    }

    /// Converts a slice of any type to a Sequence by calling `from_raw_parts` on it.
    ///
    /// The `is_signed` parameter is used to determine whether the data is interpreted as a signed value or not.
    /// The `element_size` parameter specifies the size of each element in bytes.
    pub fn from_raw_parts_with_size<T>(
        slice: &'a [T],
        element_size: usize,
        is_signed: bool,
    ) -> Self {
        assert!(element_size > 0);
        if is_signed {
            assert!(element_size <= 16);
        } else {
            assert!(element_size <= 32);
        }
        let len = std::mem::size_of_val(slice);
        assert_eq!(
            len % element_size,
            0,
            "raw data length should be a multiple of element size"
        );
        let data_slice = unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u8, len) };
        Sequence {
            data_slice,
            element_size,
            is_signed,
        }
    }
}

impl From<&Sequence<'_>> for sxt_sequence_descriptor {
    fn from(other: &Sequence<'_>) -> Self {
        sxt_sequence_descriptor {
            element_nbytes: other.element_size as u8,
            n: other.len() as u64,
            data: other.data_slice.as_ptr(),
            is_signed: other.is_signed as ::std::os::raw::c_int,
        }
    }
}

impl<'a, T> From<&'a Vec<T>> for Sequence<'a>
where
    Sequence<'a>: From<&'a [T]>,
{
    fn from(value: &'a Vec<T>) -> Self {
        value.as_slice().into()
    }
}
impl<'a, T> From<&'a mut [T]> for Sequence<'a>
where
    Sequence<'a>: From<&'a [T]>,
{
    fn from(value: &'a mut [T]) -> Self {
        value[..].into()
    }
}

macro_rules! impl_dense_sequence_for_unsigned {
    ($($t:ty),*) => {
        $(
            impl<'a> From<&'a [$t]> for Sequence<'a> {
                fn from(other: &'a [$t]) -> Self {
                    Sequence::from_raw_parts(other, false)
                }
            }
        )*
    };
}
impl_dense_sequence_for_unsigned!(
    bool,
    u8,
    u16,
    u32,
    u64,
    u128,
    curve25519_dalek::scalar::Scalar
);
macro_rules! impl_dense_sequence_for_signed {
    ($($t:ty),*) => {
        $(
            impl<'a> From<&'a [$t]> for Sequence<'a> {
                fn from(other: &'a [$t]) -> Self {
                    Sequence::from_raw_parts(other, true)
                }
            }
        )*
    };
}
impl_dense_sequence_for_signed!(i8, i16, i32, i64, i128);
macro_rules! impl_dense_sequence_for_unsigned_array {
    ($($t:ty),*) => {
        $(
            impl<'a, const N:usize> From<&'a [[$t;N]]> for Sequence<'a> {
                fn from(other: &'a [[$t;N]]) -> Self {
                    Sequence::from_raw_parts(other, false)
                }
            }
        )*
    };
}
impl_dense_sequence_for_unsigned_array!(bool, u8, u16, u32, u64, u128);

impl<'a> From<&'a [halo2curves::bn256::Fr]> for Sequence<'a> {
    fn from(other: &'a [halo2curves::bn256::Fr]) -> Self {
        let data_slice: &'static [u8] = Box::leak(
            other
                .iter()
                .flat_map(|fr| fr.to_bytes())
                .collect::<Vec<u8>>()
                .into_boxed_slice(),
        );
        let element_size = std::mem::size_of::<halo2curves::bn256::Fr>();
        let is_signed = false;

        Sequence {
            data_slice,
            element_size,
            is_signed,
        }
    }
}

#[cfg(feature = "arkworks")]
impl<'a, const N: usize> From<&'a [ark_ff::BigInt<N>]> for Sequence<'a> {
    fn from(other: &'a [ark_ff::BigInt<N>]) -> Self {
        Sequence::from_raw_parts(other, false)
    }
}

#[cfg(test)]
mod test;
