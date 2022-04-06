use crate::{Formula, Random};

pub struct Test {
    pub(crate) formulas: Vec<Formula>,
    random: Random,
}

pub const FORMULA_NUMBER_IN_ONE_PAGE: i32 = 81;
impl Test {
    pub(crate) fn new() -> Test {
        Test{formulas: Vec::new(), random: Random::new()}
    }
    pub fn generate(&mut self) {
        for _i in 0..FORMULA_NUMBER_IN_ONE_PAGE {
            let mut formula = self.generate_formula();
            while self.formulas.iter().any(|f| formula == *f) {
                formula = self.generate_formula();
            }
            self.formulas.push(formula);
        }
    }

    fn generate_formula(&mut self) -> Formula {
        let left = self.random.two_digit_number();
        let right = self.random.one_digit_number();
        let product = left * right;
        let blank = self.random.blank_position();
        let formula = Formula {
            left: if blank == 0 {None} else {Some(left)},
            right: if blank == 1 {None} else {Some(right)},
            product: if blank == 2 {None} else {Some(product)},
        };
        formula
    }
}