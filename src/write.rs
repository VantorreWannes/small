use crate::header::SmlHeaderBuilder;
use bitstream_io::BitWrite;
use paste::paste;
use std::io;

use crate::header::SmlHeader;

pub(crate) trait ToSmlStream {
    fn sml_header(&self) -> SmlHeader;
    fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()>;
}

macro_rules! impl_to_sml_stream {
    ($t:ty) => {
        impl ToSmlStream for $t {
            fn sml_header(&self) -> SmlHeader {
                let size = (Self::BITS as Self - self.leading_zeros() as Self) as u8;
                paste! {
                    SmlHeaderBuilder::new().[<with_ $t _bits>](size).build()
                }
            }

            fn sml_write<W: BitWrite>(&self, writer: &mut W, header: &SmlHeader) -> io::Result<()> {
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
