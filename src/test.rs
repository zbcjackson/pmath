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
            let mut formula = Formula::new(&mut self.random);
            while self.formulas.iter().any(|f| formula == *f) {
                formula = Formula::new(&mut self.random);
            }
            self.formulas.push(formula);
        }
    }
}