use crate::header::SmlHeader;

pub(crate) trait SmlWrite {
   fn sml_header(&self) -> SmlHeader;
}