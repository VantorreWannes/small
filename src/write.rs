use bitstream_io::BitWrite;
use std::{fmt::Write, io};

use crate::TYPE_IDENT_BIT_SIZE;

pub trait WriteSml {
    fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()>;
    fn sml_write_type<W: BitWrite>(writer: &mut W) -> io::Result<()>;
    fn sml_write<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
        Self::sml_write_type(writer)?;
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

    fn sml_write_type<W: BitWrite>(writer: &mut W) -> io::Result<()> {
        writer.write(TYPE_IDENT_BIT_SIZE.into(), 0u8)
    }
}

impl WriteSml for char {
    fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
        let mut bytes = vec![0u8; self.len_utf8()];
        self.encode_utf8(&mut bytes);
        writer.write(2, (self.len_utf8() - 1) as u8)?;
        writer.write_bytes(&bytes)
    }

    fn sml_write_type<W: BitWrite>(writer: &mut W) -> io::Result<()> {
        writer.write(TYPE_IDENT_BIT_SIZE.into(), 1u8)
    }
}

fn nibbles_needed(value: u64) -> u8 {
    if value == 0 {
        1
    } else {
        (((u64::BITS - value.leading_zeros()) + 3) / 4) as u8
    }
}

macro_rules! impl_write_sml_for_unsigned {
    ($t:ty) => {
        impl WriteSml for $t {
            fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
                let nibbles = nibbles_needed((*self).into());
                false.sml_write_value(writer)?;
                writer.write(3, nibbles - 1)?;
                writer.write((nibbles * 4).into(), *self)
            }

            fn sml_write_type<W: BitWrite>(writer: &mut W) -> io::Result<()> {
                writer.write(TYPE_IDENT_BIT_SIZE.into(), 2u8)
            }
        }
    };
}

macro_rules! impl_write_sml_for_signed {
    ($t:ty) => {
        impl WriteSml for $t {
            fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
                let nibbles = nibbles_needed((self.abs()) as u64);
                (*self < 0).sml_write_value(writer)?;
                writer.write(3, nibbles - 1)?;
                writer.write((nibbles * 4).into(), *self)
            }

            fn sml_write_type<W: BitWrite>(writer: &mut W) -> io::Result<()> {
                writer.write(TYPE_IDENT_BIT_SIZE.into(), 2u8)
            }
        }
    };
}

impl_write_sml_for_unsigned!(u8);
impl_write_sml_for_unsigned!(u16);
impl_write_sml_for_unsigned!(u32);
impl_write_sml_for_unsigned!(u64);
impl_write_sml_for_signed!(i8);
impl_write_sml_for_signed!(i16);
impl_write_sml_for_signed!(i32);
impl_write_sml_for_signed!(i64);

macro_rules! impl_write_sml_for_float {
    ($t:ty) => {
        impl WriteSml for $t {
            fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
                (Self::MANTISSA_DIGITS == 53).sml_write_value(writer)?;
                self.to_bits().sml_write_value(writer)
            }

            fn sml_write_type<W: BitWrite>(writer: &mut W) -> io::Result<()> {
                writer.write(TYPE_IDENT_BIT_SIZE.into(), 3u8)
            }
        }
    };
}

impl_write_sml_for_float!(f32);
impl_write_sml_for_float!(f64);

impl<T: WriteSml> WriteSml for Option<T> {
    fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
        self.is_some().sml_write(writer)?;
        if let Some(value) = self {
            value.sml_write(writer)?;
        }
        Ok(())
    }

    fn sml_write_type<W: BitWrite>(writer: &mut W) -> io::Result<()> {
        writer.write(TYPE_IDENT_BIT_SIZE.into(), 4u8)?;
        T::sml_write_type(writer)
    }
}

#[cfg(test)]
mod write_sml_tests {
    use super::*;
    use bitstream_io::{BigEndian, BitWriter};
    use paste::paste;

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
                    $t::sml_write_type(&mut writer)?;
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

    test_write_type!(char, vec![0b00100000]);
    test_write_value!(char, 'a', vec![0b00011000, 0b01000000]);

    test_write_type!(u8, vec![0b01000000]);
    test_write_value!(u8, 16u8, vec![0b00010001, 0b00000000]);

    test_write_type!(i8, vec![0b01000000]);
    test_write_value!(i8, 16i8, vec![0b00010001, 0b00000000]);

    #[test]
    fn write_option_bool_type() -> io::Result<()> {
        let mut data: Vec<u8> = vec![];
        let mut writer = BitWriter::endian(&mut data, BigEndian);
        Option::<bool>::sml_write_type(&mut writer)?;
        writer.byte_align()?;
        println!("{:08b}", &data[0]);
        Ok(())
    }
}
