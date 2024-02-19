pub mod write;


pub(crate) const TYPE_IDENT_BIT_SIZE: u8 = 3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test: i16 = 16;
        println!("{:08b}", test);
        println!("{}", test.abs());
        println!("{}", ((u16::BITS - test.leading_zeros())+3)/4);
    }
}
