use jdn_utils::*;

#[test]
fn test_normalized_string() {
    assert!(NormalizedString::default_verify(String::from("asdf"), "User ID").is_ok());

    assert!(NormalizedString::default_verify(String::from("asdf;zxcv"), "User ID").is_err());

    assert!(NormalizedString::default_verify(String::from("01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789"), "User ID").is_err());
}
