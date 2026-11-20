pub mod base;
pub mod file;

#[cfg(test)]
mod tests {
    use crate::file::File;

    #[test]
    fn it_works() {
        let f = File::new("Cargo.toml", false);
        println!("{}",f.read())
    }
}
