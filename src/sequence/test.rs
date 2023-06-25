use super::Sequence;
use curve25519_dalek::scalar::Scalar;

#[test]
fn we_can_convert_an_empty_slice_of_uints_to_a_sequence() {
    let s = Vec::<u8>::new();
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<u8>());
    assert!(d.is_empty());
    let s = Vec::<u32>::new();
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<u32>());
    assert!(d.is_empty());
    let s = Vec::<u128>::new();
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<u128>());
    assert!(d.is_empty());
}
#[test]
fn we_can_convert_an_empty_slice_of_scalars_to_a_sequence() {
    let s = Vec::<Scalar>::new();
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<Scalar>());
    assert!(d.is_empty());
}
#[test]
fn we_can_convert_an_empty_slice_of_bools_to_a_sequence() {
    let s = Vec::<bool>::new();
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<bool>());
    assert!(d.is_empty());
}

#[test]
fn we_can_convert_an_empty_slice_of_u64_arrays_to_a_sequence() {
    let s = Vec::<[u64; 4]>::new();
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<[u64; 4]>());
    assert!(d.is_empty());
}
#[test]
fn we_can_convert_an_empty_slice_of_u8_arrays_to_a_sequence() {
    let s = Vec::<[u8; 3]>::new();
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<[u8; 3]>());
    assert!(d.is_empty());
}

#[test]
fn we_can_convert_a_slice_of_uints_to_a_sequence_with_correct_data() {
    let s = vec![123u8, 45u8, 78u8];
    let d = Sequence::from(&s[..]);
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
    assert!(!d.is_signed);

    let s = vec![123u32, 456u32, 789u32];
    let d = Sequence::from(&s[..]);
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
    assert!(!d.is_signed);
}

#[test]
fn we_can_convert_a_slice_of_u64_arrays_to_a_sequence_with_correct_data() {
    let s = vec![
        [123u64, 456u64, 789u64, 101112u64],
        [0, 0, 0, 0],
        [321u64, 654u64, 987u64, 121110u64],
    ];
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<[u64; 4]>());
    assert_eq!(d.len(), 3);

    assert_eq!(d.data_slice[0..8], 123u64.to_le_bytes());
    assert_eq!(d.data_slice[8..8 * 2], 456u64.to_le_bytes());
    assert_eq!(d.data_slice[8 * 2..8 * 3], 789u64.to_le_bytes());
    assert_eq!(d.data_slice[8 * 3..8 * 4], 101112u64.to_le_bytes());
    assert_eq!(d.data_slice[8 * 4..8 * 8], [0; 8 * 4]);
    assert_eq!(d.data_slice[8 * 8..8 * 9], 321u64.to_le_bytes());
    assert_eq!(d.data_slice[8 * 9..8 * 10], 654u64.to_le_bytes());
    assert_eq!(d.data_slice[8 * 10..8 * 11], 987u64.to_le_bytes());
    assert_eq!(d.data_slice[8 * 11..8 * 12], 121110u64.to_le_bytes());
}
#[test]
fn we_can_convert_a_slice_of_u8_arrays_to_a_sequence_with_correct_data() {
    let s = vec![
        [123u8, 45u8, 78u8],
        [0, 0, 0],
        [1, 1, 1],
        [10u8, 11u8, 12u8],
    ];
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<[u8; 3]>());
    assert_eq!(d.len(), 4);
    assert_eq!(
        d.data_slice,
        [123u8, 45u8, 78u8, 0, 0, 0, 1, 1, 1, 10u8, 11u8, 12u8]
    );
}

#[test]
fn we_can_convert_a_slice_of_bools_to_a_sequence_with_correct_data() {
    let s = vec![true, false, true];
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<bool>());
    assert_eq!(d.len(), 3);
    assert_eq!(d.data_slice[0], 1);
    assert_eq!(d.data_slice[1], 0);
    assert_eq!(d.data_slice[2], 1);
}

#[test]
fn we_can_convert_a_slice_of_signed_ints_to_a_sequence_with_correct_data() {
    let s = vec![123i8, -45i8, 78i8];
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<i8>());
    assert_eq!(d.len(), 3);

    assert_eq!(d.data_slice[0..d.element_size], 123i8.to_le_bytes());
    assert_eq!(
        d.data_slice[d.element_size..2 * d.element_size],
        (-45i8).to_le_bytes()
    );
    assert_eq!(
        d.data_slice[2 * d.element_size..3 * d.element_size],
        78i8.to_le_bytes()
    );
    assert!(d.is_signed);

    let s = vec![123i32, -456i32, 789i32];
    let d = Sequence::from(&s[..]);
    assert_eq!(d.element_size, std::mem::size_of::<i32>());
    assert_eq!(d.len(), 3);

    assert_eq!(d.data_slice[0..d.element_size], 123i32.to_le_bytes());
    assert_eq!(
        d.data_slice[d.element_size..2 * d.element_size],
        (-456i32).to_le_bytes()
    );
    assert_eq!(
        d.data_slice[2 * d.element_size..3 * d.element_size],
        789i32.to_le_bytes()
    );
    assert!(d.is_signed);
}

#[test]
fn we_can_convert_a_slice_of_scalars_to_a_sequence_with_correct_data() {
    let s = vec![
        Scalar::from(123u32),
        -Scalar::from(456u32),
        Scalar::from(789u32),
    ];
    let d = Sequence::from(&s[..]);
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

#[test]
#[cfg(feature = "arkworks")]
fn we_can_convert_a_slice_of_arkworks_bigint_to_the_same_values_as_scalars() {
    let a = vec![
        ark_ff::BigInt::<4>::from(123u32),
        ark_ff::BigInt::<4>::from(456u32),
        ark_ff::BigInt::<4>::from(789u32),
    ];
    let b = vec![
        Scalar::from(123u32),
        Scalar::from(456u32),
        Scalar::from(789u32),
    ];
    let a_seq = Sequence::from(&a[..]);
    let b_seq = Sequence::from(&b[..]);
    assert_eq!(a_seq.element_size, b_seq.element_size);
    assert_eq!(a_seq.len(), b_seq.len());
    assert_eq!(a_seq.data_slice, b_seq.data_slice);
}
