use bitstream_io::{FromBitStream, ToBitStream};
use paste::paste;
use std::{char, cmp::max};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SmlHeader {
    bool: u8,
    u8: u8,
    u16: u8,
    u32: u8,
    u64: u8,
    u128: u8,
    i8: u8,
    i16: u8,
    i32: u8,
    i64: u8,
    i128: u8,
    char: u8,
}

impl SmlHeader {
    pub(crate) const TYPE_MIN_BITS: u32 = u8::BITS - 1;
    pub const TYPE_IDENT_BITS: u8 = 4;
    pub const BOOL_IDENT: u8 = 0;
    pub const U8_IDENT: u8 = 1;
    pub const U16_IDENT: u8 = 2;
    pub const U32_IDENT: u8 = 3;
    pub const U64_IDENT: u8 = 4;
    pub const U128_IDENT: u8 = 5;
    pub const I8_IDENT: u8 = 6;
    pub const I16_IDENT: u8 = 7;
    pub const I32_IDENT: u8 = 8;
    pub const I64_IDENT: u8 = 9;
    pub const I128_IDENT: u8 = 10;
    pub const CHAR_IDENT: u8 = 11;
    pub const ARRAY_IDENT: u8 = 12;
    pub const FLOAT_IDENT: u8 = 13;
    pub const STRUCT_IDENT: u8 = 14;
    pub const OPTION_IDENT: u8 = 15;
    pub const EOS_IDENT: u8 = 16;

    pub(crate) fn combine(&self, other: &SmlHeader) -> SmlHeader {
        SmlHeader {
            bool: max(self.bool, other.bool),
            char: max(self.char, other.char),
            i8: max(self.i8, other.i8),
            i16: max(self.i16, other.i16),
            i32: max(self.i32, other.i32),
            i64: max(self.i64, other.i64),
            i128: max(self.i128, other.i128),
            u8: max(self.u8, other.u8),
            u16: max(self.u16, other.u16),
            u32: max(self.u32, other.u32),
            u64: max(self.u64, other.u64),
            u128: max(self.u128, other.u128),
        }
    }
}

impl Default for SmlHeader {
    fn default() -> Self {
        Self {
            bool: 1,
            char: 2,
            i8: i8::BITS as u8,
            i16: i16::BITS as u8,
            i32: i32::BITS as u8,
            i64: i64::BITS as u8,
            i128: i128::BITS as u8,
            u8: u8::BITS as u8,
            u16: u16::BITS as u8,
            u32: u32::BITS as u8,
            u64: u64::BITS as u8,
            u128: u128::BITS as u8,
        }
    }
}

macro_rules! impl_get_bit_size {
    ($t:ty) => {
        paste! {
            impl SmlHeader {
                pub fn [<$t _bits>](&self) -> u8 {
                    self.$t
                }
            }
        }
    };
}

impl_get_bit_size!(bool);
impl_get_bit_size!(char);
impl_get_bit_size!(i8);
impl_get_bit_size!(i16);
impl_get_bit_size!(i32);
impl_get_bit_size!(i64);
impl_get_bit_size!(i128);
impl_get_bit_size!(u8);
impl_get_bit_size!(u16);
impl_get_bit_size!(u32);
impl_get_bit_size!(u64);
impl_get_bit_size!(u128);

impl ToBitStream for SmlHeader {
    type Error = std::io::Error;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(
        &self,
        writer: &mut W,
    ) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        writer.write(Self::TYPE_MIN_BITS, self.bool)?;
        writer.write(Self::TYPE_MIN_BITS, self.char)?;
        writer.write(Self::TYPE_MIN_BITS, self.i8)?;
        writer.write(Self::TYPE_MIN_BITS, self.i16)?;
        writer.write(Self::TYPE_MIN_BITS, self.i32)?;
        writer.write(Self::TYPE_MIN_BITS, self.i64)?;
        writer.write(Self::TYPE_MIN_BITS, self.i128)?;
        writer.write(Self::TYPE_MIN_BITS, self.u8)?;
        writer.write(Self::TYPE_MIN_BITS, self.u16)?;
        writer.write(Self::TYPE_MIN_BITS, self.u32)?;
        writer.write(Self::TYPE_MIN_BITS, self.u64)?;
        writer.write(Self::TYPE_MIN_BITS, self.u128)?;
        writer.byte_align()?;
        Ok(())
    }
}

impl FromBitStream for SmlHeader {
    type Error = std::io::Error;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(reader: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let sml_header = Self {
            bool: reader.read(Self::TYPE_MIN_BITS)?,
            char: reader.read(Self::TYPE_MIN_BITS)?,
            i8: reader.read(Self::TYPE_MIN_BITS)?,
            i16: reader.read(Self::TYPE_MIN_BITS)?,
            i32: reader.read(Self::TYPE_MIN_BITS)?,
            i64: reader.read(Self::TYPE_MIN_BITS)?,
            i128: reader.read(Self::TYPE_MIN_BITS)?,
            u8: reader.read(Self::TYPE_MIN_BITS)?,
            u16: reader.read(Self::TYPE_MIN_BITS)?,
            u32: reader.read(Self::TYPE_MIN_BITS)?,
            u64: reader.read(Self::TYPE_MIN_BITS)?,
            u128: reader.read(Self::TYPE_MIN_BITS)?,
        };
        reader.byte_align();
        Ok(sml_header)
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

    pub fn build(&self) -> SmlHeader {
        SmlHeader {
            bool: 1,
            char: self.char.unwrap_or(2),
            i8: self.i8.unwrap_or(i8::BITS as u8),
            i16: self.i16.unwrap_or(i16::BITS as u8),
            i32: self.i32.unwrap_or(i32::BITS as u8),
            i64: self.i64.unwrap_or(i64::BITS as u8),
            i128: self.i128.unwrap_or(i128::BITS as u8),
            u8: self.u8.unwrap_or(u8::BITS as u8),
            u16: self.u16.unwrap_or(u16::BITS as u8),
            u32: self.u32.unwrap_or(u32::BITS as u8),
            u64: self.u64.unwrap_or(u64::BITS as u8),
            u128: self.u128.unwrap_or(u128::BITS as u8),
        }
    }
}

macro_rules! impl_set_bit_size {
    ($t:ty) => {
        paste! {
            impl SmlHeaderBuilder {
                pub fn [<with_ $t _bits>](mut self, bits: u8) -> Self {
                    debug_assert!(bits <= 128);
                    self.$t = Some(bits);
                    self
                }
            }
        }
    };
}

impl_set_bit_size!(char);
impl_set_bit_size!(i8);
impl_set_bit_size!(i16);
impl_set_bit_size!(i32);
impl_set_bit_size!(i64);
impl_set_bit_size!(i128);
impl_set_bit_size!(u8);
impl_set_bit_size!(u16);
impl_set_bit_size!(u32);
impl_set_bit_size!(u64);
impl_set_bit_size!(u128);

impl From<SmlHeaderBuilder> for SmlHeader {
    fn from(builder: SmlHeaderBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod sml_header_tests {
    use std::io::{self, Cursor};

    use bitstream_io::{BigEndian, BitRead, BitReader, BitWrite, BitWriter};

    use super::*;

    #[test]
    fn sml_header_builder() {
        let sml_header: SmlHeader = SmlHeaderBuilder::new()
            .with_char_bits(10)
            .with_u8_bits(8)
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
            }
        );
    }

    #[test]
    fn sml_header_write() -> io::Result<()> {
        let sml_header: SmlHeader = SmlHeader::default();
        let mut data = Vec::new();
        let mut writer = BitWriter::endian(Cursor::new(&mut data), BigEndian);
        writer.build(&sml_header)?;
        assert_eq!(&data, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        Ok(())
    }

    #[test]
    fn sml_header_read() -> io::Result<()> {
        let data = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut cursor = Cursor::new(data);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        assert_eq!(reader.parse::<SmlHeader>()?, SmlHeader::default());
        Ok(())
    }
}
