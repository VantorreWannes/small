pub mod write;


pub(crate) const TYPE_IDENT_BIT_SIZE: u8 = 3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test: u8 = 8;
        println!("{:08b}", test);
        println!("{}", (test.leading_zeros()-1)/4);
    }
}
