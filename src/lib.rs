pub mod module;
#[allow(dead_code, unused_variables, unused_imports)]
fn main() {
    let vector1 = module::vector::Vector::new(1.0, 2.0, 3.0);
    let vector2 = module::vector::Vector::new(4.0, 5.0, 6.0);

    println!("Vector 1: {:?}", vector1);
    println!("Vector 2: {:?}", vector2);

}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_vector() {
        let vector1 = module::vector::Vector::new(1.0, 2.0, 3.0);
        let vector2 = module::vector::Vector::new(4.0, 5.0, 6.0);
        let scale: f64 = 2.0;

        assert_eq!(vector1.clone() + vector2, module::vector::Vector::new(5.0, 7.0, 9.0));
        assert_eq!(vector1 * scale, module::vector::Vector::new(2.0, 4.0, 6.0));
        // assert_eq!(vector2 * scale, module::vector::Vector::new(8.0, 10.0, 12.0));
        // assert_eq!(vector1 - vector2, module::vector::Vector::new(-3.0, -3.0, -3.0));
    }
}