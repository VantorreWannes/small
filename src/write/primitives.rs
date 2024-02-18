use crate::header::SmlHeader;
use bitstream_io::BitWrite;
use paste::paste;
use std::io;

pub(crate) trait WriteSml {
    fn sml_write_value<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()>;
}

macro_rules! impl_to_sml_stream {
    ($t:ty) => {
        impl WriteSml for $t {
            fn sml_write_value<W: BitWrite>(
                &self,
                writer: &mut W,
                header: &SmlHeader,
            ) -> io::Result<()> {
                paste! {
                    writer.write(header.[<$t _bits>]().into(), *self)?;
                }
                Ok(())
            }
        }
    };
}

impl_to_sml_stream!(u8);
impl_to_sml_stream!(u16);
impl_to_sml_stream!(u32);
impl_to_sml_stream!(u64);
impl_to_sml_stream!(u128);
impl_to_sml_stream!(i8);
impl_to_sml_stream!(i16);
impl_to_sml_stream!(i32);
impl_to_sml_stream!(i64);
impl_to_sml_stream!(i128);

impl WriteSml for bool {
    fn sml_write_value<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
        let self_bits: u8 = match self {
            true => 1,
            false => 0,
        };
        writer.write(header.bool_bits().into(), self_bits)?;
        Ok(())
    }
}

#[cfg(test)]
mod sml_write_primitive_tests {
    use super::*;
    use bitstream_io::{BigEndian, BitWriter};
    macro_rules! write_test_sml_write_value {
        ($t:ty) => {
            paste! {
                #[test]
                fn [<$t _sml_write_value>]() -> io::Result<()> {
                    let header = SmlHeader::default();
                    let mut data = vec![];
                    let mut writer = BitWriter::endian(&mut data, BigEndian);
                    let value: $t = $t::default();
                    value.sml_write_value(&mut writer, &header)?;
                    writer.flush()?;
                    assert_eq!(data, vec![0; header.[<$t _bits>]() as usize / 8]);
                    Ok(())
                }
            }
        };
    }

    write_test_sml_write_value!(bool);
    write_test_sml_write_value!(u8);
    write_test_sml_write_value!(u16);
    write_test_sml_write_value!(u32);
    write_test_sml_write_value!(u64);
    write_test_sml_write_value!(u128);
    write_test_sml_write_value!(i8);
    write_test_sml_write_value!(i16);
    write_test_sml_write_value!(i32);
    write_test_sml_write_value!(i64);
    write_test_sml_write_value!(i128);
}
