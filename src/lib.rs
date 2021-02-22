#![cfg_attr(not(feature = "std"), no_std)]

/// Utility function that converts a u8 integer to a ASCII byte
/// The addition does not check for a possible overflow, since the u8 numbers provided are less than 9
pub fn convert_u8_to_ascii(input: &u8) -> u8 {
    assert!(input < &9u8, "Number has to be less than 10");
    48u8 + input
}

#[test]
fn add_48_to_existing_number() {
    assert_eq!(convert_u8_to_ascii(&4), 52);
}
