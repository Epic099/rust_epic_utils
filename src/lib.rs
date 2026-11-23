pub mod base;
pub mod file;
pub mod err;

#[cfg(test)]
mod tests {
    use crate::file::File;

    #[test]
    fn it_works() {
        let f = File::new("Cargo.toml", false).unwrap();
        println!("{}",f.read().unwrap());
    }
}
