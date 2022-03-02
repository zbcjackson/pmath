use std::cmp;

pub struct Formula {
    pub(crate) left: i32,
    pub(crate) right: i32,
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