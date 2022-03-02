mod random;
mod formula;
mod formula_printer;

use crate::formula::Formula;
use crate::formula_printer::FormulaPrinter;
use crate::random::Random;

fn main() {
    let mut formulas: Vec<Formula> = Vec::new();
    let mut random = Random::new();
    for _i in 0..60 {
        let mut formula = Formula::new(&mut random);
        while formulas.iter().any(|f| formula == *f) {
            formula = Formula::new(&mut random);
        }
        formulas.push(formula);
    }
    FormulaPrinter::new(formulas).print();
}


