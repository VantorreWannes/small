use bitstream_io::{FromBitStream, ToBitStream};

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
    const BITS: u32 = u8::BITS - 1;
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
        writer.write(Self::BITS, self.bool)?;
        writer.write(Self::BITS, self.char)?;
        writer.write(Self::BITS, self.i8)?;
        writer.write(Self::BITS, self.i16)?;
        writer.write(Self::BITS, self.i32)?;
        writer.write(Self::BITS, self.i64)?;
        writer.write(Self::BITS, self.i128)?;
        writer.write(Self::BITS, self.u8)?;
        writer.write(Self::BITS, self.u16)?;
        writer.write(Self::BITS, self.u32)?;
        writer.write(Self::BITS, self.u64)?;
        writer.write(Self::BITS, self.u128)?;
        writer.write(Self::BITS, self.f32)?;
        writer.write(Self::BITS, self.f64)?;
        Ok(())
    }
}

impl FromBitStream for SmlHeader {
    type Error = std::io::Error;

    fn from_reader<R: bitstream_io::BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized {
        Ok(Self {
            bool: r.read(Self::BITS)?,
            char: r.read(Self::BITS)?,
            i8: r.read(Self::BITS)?,
            i16: r.read(Self::BITS)?,
            i32: r.read(Self::BITS)?,
            i64: r.read(Self::BITS)?,
            i128: r.read(Self::BITS)?,
            u8: r.read(Self::BITS)?,
            u16: r.read(Self::BITS)?,
            u32: r.read(Self::BITS)?,
            u64: r.read(Self::BITS)?,
            u128: r.read(Self::BITS)?,
            f32: r.read(Self::BITS)?,
            f64: r.read(Self::BITS)?,
        })
    }
}