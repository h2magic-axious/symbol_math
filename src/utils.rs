pub trait SymbolNumber {
    fn result(&self) -> f64;
    fn display(&self);
    fn to_string(&self) -> String;
}

pub struct Integer {
    pub value: i32,
}

impl SymbolNumber for Integer {
    fn result(&self) -> f64 {
        self.value as f64
    }

    fn display(&self) {
        println!("{}", self.value)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

pub struct Float {
    pub value: f64,
}

impl SymbolNumber for Float {
    fn result(&self) -> f64 {
        self.value
    }

    fn display(&self) {
        println!("{}", self.value)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}