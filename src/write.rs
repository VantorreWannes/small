use bitstream_io::BitWrite;
use std::io;

use crate::header::SmlHeader;

pub(crate) trait ToSmlStream {
	fn sml_header(&self) -> SmlHeader;
	fn sml_write<W: BitWrite>(&self, writer: &mut W) -> io::Result<()>;
}

impl ToSmlStream for SmlHeader {
	fn sml_header(&self) -> SmlHeader {
		*self
	}

	fn sml_write<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
		writer.build(self)?;
		Ok(())
	}
}
