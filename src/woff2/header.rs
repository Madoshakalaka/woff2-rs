//! The WOFF2 header

use bytes::Buf;
use four_cc::FourCC;
use thiserror::Error;

use crate::{buffer_util::BufExt, magic_numbers::WOFF2_SIGNATURE};

#[derive(Error, Debug)]
pub enum Woff2HeaderError {
    #[error("Truncated header")]
    Truncated,
    #[error("Invalid magic word")]
    InvalidMagicWord,
    #[error("Excess padding")]
    ExcessPadding,
    #[error("Overlapping streams")]
    OverlappingStreams,
}

pub struct Woff2Header {
    pub signature: FourCC,
    pub flavor: FourCC,
    pub num_tables: u16,
    pub total_sfnt_size: u32,
    pub total_compressed_size: u32,
}

impl Woff2Header {
    pub fn from_buf(buffer: &mut impl Buf) -> Result<Self, Woff2HeaderError> {
        if buffer.remaining() < 48 {
            return Err(Woff2HeaderError::Truncated);
        }

        let signature = buffer.get_four_cc();
        let flavor = buffer.get_four_cc();
        buffer.advance(4);
        let num_tables = buffer.get_u16();
        buffer.advance(2);
        let total_sfnt_size = buffer.get_u32();
        let total_compressed_size = buffer.get_u32();
        buffer.advance(24);

        Ok(Self {
            signature,
            flavor,
            num_tables,
            total_sfnt_size,
            total_compressed_size,
        })
    }

    pub fn is_valid_header(&self) -> Result<(), Woff2HeaderError> {
        if self.signature != WOFF2_SIGNATURE {
            return Err(Woff2HeaderError::InvalidMagicWord);
        }

        // TODO: Add other checks

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::test_resources::LATO_V22_LATIN_REGULAR;

    use super::Woff2Header;

    #[test]
    fn test_header() {
        let mut buffer = Cursor::new(LATO_V22_LATIN_REGULAR);
        let header = Woff2Header::from_buf(&mut buffer).unwrap();
        assert!(header.is_valid_header().is_ok());
    }
}
