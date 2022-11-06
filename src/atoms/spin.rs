extern crate rand;

use super::particle::Particle;
use getset::Getters;
use rand::Rng;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Spin {
    symbol: Option<String>,
    coefficient: i64,
    value: State,
}

#[derive(Debug)]
pub enum State {
    Up = 1,
    Down = -1,
}

impl Particle<Spin> for Spin {
    fn reverse(self) -> Spin {
        match self.value {
            State::Up => Spin {
                symbol: self.symbol,
                coefficient: self.coefficient,
                value: State::Down,
            },
            State::Down => Spin {
                symbol: self.symbol,
                coefficient: self.coefficient,
                value: State::Up,
            },
        }
    }

    fn init(self) -> Spin {
        let mut rng = rand::thread_rng();
        if let 1 = rng.gen_range(0..=1) {
            Spin {
                symbol: self.symbol,
                coefficient: self.coefficient,
                value: State::Down,
            }
        } else {
            Spin {
                symbol: self.symbol,
                coefficient: self.coefficient,
                value: State::Up,
            }
        }
    }

    fn value(self) -> i32 {
        self.value as i32
    }
}
