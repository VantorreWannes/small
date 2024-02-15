use bitstream_io::ToBitStream;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SmlHeader {
    bool: u8,
    char: u8,
    i8: u8,
    i16: u8,
    i32: u8,
    i64: u8,
    i128: u8,
    u8: u8,
    u16: u8,
    u32: u8,
    u64: u8,
    u128: u8,
    f32: u8,
    f64: u8,
}

impl SmlHeader {
    pub fn new(
        char: u8,
        i8: u8,
        i16: u8,
        i32: u8,
        i64: u8,
        i128: u8,
        u8: u8,
        u16: u8,
        u32: u8,
        u64: u8,
        u128: u8,
        f32: u8,
        f64: u8,
    ) -> Self {
        Self {
            bool: 1,
            char,
            i8,
            i16,
            i32,
            i64,
            i128,
            u8,
            u16,
            u32,
            u64,
            u128,
            f32,
            f64,
        }
    }
}

impl ToBitStream for SmlHeader {
    type Error = std::io::Error;

    fn to_writer<W: bitstream_io::BitWrite + ?Sized>(
        &self,
        writer: &mut W,
    ) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        const BITS: u32 = u8::BITS - 1;
        writer.write(BITS, self.bool)?;
        writer.write(BITS, self.char)?;
        writer.write(BITS, self.i8)?;
        writer.write(BITS, self.i16)?;
        writer.write(BITS, self.i32)?;
        writer.write(BITS, self.i64)?;
        writer.write(BITS, self.i128)?;
        writer.write(BITS, self.u8)?;
        writer.write(BITS, self.u16)?;
        writer.write(BITS, self.u32)?;
        writer.write(BITS, self.u64)?;
        writer.write(BITS, self.u128)?;
        writer.write(BITS, self.f32)?;
        writer.write(BITS, self.f64)?;
        Ok(())
    }
}
