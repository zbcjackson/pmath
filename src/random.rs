use rand::prelude::ThreadRng;
use rand::Rng;

pub struct Random {
    rng: ThreadRng,
}

impl Random {
    pub(crate) fn new() -> Random {
        Random {rng: rand::thread_rng()}
    }

    pub(crate) fn one_digit_number(&mut self) -> i32 {
        2 + self.random(8)
    }

    pub(crate) fn two_digit_number(&mut self) -> i32 {
        10 + self.random( 90)
    }

    pub(crate) fn blank_position(&mut self) -> i32 {
        self.random(3)
    }

    fn random(&mut self, number: i32) -> i32 {
        (self.rng.gen::<f32>() * number as f32).floor() as i32
    }
}
