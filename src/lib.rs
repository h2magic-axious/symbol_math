mod utils;
mod frac;


#[cfg(test)]
mod tests {
    use crate::frac::Frac;
    use crate::utils::{Integer, SymbolNumber};

    #[test]
    fn it_works() {
        let f = Frac {
            numerator: Box::new(Integer { value: 12 }),
            denominator: Box::new(Integer { value: 13 }),
        };
        f.display()
    }
}
