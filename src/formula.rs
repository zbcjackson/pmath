use std::cmp;

#[derive(PartialEq, Copy, Clone)]
pub enum Operator {
    Add,
    Multiple,
}

pub struct Formula {
    pub(crate) left: Option<f32>,
    pub(crate) operator: Operator,
    pub(crate) right: Option<f32>,
    pub(crate) result: Option<f32>,
}

impl Formula {
}

impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right && self.operator == other.operator
    }
}

impl cmp::Eq for Formula  {
    
}