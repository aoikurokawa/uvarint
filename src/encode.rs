use crate::error::UVarintError;

/// Encodes a u16 value into unsigned varint format.
///
/// # Varint Encoding Algorithm
///
/// 1. Take the lowest 7 bits of the value
/// 2. If there are more bits remaining, set the MSB to 1 (continuation bit)
/// 3. Write the byte
/// 4. Shift the value right by 7 bits
/// 5. Repeat until value is 0
///
/// # Examples
///
/// ```
/// use uvarint::encode::encode_u16;
///
/// // 300 = 0b1_0010_1100
/// // Byte 1: bits 0-6 = 0b010_1100 (44), MSB=1 → 0b1010_1100
/// // Byte 2: bits 7-13 = 0b10 (2), MSB=0 → 0b0000_0010
/// assert_eq!(encode_u16(300), vec![0xAC, 0x02]);
/// ```
///
/// # Step-by-Step Example: Encoding 300
///
/// ```text
/// Value: 300 = 0b1_0010_1100
///
/// Step 1:
///   value = 300 = 0b1_0010_1100
///   lower_7_bits = 300 & 0x7F = 0b010_1100 = 44
///   remaining = 300 >> 7 = 0b10 = 2 (not zero, so set continuation bit)
///   byte = 44 | 0x80 = 0b0010_1100 | 0b1000_0000 = 0b1010_1100 = 0xAC
///   Output: [0xAC]
///
/// Step 2:
///   value = 2 = 0b10
///   lower_7_bits = 2 & 0x7F = 0b10 = 2
///   remaining = 2 >> 7 = 0 (zero, so no continuation bit)
///   byte = 2 = 0b0000_0010 = 0x02
///   Output: [0xAC, 0x02]
///
/// Done!
/// ```
pub fn encode_u16(mut value: u16) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut result = Vec::new();

    while value > 0 {
        let mut byte = (value & 0x7F) as u8;

        value >>= 7;

        if value > 0 {
            byte |= 0x80;
        }

        result.push(byte);
    }

    result
}

/// Encodes a u32 value into unsigned varint format.
///
/// # Varint Encoding Algorithm
///
/// 1. Take the lowest 7 bits of the value
/// 2. If there are more bits remaining, set the MSB to 1 (continuation bit)
/// 3. Write the byte
/// 4. Shift the value right by 7 bits
/// 5. Repeat until value is 0
///
/// # Examples
///
/// ```
/// use uvarint::encode::encode_u32;
///
/// // 300 = 0b1_0010_1100
/// // Byte 1: bits 0-6 = 0b010_1100 (44), MSB=1 → 0b1010_1100
/// // Byte 2: bits 7-13 = 0b10 (2), MSB=0 → 0b0000_0010
/// assert_eq!(encode_u32(300), vec![0xAC, 0x02]);
/// ```
///
/// # Step-by-Step Example: Encoding 300
///
/// ```text
/// Value: 300 = 0b1_0010_1100
///
/// Step 1:
///   value = 300 = 0b1_0010_1100
///   lower_7_bits = 300 & 0x7F = 0b010_1100 = 44
///   remaining = 300 >> 7 = 0b10 = 2 (not zero, so set continuation bit)
///   byte = 44 | 0x80 = 0b0010_1100 | 0b1000_0000 = 0b1010_1100 = 0xAC
///   Output: [0xAC]
///
/// Step 2:
///   value = 2 = 0b10
///   lower_7_bits = 2 & 0x7F = 0b10 = 2
///   remaining = 2 >> 7 = 0 (zero, so no continuation bit)
///   byte = 2 = 0b0000_0010 = 0x02
///   Output: [0xAC, 0x02]
///
/// Done!
/// ```
pub fn encode_u32(mut value: u32) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut result = Vec::new();

    while value > 0 {
        let mut byte = (value & 0x7F) as u8;

        value >>= 7;

        if value > 0 {
            byte |= 0x80;
        }

        result.push(byte);
    }

    result
}

/// Encodes a u16 into a provided buffer, returning the number of bytes written.
///
/// # Examples
///
/// ```
/// use uvarint::encode::encode_u16_into;
///
/// let mut buf = [0u8; 10];
/// let n = encode_u16_into(300, &mut buf).unwrap();
/// assert_eq!(n, 2);
/// assert_eq!(&buf[..n], &[0xAC, 0x02]);
/// ```
pub fn encode_u16_into(mut value: u16, buf: &mut [u8]) -> Result<usize, UVarintError> {
    if buf.is_empty() {
        return Err(UVarintError::BufferTooSmall);
    }

    if value == 0 {
        buf[0] = 0x00;
        return Ok(1);
    }

    let mut i = 0;

    while value > 0 {
        if i >= buf.len() {
            return Err(UVarintError::BufferTooSmall);
        }

        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if value > 0 {
            byte |= 0x80;
        }

        buf[i] = byte;
        i += 1;
    }

    Ok(i)
}

/// Encodes a u32 into a provided buffer, returning the number of bytes written.
///
/// # Examples
///
/// ```
/// use uvarint::encode::encode_u32_into;
///
/// let mut buf = [0u8; 10];
/// let n = encode_u32_into(300, &mut buf).unwrap();
/// assert_eq!(n, 2);
/// assert_eq!(&buf[..n], &[0xAC, 0x02]);
/// ```
pub fn encode_u32_into(mut value: u32, buf: &mut [u8]) -> Result<usize, UVarintError> {
    if buf.is_empty() {
        return Err(UVarintError::BufferTooSmall);
    }

    // Special case: 0
    if value == 0 {
        buf[0] = 0x00;
        return Ok(1);
    }

    let mut i = 0;

    while value > 0 {
        if i >= buf.len() {
            return Err(UVarintError::BufferTooSmall);
        }

        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if value > 0 {
            byte |= 0x80;
        }

        buf[i] = byte;
        i += 1;
    }

    Ok(i)
}

/// Encodes a u64 value into unsigned varint format.
///
/// # Varint Encoding Algorithm
///
/// 1. Take the lowest 7 bits of the value
/// 2. If there are more bits remaining, set the MSB to 1 (continuation bit)
/// 3. Write the byte
/// 4. Shift the value right by 7 bits
/// 5. Repeat until value is 0
///
/// # Examples
///
/// ```
/// use uvarint::encode::encode_u64;
///
/// // 300 = 0b1_0010_1100
/// // Byte 1: bits 0-6 = 0b010_1100 (44), MSB=1 → 0b1010_1100
/// // Byte 2: bits 7-13 = 0b10 (2), MSB=0 → 0b0000_0010
/// assert_eq!(encode_u64(300), vec![0xAC, 0x02]);
/// ```
///
/// # Step-by-Step Example: Encoding 300
///
/// ```text
/// Value: 300 = 0b1_0010_1100
///
/// Step 1:
///   value = 300 = 0b1_0010_1100
///   lower_7_bits = 300 & 0x7F = 0b010_1100 = 44
///   remaining = 300 >> 7 = 0b10 = 2 (not zero, so set continuation bit)
///   byte = 44 | 0x80 = 0b0010_1100 | 0b1000_0000 = 0b1010_1100 = 0xAC
///   Output: [0xAC]
///
/// Step 2:
///   value = 2 = 0b10
///   lower_7_bits = 2 & 0x7F = 0b10 = 2
///   remaining = 2 >> 7 = 0 (zero, so no continuation bit)
///   byte = 2 = 0b0000_0010 = 0x02
///   Output: [0xAC, 0x02]
///
/// Done!
/// ```
pub fn encode_u64(mut value: u64) -> Vec<u8> {
    // Special case: 0 encodes as a single byte
    if value == 0 {
        return vec![0x00];
    }

    let mut result = Vec::new();

    while value > 0 {
        // Extract the lower 7 bits
        let mut byte = (value & 0x7F) as u8;

        // Shift value right by 7 bits
        value >>= 7;

        // If there are more bits, set the continuation bit (MSB)
        if value > 0 {
            byte |= 0x80; // Set MSB to 1
        }
        // Otherwise MSB stays 0 (this is the last byte)

        result.push(byte);
    }

    result
}

/// Encodes a u64 into a provided buffer, returning the number of bytes written.
///
/// # Examples
///
/// ```
/// use uvarint::encode::encode_u64_into;
///
/// let mut buf = [0u8; 10];
/// let n = encode_u64_into(300, &mut buf).unwrap();
/// assert_eq!(n, 2);
/// assert_eq!(&buf[..n], &[0xAC, 0x02]);
/// ```
///
/// # Errors
///
/// Returns `UVarintError::BufferTooSmall` if the buffer is too small.
pub fn encode_u64_into(mut value: u64, buf: &mut [u8]) -> Result<usize, UVarintError> {
    if buf.is_empty() {
        return Err(UVarintError::BufferTooSmall);
    }

    // Special case: 0
    if value == 0 {
        buf[0] = 0x00;
        return Ok(1);
    }

    let mut i = 0;

    while value > 0 {
        if i >= buf.len() {
            return Err(UVarintError::BufferTooSmall);
        }

        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if value > 0 {
            byte |= 0x80;
        }

        buf[i] = byte;
        i += 1;
    }

    Ok(i)
}

#[cfg(test)]
mod encode_tests {
    use super::*;

    #[test]
    fn test_encode_u16_single_byte() {
        assert_eq!(encode_u16(0), vec![0x00]);
        assert_eq!(encode_u16(1), vec![0x01]);
        assert_eq!(encode_u16(5), vec![0x05]);
        assert_eq!(encode_u16(127), vec![0x7F]); // Max single byte
    }

    #[test]
    fn test_encode_u16_two_bytes() {
        assert_eq!(encode_u16(128), vec![0x80, 0x01]);
        assert_eq!(encode_u16(300), vec![0xAC, 0x02]);
        assert_eq!(encode_u16(16_383), vec![0xFF, 0x7F]);
    }

    #[test]
    fn test_encode_u16_step_by_step() {
        let mut value = 300_u32;
        let mut result = Vec::new();

        let mut byte = (value & 0x7F) as u8;
        assert_eq!(byte, 44);

        value >>= 7;
        assert_eq!(value, 2);

        byte |= 0x80;
        assert_eq!(byte, 0xAC);
        result.push(byte);

        byte = (value & 0x7F) as u8;
        assert_eq!(byte, 2);

        value >>= 7;
        assert_eq!(value, 0);

        assert_eq!(byte, 0x02);
        result.push(byte);

        assert_eq!(result, vec![0xAC, 0x02]);
        assert_eq!(encode_u32(300), vec![0xAC, 0x02]);
    }

    #[test]
    fn test_encode_max_u16() {
        let encoded = encode_u16(u16::MAX);
        assert_eq!(encoded.len(), 3);
        assert_eq!(encoded, vec![0xFF, 0xFF, 0x03]);
    }

    #[test]
    fn test_encode_u32_single_byte() {
        assert_eq!(encode_u32(0), vec![0x00]);
        assert_eq!(encode_u32(1), vec![0x01]);
        assert_eq!(encode_u32(5), vec![0x05]);
        assert_eq!(encode_u32(127), vec![0x7F]); // Max single byte
    }

    #[test]
    fn test_encode_u32_two_bytes() {
        // 128 = minimum 2-byte value
        assert_eq!(encode_u32(128), vec![0x80, 0x01]);

        // 300
        assert_eq!(encode_u32(300), vec![0xAC, 0x02]);

        // 16,383 = maximum 2-byte value
        assert_eq!(encode_u32(16_383), vec![0xFF, 0x7F]);
    }

    #[test]
    fn test_encode_u32_step_by_step() {
        let mut value = 300_u32;
        let mut result = Vec::new();

        let mut byte = (value & 0x7F) as u8; // 0b010_1100 = 44
        assert_eq!(byte, 44);

        value >>= 7;
        assert_eq!(value, 2);

        byte |= 0x80;
        assert_eq!(byte, 0xAC);
        result.push(byte);

        byte = (value & 0x7F) as u8;
        assert_eq!(byte, 2);

        value >>= 7;
        assert_eq!(value, 0);

        assert_eq!(byte, 0x02);
        result.push(byte);

        assert_eq!(result, vec![0xAC, 0x02]);
        assert_eq!(encode_u32(300), vec![0xAC, 0x02]);
    }

    #[test]
    fn test_encode_max_u32() {
        let encoded = encode_u32(u32::MAX);
        assert_eq!(encoded.len(), 5);
        assert_eq!(encoded, vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
    }

    #[test]
    fn test_encode_u64_single_byte() {
        assert_eq!(encode_u64(0), vec![0x00]);
        assert_eq!(encode_u64(1), vec![0x01]);
        assert_eq!(encode_u64(5), vec![0x05]);
        assert_eq!(encode_u64(127), vec![0x7F]); // Max single byte
    }

    #[test]
    fn test_encode_u64_two_bytes() {
        // 128 = minimum 2-byte value
        assert_eq!(encode_u64(128), vec![0x80, 0x01]);

        // 300
        assert_eq!(encode_u64(300), vec![0xAC, 0x02]);

        // 16,383 = maximum 2-byte value
        assert_eq!(encode_u64(16_383), vec![0xFF, 0x7F]);
    }

    #[test]
    fn test_encode_u64_step_by_step_300() {
        // Let's manually trace encoding 300
        let mut value = 300_u64;
        let mut result = Vec::new();

        // Step 1: value = 300 = 0b1_0010_1100
        let mut byte = (value & 0x7F) as u8; // 0b010_1100 = 44
        assert_eq!(byte, 44);

        value >>= 7; // value = 0b10 = 2
        assert_eq!(value, 2);

        byte |= 0x80; // Add continuation bit: 44 | 128 = 172 = 0xAC
        assert_eq!(byte, 0xAC);
        result.push(byte);

        // Step 2: value = 2
        byte = (value & 0x7F) as u8; // 0b10 = 2
        assert_eq!(byte, 2);

        value >>= 7; // value = 0
        assert_eq!(value, 0);

        // No continuation bit (value is 0)
        assert_eq!(byte, 0x02);
        result.push(byte);

        // Final result
        assert_eq!(result, vec![0xAC, 0x02]);
        assert_eq!(encode_u64(300), vec![0xAC, 0x02]);
    }

    #[test]
    fn test_encode_max_u64() {
        let encoded = encode_u64(u64::MAX);
        assert_eq!(encoded.len(), 10);
        assert_eq!(
            encoded,
            vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
    }
}
