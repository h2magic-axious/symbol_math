use std::cmp::max;
use std::ops::Add;
use std::slice::range;
use crate::symbol_number::base::{Double, Integer, SymbolNumber};
use crate::utils::merge_vector;

/*
For Example:

Float{
    integer: 10,
    decimal: [1,2,3,4]
}
= 10.1234
 */
pub struct Float {
    pub integer: Integer,
    pub decimal: Vec<Integer>,
}

impl SymbolNumber for Float {
    fn result(&self) -> Double {
        let str_decimal: String = self.decimal.iter().map(|v| v.to_string()).collect::<String>();
        let res = format!("{}.{}", self.integer, str_decimal);
        res.parse::<Double>().unwrap()
    }

    fn to_string(&self) -> String {
        format!("{}", self.result())
    }
}

impl Add for Float {
    type Output = Float;

    fn add(self, rhs: Self) -> Self::Output {
        let decimals = merge_vector(&self.decimal, &rhs.decimal);
        let mut result = Vec::new();

        let mut c = 0;
        for el in decimals.iter().rev() {
            let temp = c + el;
            result.push(temp % 10);
            c = temp / 10;
        }

        if c > 0 {
            result.insert(0, c);
        }

        Float {
            integer: self.integer + rhs.integer,
            decimal: result,
        }
    }
}