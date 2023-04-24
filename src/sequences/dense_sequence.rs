// -*- mode: rust; -*-
//
// Authors:
// - Joe <joseribeiro1017@gmail.com>
// - Ryan Burn <ryan@spaceandtime.io>

use byte_slice_cast::AsByteSlice;
use curve25519_dalek::scalar::Scalar;

/// This `DenseSequence` stores the slice view
/// of a contiguous column data table.
/// It doesn't matter how the data is represented.
/// The only thing that matters is that
/// the column table is contiguous in memory,
/// and that the structure holding the memory
/// can be represented as a slice view
/// of u8 elements. Vectors of primitive types
/// automatically can be cast to this kind of
/// slice view, by using `byte-slice-cast` trait extension.
pub struct DenseSequence<'a> {
    /// Represents a slice
    /// view of any region of memory
    /// converted to a u8 slice view.
    ///
    /// For doing the conversion from
    /// an arbitrary slice array to a
    /// u8 slice array, we use the `byte-slice-cast`
    /// dependency, which implements the traits `as_byte_slice`.
    /// To do this conversion, you can follow the example below:
    ///
    /// ```
    /// use byte_slice_cast::AsByteSlice;
    /// let data: Vec<u16> = vec![2000, 7500, 5000, 1500];
    /// let data_slice = &data.as_byte_slice();
    /// ```
    ///
    /// Be careful when using signed data, since we cast everything to
    /// unsigned bytes.
    pub data_slice: &'a [u8],

    /// Represents the total number of
    /// bytes of each element encoded in the
    /// `data_slice` view
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
    /// - u8 data slice types: element_size = std::mem::size_of::<u8>()
    /// - u16 data slice types: element_size = std::mem::size_of::<u16>()
    /// - u32 data slice types: element_size = std::mem::size_of::<u32>()
    /// - u64 data slice types: element_size = std::mem::size_of::<u64>()
    /// - u128 data slice types: element_size = std::mem::size_of::<u128>()
    pub element_size: usize,
}

impl DenseSequence<'_> {
    /// Returns the number of elements in the Dense Sequence
    pub fn len(&self) -> usize {
        self.data_slice.len() / self.element_size
    }

    /// Returns true if the sequence is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        (*self).len() == 0
    }

    pub(super) fn to_data_properties(&self) -> (u8, usize, *const u8) {
        assert!(
            self.data_slice.len() % self.element_size == 0,
            "data_slice length is not a multiple of element_size in the dense object"
        );

        let num_rows = (*self).len();

        (self.element_size as u8, num_rows, self.data_slice.as_ptr())
    }
}

macro_rules! into_dense_sequence {
    ($tt:ty) => {
        #[cfg(target_endian = "little")]
        impl<'a> From<&'a [$tt]> for DenseSequence<'a> {
            fn from(value: &'a [$tt]) -> Self {
                DenseSequence {
                    data_slice: value.as_byte_slice(),
                    element_size: std::mem::size_of::<$tt>(),
                }
            }
        }
    };
}

into_dense_sequence!(u8);
into_dense_sequence!(u16);
into_dense_sequence!(u32);
into_dense_sequence!(u64);
into_dense_sequence!(u128);

#[cfg(target_endian = "little")]
impl<'a> From<&'a [bool]> for DenseSequence<'a> {
    fn from(value: &'a [bool]) -> Self {
        let len = std::mem::size_of_val(value);
        let slice = unsafe { std::slice::from_raw_parts(value.as_ptr() as *const u8, len) };
        DenseSequence {
            data_slice: slice,
            element_size: std::mem::size_of::<bool>(),
        }
    }
}

#[cfg(target_endian = "little")]
impl<'a> From<&'a [Scalar]> for DenseSequence<'a> {
    fn from(value: &'a [Scalar]) -> Self {
        let len = std::mem::size_of_val(value);
        let slice = unsafe { std::slice::from_raw_parts(value.as_ptr() as *const u8, len) };
        DenseSequence {
            data_slice: slice,
            element_size: std::mem::size_of::<Scalar>(),
        }
    }
}

#[cfg(test)]
#[cfg(target_endian = "little")]
mod test {
    use super::DenseSequence;
    use curve25519_dalek::scalar::Scalar;

    #[test]
    fn we_can_convert_an_empty_slice_of_uints_to_a_dense_sequence() {
        let s = Vec::<u8>::new();
        let d = DenseSequence::from(&s[..]);
        assert_eq!(d.element_size, std::mem::size_of::<u8>());
        assert!(d.is_empty());
        let s = Vec::<u32>::new();
        let d = DenseSequence::from(&s[..]);
        assert_eq!(d.element_size, std::mem::size_of::<u32>());
        assert!(d.is_empty());
        let s = Vec::<u128>::new();
        let d = DenseSequence::from(&s[..]);
        assert_eq!(d.element_size, std::mem::size_of::<u128>());
        assert!(d.is_empty());
    }
    #[test]
    fn we_can_convert_an_empty_slice_of_scalars_to_a_dense_sequence() {
        let s = Vec::<Scalar>::new();
        let d = DenseSequence::from(&s[..]);
        assert_eq!(d.element_size, std::mem::size_of::<Scalar>());
        assert!(d.is_empty());
    }
    #[test]
    fn we_can_convert_an_empty_slice_of_bools_to_a_dense_sequence() {
        let s = Vec::<bool>::new();
        let d = DenseSequence::from(&s[..]);
        assert_eq!(d.element_size, std::mem::size_of::<bool>());
        assert!(d.is_empty());
    }

    #[test]
    fn we_can_convert_a_slice_of_uints_to_a_dense_sequence_with_correct_data() {
        let s = vec![123u8, 45u8, 78u8];
        let d = DenseSequence::from(&s[..]);
        assert_eq!(d.element_size, std::mem::size_of::<u8>());
        assert_eq!(d.len(), 3);

        assert_eq!(d.data_slice[0..d.element_size], 123u8.to_le_bytes());
        assert_eq!(
            d.data_slice[d.element_size..2 * d.element_size],
            45u8.to_le_bytes()
        );
        assert_eq!(
            d.data_slice[2 * d.element_size..3 * d.element_size],
            78u8.to_le_bytes()
        );

        let s = vec![123u32, 456u32, 789u32];
        let d = DenseSequence::from(&s[..]);
        assert_eq!(d.element_size, std::mem::size_of::<u32>());
        assert_eq!(d.len(), 3);

        assert_eq!(d.data_slice[0..d.element_size], 123u32.to_le_bytes());
        assert_eq!(
            d.data_slice[d.element_size..2 * d.element_size],
            456u32.to_le_bytes()
        );
        assert_eq!(
            d.data_slice[2 * d.element_size..3 * d.element_size],
            789u32.to_le_bytes()
        );
    }

    #[test]
    fn we_can_convert_a_slice_of_bools_to_a_dense_sequence_with_correct_data() {
        let s = vec![true, false, true];
        let d = DenseSequence::from(&s[..]);
        assert_eq!(d.element_size, std::mem::size_of::<bool>());
        assert_eq!(d.len(), 3);
        assert_eq!(d.data_slice[0], 1);
        assert_eq!(d.data_slice[1], 0);
        assert_eq!(d.data_slice[2], 1);
    }

    #[test]
    fn we_can_convert_a_slice_of_scalars_to_a_dense_sequence_with_correct_data() {
        let s = vec![
            Scalar::from(123u32),
            -Scalar::from(456u32),
            Scalar::from(789u32),
        ];
        let d = DenseSequence::from(&s[..]);
        assert_eq!(d.element_size, std::mem::size_of::<Scalar>());
        assert_eq!(d.len(), 3);

        assert_eq!(
            d.data_slice[0..d.element_size],
            Scalar::from(123u32).as_bytes()[..]
        );
        assert_eq!(
            d.data_slice[d.element_size..2 * d.element_size],
            (-Scalar::from(456u32)).as_bytes()[..]
        );
        assert_eq!(
            d.data_slice[2 * d.element_size..3 * d.element_size],
            Scalar::from(789u32).as_bytes()[..]
        );
    }
}
