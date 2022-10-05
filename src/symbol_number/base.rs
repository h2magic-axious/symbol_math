pub type Integer = isize;
pub type Double = f64;

pub trait SymbolNumber {
    fn result(&self) -> Double;
    fn to_string(&self) -> String;
}
