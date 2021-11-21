use std::string::FromUtf8Error;

use thiserror::Error;

/// An enumeration of errors that can occur during custom serialization.
#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("Expected {0} bytes, found {1}")]
    UnexpectedByteCount(usize, usize),
    #[error("Could not deserialize to string, invalid UTF8")]
    InvalidString {
        #[from]
        source: FromUtf8Error,
    },
    #[error("{0}")]
    InvalidValue(String),
}

/// Appends the string representation of the given value to the buffer.
pub fn serialize_string<T: Into<String>>(value: T, buffer: &mut Vec<u8>) {
    let mut value = value.into().into_bytes();
    for b in value.len().to_le_bytes() {
        buffer.push(b);
    }
    buffer.append(&mut value);
}

/// Removes the next string value from the buffer.
pub fn deserialize_string<T: TryFrom<String>>(buffer: &mut Vec<u8>) -> Result<T, SerializationError>
where
    <T as TryFrom<String>>::Error: ToString,
{
    let value_size = remove_usize(buffer)?;
    let tmp = buffer.split_off(value_size);
    let result = String::from_utf8(buffer.to_owned()).map_err(|ex| ex.into());
    *buffer = tmp;
    result.and_then(|value| {
        T::try_from(value).map_err(|ex| SerializationError::InvalidValue(ex.to_string()))
    })
}

/// Appends the given collection to the buffer.
pub fn serialize_vec<T: Into<String>>(value: Vec<T>, buffer: &mut Vec<u8>) {
    for b in value.len().to_le_bytes() {
        buffer.push(b);
    }
    for item in value {
        serialize_string(item.into(), buffer);
    }
}

/// Removes the next collection of strings from the buffer.
/// If an error occurs for an element after the first, the buffer is left in an indeterminate state.
pub fn deserialize_vec<T: TryFrom<String>>(
    buffer: &mut Vec<u8>,
) -> Result<Vec<T>, SerializationError>
where
    <T as TryFrom<String>>::Error: ToString,
{
    let num_items = remove_usize(buffer)?;
    let mut result = Vec::with_capacity(num_items);
    for _ in 0..num_items {
        result.push(deserialize_string(buffer)?);
    }
    Ok(result)
}

/// Prepends the length of the buffer to the buffer.
pub fn finalize_serialization(buffer: &mut Vec<u8>) {
    let buffer_len = buffer.len();
    for (index, b) in buffer_len.to_le_bytes().iter().enumerate() {
        buffer.insert(index, *b);
    }
}

/// Removes a usize for the following value from the buffer.
/// If the buffer does not contain enough elements to create a usize, the buffer is unchanged
/// and an error is returned.
pub fn remove_usize(buffer: &mut Vec<u8>) -> Result<usize, SerializationError> {
    let usize_len = std::mem::size_of::<usize>();
    if buffer.len() < usize_len {
        return Err(SerializationError::UnexpectedByteCount(
            usize_len,
            buffer.len(),
        ));
    }
    let remaining_bytes = buffer.split_off(usize_len);
    let result = usize::from_le_bytes(buffer.as_slice().try_into().unwrap());
    *buffer = remaining_bytes;
    Ok(result)
}

/// Removes a u32 for the following value from the buffer.
/// If the buffer does not contain enough elements to create a u32, the buffer is unchanged
/// and an error is returned.
pub fn remove_u32(buffer: &mut Vec<u8>) -> Result<u32, SerializationError> {
    let u32_len = std::mem::size_of::<u32>();
    if buffer.len() < u32_len {
        return Err(SerializationError::UnexpectedByteCount(
            u32_len,
            buffer.len(),
        ));
    }
    let remaining_bytes = buffer.split_off(u32_len);
    let result = u32::from_le_bytes(buffer.as_slice().try_into().unwrap());
    *buffer = remaining_bytes;
    Ok(result)
}
