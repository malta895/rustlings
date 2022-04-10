// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value.cmp(&0) {
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Less => Err(CreationError::Negative),
            _ => Ok(PositiveNonzeroInteger(value as u64)),
        }
    }
}

#[test]
fn test_creation_ok() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
}

#[test]
fn test_creation_of_negative_fails() {
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
}

#[test]
fn test_creation_of_zero_fails() {
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
