use crate::random::Random;
use std::cmp;

pub struct Formula {
    pub(crate) left: i32,
    pub(crate) right: i32,
}

impl Formula {
    pub(crate) fn new(random: &mut Random) -> Formula {
        Formula {
            left: random.two_digit_number(),
            right: random.one_digit_number()
        }
    }
}

impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl cmp::Eq for Formula  {
    
}