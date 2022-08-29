use std::convert::TryFrom;
use std::fmt;
use std::str::{from_utf8, FromStr};
use std::u8;

#[derive(Debug, PartialEq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        let lowercase = 65u8..90;
        let uppercase = 97u8..122;
        //Check each byte to see if it contains the correct bytes
        for b in bytes {
            if lowercase.contains(&b) || uppercase.contains(&b) {
                continue;
            } else {
                return Err(());
            }
        }
        Ok(ChunkType { bytes: bytes })
    }
}

impl FromStr for ChunkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(());
        }
        let sb = s.as_bytes();
        let bytes = [sb[0], sb[1], sb[2], sb[3]];

        ChunkType::try_from(bytes)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        let valid = self.bytes;
        valid
    }
    pub fn is_valid(&self) -> bool {
        let lowercase = 65u8..90;
        let valid = lowercase.contains(&self.bytes[2]);
        valid
    }

    pub fn is_critical(&self) -> bool {
        let lowercase = 65u8..90;
        let crit = lowercase.contains(&self.bytes[0]);
        crit
    }

    pub fn is_public(&self) -> bool {
        let lowercase = 65u8..90;
        let public = lowercase.contains(&self.bytes[1]);
        public
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let lowercase = 65u8..90;
        let valid = lowercase.contains(&self.bytes[2]);
        valid
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let lowercase = 65u8..90;
        let safe = lowercase.contains(&self.bytes[3]);
        !safe
    }
}
/*
#[allow(unused_variables)]
#[warn(unused_comparisons)]
#[warn(unnameable_test_items)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

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
*/
