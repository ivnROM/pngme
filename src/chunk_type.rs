use std::fmt::Display;
use std::str::FromStr;
use crate::{Error, Result};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChunkType {
    code: [u8; 4],
}

#[derive(Debug)]
enum ChunkTypeErrors {
    IsNotAlphabetic,
}

impl std::error::Error for ChunkTypeErrors{}

impl Display for ChunkTypeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeErrors::IsNotAlphabetic => write!(f, "El caracter no se encuentra dentro de los rangos ASCII permitidos: 65-90 o 97-122"),
        }
    }
}


impl ChunkType {

    pub fn bytes(&self) -> [u8; 4] {
        self.code
    }

    pub fn is_critical(&self) -> bool {
        // primer byte [bit 5]
        let byte = self.code[0];
        let bit = (byte >> 5) & 1;
        bit == 0
    }

    pub fn is_public(&self) -> bool {
        // segundo byte 
        let byte = self.code[1];
        let bit = (byte >> 5) & 1;
        bit == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        // tercer byte 
        let byte = self.code[2];
        let bit = (byte >> 5) & 1;
        bit == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        // cuarto byte
        let byte = self.code[3];
        let bit = (byte >> 5) & 1;
        bit == 1
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

// Implementaciones de traits de datos primitivos
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self> {
        for byte in value {
            if !byte.is_ascii_alphabetic(){
                let err: Error = ChunkTypeErrors::IsNotAlphabetic.into();
                return Err(err);
            }
        }
        Ok(ChunkType {code: value})
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.as_bytes();
        let s: [u8; 4] = s[0..4].try_into()?;
        for byte in s {
            if !byte.is_ascii_alphabetic(){
                let err: Error = ChunkTypeErrors::IsNotAlphabetic.into();
                return Err(err);
            }
        }
        Ok(ChunkType {code: s})
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.code[0] as char, self.code[1] as char, self.code[2] as char, self.code[3] as char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
