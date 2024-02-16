pub mod header;
pub mod write;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test: i8 = 1;
        dbg!(test.leading_zeros());
        println!("{:08b}", 2u8);
    }
}
