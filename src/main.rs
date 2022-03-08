mod random;
mod formula;
mod test_printer;
mod test;

use crate::formula::Formula;
use crate::test_printer::TestPrinter;
use crate::random::Random;
use crate::test::Test;

fn main() {
    let mut tests: Vec<Test> = vec![];
    for _i in 0..3 {
        let mut test = Test::new();
        test.generate();
        tests.push(test);
    }
    TestPrinter::new(tests).print();
}


