use crate::utils::SymbolNumber;


pub struct Frac<> {
    pub numerator: Box<dyn SymbolNumber>,
    pub denominator: Box<dyn SymbolNumber>,
}


impl SymbolNumber for Frac {
    fn result(&self) -> f64 {
        self.numerator.result() / self.denominator.result()
    }

    fn display(&self) {
        let str_numerator = self.numerator.to_string();
        let str_denominator = self.denominator.to_string();

        println!("{}", str_numerator);
        println!("{}", "-".repeat(str_denominator.len()));
        println!("{}", str_denominator);
    }

    fn to_string(&self) -> String {
        self.display();
        format!("<Frac({}, {})>", self.numerator.to_string(), self.denominator.to_string())
    }
}
