use crate::{Formula, Random};

pub struct Test {
    pub(crate) formulas: Vec<Formula>,
    random: Random,
}

impl Test {
    pub(crate) fn new() -> Test {
        Test{formulas: Vec::new(), random: Random::new()}
    }
    pub fn generate(&mut self) {
        for _i in 0..60 {
            let mut formula = self.generate_formula();
            while self.formulas.iter().any(|f| formula == *f) {
                formula = self.generate_formula();
            }
            self.formulas.push(formula);
        }
    }

    fn generate_formula(&mut self) -> Formula {
        Formula {
            left: self.random.two_digit_number(),
            right: self.random.one_digit_number()
        }
    }
}