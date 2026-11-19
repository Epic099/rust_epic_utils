pub mod base;

#[cfg(test)]
mod tests {
    use crate::base::{Vector2, Vector2Int};

    

    #[test]
    fn it_works() {
        let x = Vector2Int::new(1, 1);
        println!("{}", x);
        println!("{:?}", x);
    }
}
