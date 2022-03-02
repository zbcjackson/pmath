mod random;
mod formula;
mod test_printer;
mod test;

use crate::formula::Formula;
use crate::test_printer::TestPrinter;
use crate::random::Random;
use crate::test::Test;

fn main() {
    let mut test = Test::new();
    test.generate();
    TestPrinter::new(test).print();
}


