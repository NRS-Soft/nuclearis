#![cfg_attr(not(feature = "std"), no_std)]

pub fn convert_u8_to_ascii(input: &u8) -> Option<u8> {
    match 48u8.checked_add(*input) {
        Some(val) => Some(val),
        None => None,
    }
}

#[test]
fn add_48_to_existing_number() {
    assert_eq!(convert_u8_to_ascii(&20), Some(68));
}

#[test]
fn check_for_overflow() {
    assert_eq!(convert_u8_to_ascii(&240), None);
}
