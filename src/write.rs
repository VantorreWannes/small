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

#[cfg(test)]
mod write_sml_tests {
    use bitstream_io::{BigEndian, BitWriter};
    use paste::paste;
    use super::*;

    macro_rules! test_write_value {
        ($t:ty, $v:expr, $a:expr) => {
            paste! {
                #[test]
                fn [<write_ $t _value_ $v>]() -> io::Result<()> {
                    let mut data: Vec<u8> = vec![];
                    let mut writer = BitWriter::endian(&mut data, BigEndian);
                    $v.sml_write_value(&mut writer)?;
                    writer.byte_align()?;
                    //println!("{:08b}", &data[0]);
                    //println!("{:08b}", &data[1]);
                    assert_eq!(data, $a);
                    Ok(())
                }
            }
        };
    }

    macro_rules! test_write_type {
        ($t:ty, $v:expr) => {
            paste! {
                #[test]
                fn [<write_ $t _type>]() -> io::Result<()> {
                    let mut data: Vec<u8> = vec![];
                    let mut writer = BitWriter::endian(&mut data, BigEndian);
                    $t::default().sml_write_type(&mut writer)?;
                    writer.byte_align()?;
                    //println!("{:08b}", &data[0]);
                    assert_eq!(data, $v);
                    Ok(())
                }
            }
        };
    }

    test_write_type!(bool, vec![0b00000000]);
    test_write_value!(bool, true, vec![0b10000000]);
    test_write_value!(bool, false, vec![0b00000000]);
    
    test_write_type!(char, vec![0b10000000]);
    test_write_value!(char, 'a', vec![0b00011000, 0b01000000]);

    
}
