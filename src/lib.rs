pub mod base;
pub mod file;

#[cfg(test)]
mod tests {
    use crate::base::{Color};

    #[test]
    fn it_works() {
        let c = Color::from_hex("FFDAFF");
        println!("{}", c.to_hex());
        println!("{}", c);
        println!("{:?}", c.to_tuple());
    }
}
