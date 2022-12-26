use std::str::FromStr;

#[derive(Debug)]
struct ChunkType {
    bytes: [u8; 4],
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();
        Ok(ChunkType {
            bytes: [
                c.next().unwrap() as u8,
                c.next().unwrap() as u8,
                c.next().unwrap() as u8,
                c.next().unwrap() as u8,
            ],
        })
    }
}

impl From<[u8; 4]> for ChunkType {
    fn from(value: [u8; 4]) -> Self {
        ChunkType { bytes: value }
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes
            .iter()
            .zip(other.bytes.iter())
            .all(|(a, b)| a == b)
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_critical(&self) -> bool {
        match self.bytes[0] & (1 << 5) {
            1 => false,
            0 => true,
            _ => false,
        }
    }

    // TODO: change these to use the format in is_safe_to_copy
    fn is_public(&self) -> bool {
        match self.bytes[1] & (1 << 5) {
            1 => false,
            0 => true,
            _ => false,
        }
    }

    fn is_reserved_bit_valid(&self) -> bool {
        matches!(self.bytes[2] & (1 << 5), 0)
    }

    fn is_safe_to_copy(&self) -> bool {
        (self.bytes[3] & (1 << 5)) != 0
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
    //
    // #[test]
    // pub fn test_valid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_valid());
    // }
    //
    // #[test]
    // pub fn test_invalid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_valid());
    //
    //     let chunk = ChunkType::from_str("Ru1t");
    //     assert!(chunk.is_err());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_string() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert_eq!(&chunk.to_string(), "RuSt");
    // }
    //
    // #[test]
    // pub fn test_chunk_type_trait_impls() {
    //     let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //     let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //     let _chunk_string = format!("{}", chunk_type_1);
    //     let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    // }
}
