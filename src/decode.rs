use crate::error::UVarintError;

/// Decodes a variable-length unsigned 32-bit integer from a byte slice.
///
/// # Varint Encoding Format
///
/// Varints use the lower 7 bits of each byte for data, and the most significant
/// bit (MSB) as a continuation flag:
/// - MSB = 1: More bytes follow
/// - MSB = 0: This is the last byte
///
/// Bytes are stored in little-endian order (least significant bits first).
///
/// # Examples
///
/// ```
/// use uvarint::decode::decode_u32;
///
/// // Two bytes: 300
/// // 300 = 0b1_0010_1100 (needs 9 bits)
/// // Split into 7-bit chunks: [010_1100] [0000_010]
/// // Reverse order & add continuation bits: [1010_1100] [0000_0010]
/// assert_eq!(decode_u32(&[0xAC, 0x02]).unwrap(), (2, 300));
/// ```
///
/// # Step-by-Step Decoding Process
///
/// For the value 300 encoded as `[0xAC, 0x02]`:
///
/// **Byte 1: 0xAC (0b1010_1100)**
/// ```text
/// data_bits = 0xAC & 0x7F        // Mask out MSB
///           = 0b1010_1100 & 0b0111_1111
///           = 0b0010_1100 (44 in decimal)
/// value = 44 << 0 = 44
/// continuation = 0xAC & 0x80     // Check MSB
///              = 0b1000_0000 (non-zero, continue!)
/// ```
///
/// **Byte 2: 0x02 (0b0000_0010)**
/// ```text
/// data_bits = 0x02 & 0x7F
///           = 0b0000_0010 (2 in decimal)
/// value = 44 | (2 << 7)          // Shift by 7 bits for each byte
///       = 44 | 256
///       = 300
/// continuation = 0x02 & 0x80
///              = 0 (MSB is 0, done!)
/// ```
pub fn decode_u32(data: &[u8]) -> Result<(usize, u32), UVarintError> {
    let mut value: u32 = 0;

    for (i, &byte) in data.iter().take(5).enumerate() {
        let data_bits = (byte & 0x7F) as u32;

        value = value
            .checked_add(
                data_bits
                    .checked_shl(i as u32 * 7)
                    .ok_or(UVarintError::Overflow)?,
            )
            .ok_or(UVarintError::Overflow)?;

        if (byte & 0x80) == 0 {
            return Ok((i + 1, value));
        }
    }

    Err(UVarintError::Incomplete)
}

/// Decodes a variable-length unsigned 64-bit integer from a byte slice.
///
/// # Varint Encoding Format
///
/// Varints use the lower 7 bits of each byte for data, and the most significant
/// bit (MSB) as a continuation flag:
/// - MSB = 1: More bytes follow
/// - MSB = 0: This is the last byte
///
/// Bytes are stored in little-endian order (least significant bits first).
///
/// # Examples
///
/// ```
/// use uvarint::decode::decode_u64;
///
/// // Two bytes: 300
/// // 300 = 0b1_0010_1100 (needs 9 bits)
/// // Split into 7-bit chunks: [010_1100] [0000_010]
/// // Reverse order & add continuation bits: [1010_1100] [0000_0010]
/// assert_eq!(decode_u64(&[0xAC, 0x02]).unwrap(), (2, 300));
/// ```
///
/// # Step-by-Step Decoding Process
///
/// For the value 300 encoded as `[0xAC, 0x02]`:
///
/// **Byte 1: 0xAC (0b1010_1100)**
/// ```text
/// data_bits = 0xAC & 0x7F        // Mask out MSB
///           = 0b1010_1100 & 0b0111_1111
///           = 0b0010_1100 (44 in decimal)
/// value = 44 << 0 = 44
/// continuation = 0xAC & 0x80     // Check MSB
///              = 0b1000_0000 (non-zero, continue!)
/// ```
///
/// **Byte 2: 0x02 (0b0000_0010)**
/// ```text
/// data_bits = 0x02 & 0x7F
///           = 0b0000_0010 (2 in decimal)
/// value = 44 | (2 << 7)          // Shift by 7 bits for each byte
///       = 44 | 256
///       = 300
/// continuation = 0x02 & 0x80
///              = 0 (MSB is 0, done!)
/// ```
pub fn decode_u64(data: &[u8]) -> Result<(usize, u64), UVarintError> {
    let mut value: u64 = 0;

    for (i, &byte) in data.iter().take(10).enumerate() {
        let data_bits = (byte & 0x7F) as u64;

        value = value
            .checked_add(
                data_bits
                    .checked_shl(i as u32 * 7)
                    .ok_or(UVarintError::Overflow)?,
            )
            .ok_or(UVarintError::Overflow)?;

        if (byte & 0x80) == 0 {
            return Ok((i + 1, value));
        }
    }

    Err(UVarintError::Incomplete)
}

/// Decodes a variable-length unsigned 128-bit integer from a byte slice.
///
/// # Varint Encoding Format
///
/// Varints use the lower 7 bits of each byte for data, and the most significant
/// bit (MSB) as a continuation flag:
/// - MSB = 1: More bytes follow
/// - MSB = 0: This is the last byte
///
/// Bytes are stored in little-endian order (least significant bits first).
///
/// # Examples
///
/// ```
/// use uvarint::decode::decode_u128;
///
/// // Two bytes: 300
/// // 300 = 0b1_0010_1100 (needs 9 bits)
/// // Split into 7-bit chunks: [010_1100] [0000_010]
/// // Reverse order & add continuation bits: [1010_1100] [0000_0010]
/// assert_eq!(decode_u128(&[0xAC, 0x02]).unwrap(), (2, 300));
/// ```
pub fn decode_u128(data: &[u8]) -> Result<(usize, u128), UVarintError> {
    let mut value: u128 = 0;

    for (i, &byte) in data.iter().take(16).enumerate() {
        let data_bits = (byte & 0x7F) as u128;

        value = value
            .checked_add(
                data_bits
                    .checked_shl(i as u32 * 7)
                    .ok_or(UVarintError::Overflow)?,
            )
            .ok_or(UVarintError::Overflow)?;

        if (byte & 0x80) == 0 {
            return Ok((i + 1, value));
        }
    }

    Err(UVarintError::Incomplete)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_byte_u32_values() {
        assert_eq!(decode_u32(&[0x00]).unwrap(), (1, 0));
        assert_eq!(decode_u32(&[0x01]).unwrap(), (1, 1));
        assert_eq!(decode_u32(&[0x05]).unwrap(), (1, 5));
        assert_eq!(decode_u32(&[0x7F]).unwrap(), (1, 127));
    }

    #[test]
    fn test_two_byte_u32_values() {
        assert_eq!(decode_u32(&[0x80, 0x01]).unwrap(), (2, 128));
        assert_eq!(decode_u32(&[0xAC, 0x02]).unwrap(), (2, 300));
        assert_eq!(decode_u32(&[0xFF, 0x7F]).unwrap(), (2, 16_383));
    }

    #[test]
    fn test_single_byte_u64_values() {
        assert_eq!(decode_u64(&[0x00]).unwrap(), (1, 0));
        assert_eq!(decode_u64(&[0x01]).unwrap(), (1, 1));
        assert_eq!(decode_u64(&[0x05]).unwrap(), (1, 5));
        assert_eq!(decode_u64(&[0x7F]).unwrap(), (1, 127));
    }

    #[test]
    fn test_two_byte_u64_values() {
        assert_eq!(decode_u64(&[0x80, 0x01]).unwrap(), (2, 128));
        assert_eq!(decode_u64(&[0xAC, 0x02]).unwrap(), (2, 300));
        assert_eq!(decode_u64(&[0xFF, 0x7F]).unwrap(), (2, 16_383));
    }

    #[test]
    fn test_single_byte_u128_values() {
        assert_eq!(decode_u128(&[0x00]).unwrap(), (1, 0));
        assert_eq!(decode_u128(&[0x01]).unwrap(), (1, 1));
        assert_eq!(decode_u128(&[0x05]).unwrap(), (1, 5));
        assert_eq!(decode_u128(&[0x7F]).unwrap(), (1, 127));
    }

    #[test]
    fn test_two_byte_u128_values() {
        // 128 = 0b1000_0000
        // Split: [000_0000] [000_0001]
        // Encode: [1000_0000] [0000_0001]
        assert_eq!(decode_u128(&[0x80, 0x01]).unwrap(), (2, 128));

        // 300 = 0b1_0010_1100
        // Split: [010_1100] [000_0010]
        // Encode: [1010_1100] [0000_0010]
        assert_eq!(decode_u128(&[0xAC, 0x02]).unwrap(), (2, 300));

        // 16,383 = 0b11_1111_1111_1111 (max 2-byte value)
        // Split: [111_1111] [111_1111]
        // Encode: [1111_1111] [0111_1111]
        assert_eq!(decode_u128(&[0xFF, 0x7F]).unwrap(), (2, 16_383));
    }
}
