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
