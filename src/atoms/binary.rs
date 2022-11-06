extern crate rand;

use super::particle::Particle;
use getset::Getters;
use rand::Rng;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Binary {
    symbol: Option<String>,
    coefficient: i64,
    value: Bit,
}

#[derive(Debug)]
pub enum Bit {
    One = 1,
    Zero = 0,
}

impl Particle<Binary> for Binary {
    fn reverse(self) -> Binary {
        match self.value {
            Bit::One => Binary {
                symbol: self.symbol,
                coefficient: self.coefficient,
                value: Bit::Zero,
            },
            Bit::Zero => Binary {
                symbol: self.symbol,
                coefficient: self.coefficient,
                value: Bit::One,
            },
        }
    }

    fn init(self) -> Binary {
        let mut rng = rand::thread_rng();
        if let 1 = rng.gen_range(0..=1) {
            Binary {
                symbol: self.symbol,
                coefficient: self.coefficient,
                value: Bit::One,
            }
        } else {
            Binary {
                symbol: self.symbol,
                coefficient: self.coefficient,
                value: Bit::Zero,
            }
        }
    }

    fn value(self) -> i32 {
        self.value as i32
    }
}
