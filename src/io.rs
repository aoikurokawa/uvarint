use std::io::{Read, Write};

use crate::{
    decode::{decode_u32, decode_u64},
    encode_u32, encode_u64,
    error::UVarintError,
};

/// Read a varint-encoded u32 from any `Read` implementation
///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
/// use uvarint::io::read_u32;
///
/// let data = vec![0xAC, 0x02];
/// let mut cursor = Cursor::new(data);
/// let value = read_u32(&mut cursor).unwrap();
/// assert_eq!(value, 300);
/// ```
pub fn read_u32<R: Read>(reader: &mut R) -> Result<u32, UVarintError> {
    let mut buf = [0u8; 10];
    let mut bytes_read = 0;

    for i in 0..10 {
        reader
            .read_exact(&mut buf[i..i + 1])
            .map_err(|_| UVarintError::Incomplete)?;

        bytes_read = i + 1;

        if (buf[i] & 0x80) == 0 {
            break;
        }
    }

    let (_, value) = decode_u32(&buf[..bytes_read])?;
    Ok(value)
}

/// Read a varint-encoded u64 from any `Read` implementation
///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
/// use uvarint::io::read_u64;
///
/// let data = vec![0xAC, 0x02];
/// let mut cursor = Cursor::new(data);
/// let value = read_u64(&mut cursor).unwrap();
/// assert_eq!(value, 300);
/// ```
pub fn read_u64<R: Read>(reader: &mut R) -> Result<u64, UVarintError> {
    let mut buf = [0u8; 10];
    let mut bytes_read = 0;

    for i in 0..10 {
        reader
            .read_exact(&mut buf[i..i + 1])
            .map_err(|_| UVarintError::Incomplete)?;

        bytes_read = i + 1;

        if (buf[i] & 0x80) == 0 {
            break;
        }
    }

    let (_, value) = decode_u64(&buf[..bytes_read])?;
    Ok(value)
}

/// Write a varint-encoded u64 to any `Write` implementation
///
/// # Examples
///
/// ```rust
/// use uvarint::io::write_u64;
///
/// let mut buf = Vec::new();
/// write_u64(&mut buf, 300).unwrap();
/// assert_eq!(buf, vec![0xAC, 0x02]);
/// ```
pub fn write_u64<W: Write>(writer: &mut W, value: u64) -> Result<usize, UVarintError> {
    let bytes = encode_u64(value);
    writer
        .write_all(&bytes)
        .map_err(|_| UVarintError::WriteFailed)?;
    Ok(bytes.len())
}

/// Write a varint-encoded u32 to any `Write` implementation
///
/// # Examples
///
/// ```rust
/// use uvarint::io::write_u32;
///
/// let mut buf = Vec::new();
/// write_u32(&mut buf, 300).unwrap();
/// assert_eq!(buf, vec![0xAC, 0x02]);
/// ```
pub fn write_u32<W: Write>(writer: &mut W, value: u32) -> Result<usize, UVarintError> {
    let bytes = encode_u32(value);
    writer
        .write_all(&bytes)
        .map_err(|_| UVarintError::WriteFailed)?;
    Ok(bytes.len())
}

/// Extension trait for reading varints from `Read` types
///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
/// use uvarint::io::ReadVarintExt;
///
/// let data = vec![0xAC, 0x02, 0xFF, 0x01];
/// let mut cursor = Cursor::new(data);
///
/// assert_eq!(cursor.read_varint_u64().unwrap(), 300);
/// assert_eq!(cursor.read_varint_u64().unwrap(), 255);
/// ```
pub trait ReadVarintExt: Read + Sized {
    fn read_varint_u64(&mut self) -> Result<u64, UVarintError> {
        read_u64(self)
    }

    fn read_varint_u32(&mut self) -> Result<u32, UVarintError> {
        read_u32(self)
    }
}

impl<R: Read> ReadVarintExt for R {}

/// Extension trait for writing varints to `Write` types
///
/// # Examples
///
/// ```rust
/// use uvarint::io::WriteVarintExt;
///
/// let mut buf = Vec::new();
/// buf.write_varint_u64(300).unwrap();
/// buf.write_varint_u32(255).unwrap();
///
/// assert_eq!(buf, vec![0xAC, 0x02, 0xFF, 0x01]);
/// ```
pub trait WriteVarintExt: Write + Sized {
    fn write_varint_u64(&mut self, value: u64) -> Result<usize, UVarintError> {
        write_u64(self, value)
    }

    fn write_varint_u32(&mut self, value: u32) -> Result<usize, UVarintError> {
        write_u32(self, value)
    }
}

impl<W: Write> WriteVarintExt for W {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_u64() {
        let data = vec![0xAC, 0x02];
        let mut cursor = Cursor::new(data);
        assert_eq!(read_u64(&mut cursor).unwrap(), 300);
    }

    #[test]
    fn test_read_u32() {
        let data = vec![0xAC, 0x02];
        let mut cursor = Cursor::new(data);
        assert_eq!(read_u32(&mut cursor).unwrap(), 300);
    }

    #[test]
    fn test_write_u64() {
        let mut buf = Vec::new();
        let bytes_written = write_u64(&mut buf, 300).unwrap();
        assert_eq!(bytes_written, 2);
        assert_eq!(buf, vec![0xAC, 0x02]);
    }

    #[test]
    fn test_write_u32() {
        let mut buf = Vec::new();
        let bytes_written = write_u32(&mut buf, 300).unwrap();
        assert_eq!(bytes_written, 2);
        assert_eq!(buf, vec![0xAC, 0x02]);
    }

    #[test]
    fn test_read_multiple_varints() {
        let data = vec![
            0xAC, 0x02, // 300
            0xFF, 0x01, // 255
            0x05, // 5
        ];
        let mut cursor = Cursor::new(data);

        assert_eq!(cursor.read_varint_u64().unwrap(), 300);
        assert_eq!(cursor.read_varint_u64().unwrap(), 255);
        assert_eq!(cursor.read_varint_u64().unwrap(), 5);
    }

    #[test]
    fn test_write_multiple_varints() {
        let mut buf = Vec::new();

        buf.write_varint_u64(300).unwrap();
        buf.write_varint_u64(255).unwrap();
        buf.write_varint_u64(5).unwrap();

        assert_eq!(buf, vec![0xAC, 0x02, 0xFF, 0x01, 0x05]);
    }

    #[test]
    fn test_read_write_roundtrip() {
        let values = vec![0, 1, 127, 128, 300, 16_383, 16_384, u32::MAX as u64];

        for &value in &values {
            let mut buf = Vec::new();
            write_u64(&mut buf, value).unwrap();

            let mut cursor = Cursor::new(buf);
            let decoded = read_u64(&mut cursor).unwrap();

            assert_eq!(decoded, value);
        }
    }

    #[test]
    fn test_read_from_file() {
        use std::fs::File;
        use std::io::Write;

        // Write to temp file
        let mut file = File::create("/tmp/test_varint.bin").unwrap();
        file.write_all(&[0xAC, 0x02]).unwrap();
        drop(file);

        // Read back
        let mut file = File::open("/tmp/test_varint.bin").unwrap();
        let value = read_u64(&mut file).unwrap();
        assert_eq!(value, 300);

        std::fs::remove_file("/tmp/test_varint.bin").unwrap();
    }

    #[test]
    fn test_read_incomplete() {
        let data = vec![0x80];
        let mut cursor = Cursor::new(data);

        assert!(matches!(
            read_u64(&mut cursor),
            Err(UVarintError::Incomplete)
        ));
    }
}
