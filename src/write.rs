use bitstream_io::BitWrite;
use std::io;

use crate::TYPE_BIT_SIZE;

pub trait WriteSml {
    fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()>;
    fn sml_write_type<W: BitWrite>(&self, writer: &mut W) -> io::Result<()>;
    fn sml_write<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
        self.sml_write_type(writer)?;
        self.sml_write_value(writer)
    }
}

impl WriteSml for bool {
    fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
        let value: u8 = match self {
            true => 1,
            false => 0,
        };
        writer.write(1, value)
    }

    fn sml_write_type<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
        writer.write(TYPE_BIT_SIZE.into(), 0u8)
    }
}

impl WriteSml for char {
    fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
        let mut bytes = vec![0u8; self.len_utf8()];
        self.encode_utf8(&mut bytes);
        writer.write(2, (self.len_utf8() - 1) as u8)?;
        writer.write_bytes(&bytes)
    }

    fn sml_write_type<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
        writer.write(TYPE_BIT_SIZE.into(), 1u8)
    }
}
