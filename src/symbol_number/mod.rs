pub mod base;
mod float;

#[cfg(test)]
mod tests {
    use crate::symbol_number::base::SymbolNumber;
    use crate::symbol_number::float::Float;

    #[test]
    fn test_float() {
        let v1 = Float {
            integer: 0,
            decimal: vec![1, 2, 3, 3, 4],
        };
        println!("{}", v1.to_string());
    }
}
