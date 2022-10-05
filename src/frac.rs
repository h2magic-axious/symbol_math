// use crate::symbol_number::base::SymbolNumber;
//
//
// pub struct Frac<> {
//     pub numerator: Box<dyn SymbolNumber>,
//     pub denominator: Box<dyn SymbolNumber>,
// }
//
//
// impl SymbolNumber for Frac {
//     fn result(&self) -> f64 {
//         self.numerator.result() / self.denominator.result()
//     }
//
//
//     fn to_string(&self) -> String {
//         self.display();
//         format!("<Frac({}, {})>", self.numerator.to_string(), self.denominator.to_string())
//     }
// }
