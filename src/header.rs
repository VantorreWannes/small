use bitstream_io::{FromBitStream, ToBitStream};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SmlHeader {
    bool: u8,
    char: u8,
    i8: u8,
    i16: u8,
    i32: u8,
    i64: u8,
    i128: u8,
    u8: u8,
    u16: u8,
    u32: u8,
    u64: u8,
    u128: u8,
    f32: u8,
    f64: u8,
}

impl SmlHeader {
    const BITS: u32 = u8::BITS - 1;
}

impl ToBitStream for SmlHeader {
    type Error = std::io::Error;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(
        &self,
        writer: &mut W,
    ) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        writer.write(Self::BITS, self.bool)?;
        writer.write(Self::BITS, self.char)?;
        writer.write(Self::BITS, self.i8)?;
        writer.write(Self::BITS, self.i16)?;
        writer.write(Self::BITS, self.i32)?;
        writer.write(Self::BITS, self.i64)?;
        writer.write(Self::BITS, self.i128)?;
        writer.write(Self::BITS, self.u8)?;
        writer.write(Self::BITS, self.u16)?;
        writer.write(Self::BITS, self.u32)?;
        writer.write(Self::BITS, self.u64)?;
        writer.write(Self::BITS, self.u128)?;
        writer.write(Self::BITS, self.f32)?;
        writer.write(Self::BITS, self.f64)?;
        Ok(())
    }
}

impl FromBitStream for SmlHeader {
    type Error = std::io::Error;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            bool: r.read(Self::BITS)?,
            char: r.read(Self::BITS)?,
            i8: r.read(Self::BITS)?,
            i16: r.read(Self::BITS)?,
            i32: r.read(Self::BITS)?,
            i64: r.read(Self::BITS)?,
            i128: r.read(Self::BITS)?,
            u8: r.read(Self::BITS)?,
            u16: r.read(Self::BITS)?,
            u32: r.read(Self::BITS)?,
            u64: r.read(Self::BITS)?,
            u128: r.read(Self::BITS)?,
            f32: r.read(Self::BITS)?,
            f64: r.read(Self::BITS)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SmlHeaderBuilder {
    char: Option<u8>,
    i16: Option<u8>,
    i8: Option<u8>,
    i64: Option<u8>,
    i32: Option<u8>,
    i128: Option<u8>,
    u8: Option<u8>,
    u16: Option<u8>,
    u32: Option<u8>,
    u64: Option<u8>,
    u128: Option<u8>,
    f32: Option<u8>,
    f64: Option<u8>,
}

impl SmlHeaderBuilder {
    pub fn new() -> Self {
        Self {
            char: None,
            i16: None,
            i8: None,
            i64: None,
            i32: None,
            i128: None,
            u8: None,
            u16: None,
            u32: None,
            u64: None,
            u128: None,
            f32: None,
            f64: None,
        }
    }

    pub fn with_char_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.char = Some(bits);
        self
    }

    pub fn with_i8_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.i8 = Some(bits);
        self
    }

    pub fn with_i16_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.i16 = Some(bits);
        self
    }

    pub fn with_i32_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.i32 = Some(bits);
        self
    }

    pub fn with_i64_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.i64 = Some(bits);
        self
    }

    pub fn with_i128_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.i128 = Some(bits);
        self
    }

    pub fn with_u8_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.u8 = Some(bits);
        self
    }

    pub fn with_u16_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.u16 = Some(bits);
        self
    }

    pub fn with_u32_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.u32 = Some(bits);
        self
    }

    pub fn with_u64_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.u64 = Some(bits);
        self
    }

    pub fn with_u128_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.u128 = Some(bits);
        self
    }

    pub fn with_f32_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.f32 = Some(bits);
        self
    }

    pub fn with_f64_size(mut self, bits: u8) -> Self {
        debug_assert!(bits <= 128);
        self.f64 = Some(bits);
        self
    }

    pub fn to_sml_header(&self) -> SmlHeader {
        SmlHeader {
            bool: 1,
            char: self.char.unwrap_or(0),
            i8: self.i8.unwrap_or(0),
            i16: self.i16.unwrap_or(0),
            i32: self.i32.unwrap_or(0),
            i64: self.i64.unwrap_or(0),
            i128: self.i128.unwrap_or(0),
            u8: self.u8.unwrap_or(0),
            u16: self.u16.unwrap_or(0),
            u32: self.u32.unwrap_or(0),
            u64: self.u64.unwrap_or(0),
            u128: self.u128.unwrap_or(0),
            f32: self.f32.unwrap_or(0),
            f64: self.f64.unwrap_or(0),
        }
    }
}

impl From<SmlHeaderBuilder> for SmlHeader {
    fn from(builder: SmlHeaderBuilder) -> Self {
        builder.to_sml_header()
    }
}

#[cfg(test)]
mod sml_header_tests {
    use super::*;

    #[test]
    fn sml_header_builder() {
        let sml_header: SmlHeader = SmlHeaderBuilder::new()
            .with_char_size(10)
            .with_u8_size(8)
            .into();
        assert_eq!(
            sml_header,
            SmlHeader {
                bool: 1,
                char: 10,
                i8: 0,
                i16: 0,
                i32: 0,
                i64: 0,
                i128: 0,
                u8: 8,
                u16: 0,
                u32: 0,
                u64: 0,
                u128: 0,
                f32: 0,
                f64: 0,
            }
        );
    }
}
