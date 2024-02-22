pub mod write;
pub(crate) const TYPE_IDENT_BIT_SIZE: u8 = 3;


#[cfg(test)]
mod tests {
    use std::io;

    use bitstream_io::{BigEndian, BitWrite, BitWriter};
    use derive_sml::WriteSml;

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

    #[derive(WriteSml, Debug, Default)]
    pub struct TestStruct {
        a: u8,
        b: u8,
        c: bool,
    }

    #[test]
    fn test_derive() -> io::Result<()> {
        let text = TestStruct::default();
        let mut data: Vec<u8> = vec![];
        let mut writer = BitWriter::endian(&mut data, BigEndian);
        text.sml_write(&mut writer)?;
        writer.byte_align()?;
        dbg!(&data, data.len());
        Ok(())
    }
}
