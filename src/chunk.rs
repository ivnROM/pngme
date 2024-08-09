#![allow(unused_variables)]
use std::{fmt::Display, io::Read};
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::chunk_type::ChunkType;
use crate::{Error, Result};

enum ChunkError {
    UnreadableByte,
}

// implementar esto
impl std::error::Error for ChunkError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }

    fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}

pub struct Chunk {
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    length: u32,
    crc: u32,
}

impl Chunk {
    const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
        let length: u32 = chunk_data.bytes()
                                    .count()
                                    .try_into()
                                    .unwrap();
        let crc_sum = Chunk::get_checksum(chunk_data.clone(), chunk_type.bytes());
        Chunk {
            chunk_type,
            chunk_data,
            length,
            crc: crc_sum,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.chunk_data.as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let data = self.data().bytes();
        let mut string = String::new();
        for byte in data {
            let byte = match byte {
                Ok(val) => val,
                Err(_) => return Err(),
            };
            string.push(byte as char);            
        }
        return Ok(string)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let byte_vec = Vec::<u8>::new();
        let byte_vec = byte_vec
           .iter()
           .cloned()
           .chain(self.length.to_be_bytes())
           .chain(self.chunk_type.bytes())
           .chain(self.chunk_data.iter().cloned())
           .chain(self.crc.to_be_bytes())
           .collect();
        return byte_vec
    }

    fn get_checksum(mut chunk_data: Vec<u8>, chunk_type_code: [u8; 4]) -> u32 {
        chunk_data.extend_from_slice(&chunk_type_code);
        let chunk_data = &chunk_data[..];
        let sum = Chunk::CRC.checksum(chunk_data);
        sum
    }
}

// impl Display for Chunk {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     }
// }

fn main() {
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
                                        .to_be_bytes()
                                        .iter()
                                        .chain(chunk_type.iter())
                                        .chain(message_bytes.iter())
                                        .chain(crc.to_be_bytes().iter())
                                        .copied()
                                        .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
        }

    }
}

