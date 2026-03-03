use std::path::Path;

use chardetng::EncodingDetector;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("read file failed: {0}")]
    Read(#[from] std::io::Error),
    #[error("utf16 decode failed: {0}")]
    Utf16(#[from] std::string::FromUtf16Error),
    #[error("decode text failed")]
    Decode,
}

pub fn read_text_with_fallback(path: &Path) -> Result<String, LoadError> {
    let bytes = std::fs::read(path)?;
    decode_text_with_fallback(&bytes)
}

pub fn write_text_utf8(path: &Path, text: &str) -> Result<(), std::io::Error> {
    std::fs::write(path, text.as_bytes())
}

fn decode_text_with_fallback(bytes: &[u8]) -> Result<String, LoadError> {
    if let Ok(utf8) = String::from_utf8(bytes.to_vec()) {
        return Ok(utf8);
    }

    if let Some(utf16) = try_decode_utf16_with_bom(bytes)? {
        return Ok(utf16);
    }

    let mut detector = EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, true);
    let (cow, _, had_errors) = encoding.decode(bytes);
    if !had_errors {
        return Ok(cow.into_owned());
    }

    let (gbk_cow, _, gbk_had_errors) = encoding_rs::GBK.decode(bytes);
    if !gbk_had_errors {
        return Ok(gbk_cow.into_owned());
    }

    Err(LoadError::Decode)
}

fn try_decode_utf16_with_bom(bytes: &[u8]) -> Result<Option<String>, LoadError> {
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return decode_utf16(&bytes[2..], Endian::Little).map(Some);
    }

    if bytes.starts_with(&[0xFE, 0xFF]) {
        return decode_utf16(&bytes[2..], Endian::Big).map(Some);
    }

    Ok(None)
}

#[derive(Copy, Clone)]
enum Endian {
    Little,
    Big,
}

fn decode_utf16(bytes: &[u8], endian: Endian) -> Result<String, LoadError> {
    if !bytes.len().is_multiple_of(2) {
        return Err(LoadError::Decode);
    }

    let mut units = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks_exact(2) {
        let unit = match endian {
            Endian::Little => u16::from_le_bytes([chunk[0], chunk[1]]),
            Endian::Big => u16::from_be_bytes([chunk[0], chunk[1]]),
        };
        units.push(unit);
    }

    Ok(String::from_utf16(&units)?)
}
