use crate::header::SmlHeaderBuilder;
use bitstream_io::BitWrite;
use std::io;

use crate::header::SmlHeader;

pub(crate) trait ToSmlStream {
    fn sml_header(&self) -> SmlHeader;
    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()>;
}

impl ToSmlStream for u8 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_u8_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        writer.write(header.u8_bits().into(), *self)
    }
}

impl ToSmlStream for u16 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_u16_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        const U8_MAX: u16 = u8::MAX as u16;
        match self {
            ..=U8_MAX => (*self as u8).sml_write(writer, header),
            _ => writer.write(header.u16_bits().into(), *self),
        }
    }
}


impl ToSmlStream for u32 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_u32_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        const U8_MAX: u32 = u8::MAX as u32;
        const U16_MAX: u32 = u16::MAX as u32;
        match self {
            ..=U8_MAX => (*self as u8).sml_write(writer, header),
            ..=U16_MAX => (*self as u16).sml_write(writer, header),
            _ => writer.write(header.u32_bits().into(), *self),
        }
    }
}


impl ToSmlStream for u64 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_u64_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        const U8_MAX: u64 = u8::MAX as u64;
        const U16_MAX: u64 = u16::MAX as u64;
        const U32_MAX: u64 = u32::MAX as u64;
        match self {
            ..=U8_MAX => (*self as u8).sml_write(writer, header),
            ..=U16_MAX => (*self as u16).sml_write(writer, header),
            ..=U32_MAX => (*self as u32).sml_write(writer, header),
            _ => writer.write(header.u64_bits().into(), *self),
        }
    }
}

impl ToSmlStream for u128 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_u128_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        const U8_MAX: u128 = u8::MAX as u128;
        const U16_MAX: u128 = u16::MAX as u128;
        const U32_MAX: u128 = u32::MAX as u128;
        const U64_MAX: u128 = u64::MAX as u128;
        match self {
            ..=U8_MAX => (*self as u8).sml_write(writer, header),
            ..=U16_MAX => (*self as u16).sml_write(writer, header),
            ..=U32_MAX => (*self as u32).sml_write(writer, header),
            ..=U64_MAX => (*self as u64).sml_write(writer, header),
            _ => writer.write(header.u128_bits().into(), *self),
        }
    }
}


impl ToSmlStream for i8 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_i8_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        writer.write(header.i8_bits().into(), *self)
    }
}

impl ToSmlStream for i16 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_i16_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        const I8_MAX: i16 = i8::MAX as i16;
        match self {
            ..=I8_MAX => (*self as i8).sml_write(writer, header), 
            _ => writer.write(header.i16_bits().into(), *self)
        }
        
    }
}


impl ToSmlStream for i32 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_i32_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        const I8_MAX: i32 = i8::MAX as i32;
        const I16_MAX: i32 = i16::MAX as i32;
        match self {
            ..=I8_MAX => (*self as i8).sml_write(writer, header),
            ..=I16_MAX => (*self as i16).sml_write(writer, header),
            _ => writer.write(header.i16_bits().into(), *self)
        }
    }
}


impl ToSmlStream for i64 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_i64_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        const I8_MAX: i64 = i8::MAX as i64;
        const I16_MAX: i64 = i16::MAX as i64;
        const I32_MAX: i64 = i32::MAX as i64;
        match self {
            ..=I8_MAX => (*self as i8).sml_write(writer, header),
            ..=I16_MAX => (*self as i16).sml_write(writer, header),
            ..=I32_MAX => (*self as i32).sml_write(writer, header),
            _ => writer.write(header.i64_bits().into(), *self)
        }
    }
}


impl ToSmlStream for i128 {
    fn sml_header(&self) -> SmlHeader {
        let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
        SmlHeaderBuilder::new().with_i128_bits(size).build()
    }

    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        const I8_MAX: i128 = i8::MAX as i128;
        const I16_MAX: i128 = i16::MAX as i128;
        const I32_MAX: i128 = i32::MAX as i128;
        const I64_MAX: i128 = i64::MAX as i128;
        match self {
            ..=I8_MAX => (*self as i8).sml_write(writer, header),
            ..=I16_MAX => (*self as i16).sml_write(writer, header),
            ..=I32_MAX => (*self as i32).sml_write(writer, header),
            ..=I64_MAX => (*self as i64).sml_write(writer, header),
            _ => writer.write(header.i128_bits().into(), *self)
        }
    }
}