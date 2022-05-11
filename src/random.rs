use rand::prelude::ThreadRng;
use rand::Rng;

pub struct Random {
    rng: ThreadRng,
}

impl Random {
}

impl Random {
    pub(crate) fn new() -> Random {
        Random {rng: rand::thread_rng()}
    }

    pub(crate) fn one_digit_number(&mut self) -> f32 {
        2.0 + self.random(8)
    }

    pub(crate) fn two_digit_number(&mut self) -> f32 {
        10.0 + self.random( 90)
    }

    pub(crate) fn float_number(&mut self) -> f32 {
        self.random(10000) / 100.0
    }

    pub(crate) fn blank_position(&mut self) -> i32 {
        self.random(3) as i32
    }

    pub(crate) fn half_half(&mut self) -> i32 {
        self.random(2) as i32
    }

    fn random(&mut self, number: i32) -> f32 {
        (self.rng.gen::<f32>() * number as f32).floor()
    }
}

#[cfg(test)]
mod tests {
    use crate::Random;

    #[test]
    fn one_digit_number() {
        let mut random = Random::new();
        for _ in 0..1000 {
            let n = random.one_digit_number();
            assert_eq!(n.fract(), 0.0);
            assert!(n >= 2.0, "n = {}", n);
        }
    }

    #[test]
    fn two_digit_number() {
        let mut random = Random::new();
        for _ in 0..1000 {
            let n = random.two_digit_number();
            assert_eq!(n.fract(), 0.0);
            assert!(n >= 10.0, "n = {}", n);
        }
    }
}