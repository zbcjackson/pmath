mod random;
mod formula;
mod formula_printer;
mod test;

use crate::formula::Formula;
use crate::formula_printer::FormulaPrinter;
use crate::random::Random;
use crate::test::Test;

fn main() {
    let mut test = Test::new();
    test.generate();
    FormulaPrinter::new(test.formulas).print();
}


