use super::traits::ASCII;
use super::*;

#[test]
fn add_48_to_existing_number() {
    let value: u8 = 4;
    assert_eq!(value.to_ascii(), Ok(52));
}
#[test]
fn prevent_integer_overflow() {
    assert_eq!(convert_u8_to_ascii(&250), Err(()));
}
