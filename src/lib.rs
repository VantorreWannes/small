use derive_sml::make_answer;

pub mod write;
pub(crate) const TYPE_IDENT_BIT_SIZE: u8 = 3;

make_answer!();

#[cfg(test)]
mod tests {
    use std::io;

    use bitstream_io::{BigEndian, BitWrite, BitWriter};

    use self::write::WriteSml;

    use super::*;

    #[test]
    fn it_works() {
        let test: i16 = 16;
        println!("{:08b}", test);
        println!("{}", test.abs());
        println!("{}", ((u16::BITS - test.leading_zeros())+3)/4);
    }

    #[test]
    fn test_compactness() -> io::Result<()> {
        let text = "Hello my name is navoko";
        let mut data: Vec<u8> = vec![];
        let mut writer = BitWriter::endian(&mut data, BigEndian);
        text.sml_write(&mut writer)?;
        writer.byte_align()?;
        dbg!(text.len());
        dbg!(&data, data.len());
        Ok(())
    }

    #[test]
    fn test_struct_derive() -> io::Result<()> {
        assert_eq!(42, answer());
        Ok(())
    }
}
