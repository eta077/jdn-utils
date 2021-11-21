use jdn_utils::*;

#[test]
fn test_normalized_string() {
    assert!(NormalizedString::default_verify(String::from("asdf"), "User ID").is_ok());

    assert!(NormalizedString::default_verify(String::from("asdf;zxcv"), "User ID").is_err());

    assert!(NormalizedString::default_verify(String::from("01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789"), "User ID").is_err());
}

#[test]
fn test_u32_serialization() {
    let mut buffer = Vec::new();

    assert!(remove_u32(&mut buffer).is_err());

    buffer.append(&mut Vec::from(32u32.to_le_bytes()));

    assert_eq!(remove_u32(&mut buffer), Ok(32));
}
