pub mod write;


pub(crate) const TYPE_BIT_SIZE: u8 = 3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test: u8 = 64;
        dbg!(test.leading_zeros());
        let value = 64;
        println!("{:08b}", b'a');
        println!("{}", test.trailing_zeros());
    }
}
