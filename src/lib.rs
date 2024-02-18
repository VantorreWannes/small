pub mod write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test: u8 = 64;
        dbg!(test.leading_zeros());
        let value = 64;
        println!("{:08b}", test | (value >> (8 - test.trailing_zeros())));
        println!("{}", test.trailing_zeros());
    }
}
