use std::cmp;

pub struct Formula {
    pub(crate) left: Option<i32>,
    pub(crate) right: Option<i32>,
    pub(crate) product: Option<i32>,
}

impl Formula {
}

impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl cmp::Eq for Formula  {
    
}