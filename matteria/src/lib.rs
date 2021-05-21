#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::vec::Vec;
/// Utility function that converts an u8 integer into a ASCII byte by adding 48 to the provided number.
///
/// The function uses checked_add which returns an Some(number) if everything is ok,
/// and None if a integer overflow happened.
///
/// ## ASCII Table
///
/// |Dec   | Hex   | Binary          | HTML   | Char   | Description    |
/// |------|-------|-----------------|--------|--------|----------------|
/// | 48   | 30    | 00110000        | &#48;  | 0      | Zero           |
/// | 49   | 31    | 00110001        | &#49;  | 1      | One            |
/// | 50   | 32    | 00110010        | &#50;  | 2      | Two            |
/// | 51   | 33    | 00110011        | &#51;  | 3      | Three          |
/// | 52   | 34    | 00110100        | &#52;  | 4      | Four           |
/// | 53   | 35    | 00110101        | &#53;  | 5      | Five           |
/// | 54   | 36    | 00110110        | &#54;  | 6      | Six            |
/// | 55   | 37    | 00110111        | &#55;  | 7      | Seven          |
/// | 56   | 38    | 00111000        | &#56;  | 8      | Eight          |
/// | 57   | 39    | 00111001        | &#57;  | 9      | Nine           |
///
pub fn convert_u8_to_ascii(input: &u8) -> Result<u8, ()> {
    match input.checked_add(48u8) {
        Some(input) => Ok(input),
        None => Err(()),
    }
}

/// Utility function that matches an `Option<Vec<T>>` param and checks for the existence
/// of a value inside the vector.
pub fn validate_option_vec(input: &Option<Vec<u32>>, guard: &u32) -> bool {
    match input {
        Some(vec) => vec.iter().any(|s| match s {
            x if x == guard => true,
            _ => false,
        }),
        None => false,
    }
}

pub fn generate_sequence_vector(sequence: u16) -> Result<Vec<u8>, ()> {
    let digits: Vec<_> = Digits::new(sequence).collect();
    // TODO: Refactorizar este codigo:
    // El codigo (lineas 134..144) recibe el argumento digits que es un array de integers
    // por ejemplo [4,2] lo que debemos obtener es [0,4,2]
    let mut bytes_vec = Vec::new();
    while digits.len() + bytes_vec.len() < 3 {
        bytes_vec.push(0);
    }
    for n in &digits {
        bytes_vec.push(*n);
    }

    for i in 0..3 {
        bytes_vec[i] = convert_u8_to_ascii(&bytes_vec[i])?;
    }
    Ok(bytes_vec)
}

struct Digits {
    n: u8,
    divisor: u8,
}

impl Digits {
    fn new(n: u16) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits {
            n: n as u8,
            divisor: divisor as u8,
        }
    }
}

impl Iterator for Digits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
}

pub fn convert_to_vec_u8(input: &str) -> Vec<u8> {
    input.as_bytes().to_vec()
}

#[test]
fn add_48_to_existing_number() {
    assert_eq!(convert_u8_to_ascii(&4), Ok(52));
}
#[test]
fn prevent_integer_overflow() {
    assert_eq!(convert_u8_to_ascii(&250), Err(()));
}
