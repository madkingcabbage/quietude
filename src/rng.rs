use std::num::Wrapping;

use serde::{Deserialize, Serialize};

use crate::utils::lfsr;

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct TickBasedRng {
    num: Option<u32>,
    tick: u32,
    seed: u32,
}

impl TickBasedRng {
    pub fn rand(&mut self) -> u32 {
        if let Some(num) = self.num {
            self.num = Some(lfsr(num));
        } else {
            let mut lfsr_input = Wrapping(self.seed) + Wrapping(self.tick);
            if lfsr_input.0 == 0 || lfsr_input.0 == 0xffffffff {
                lfsr_input -= 2;
            }
            self.num = Some(lfsr(lfsr_input.0));
        }
        self.num.unwrap()
    }

    pub fn reset(&mut self, tick: u32) {
        self.num = None;
        self.tick = tick;
    }

    pub fn new(seed: u32, tick: u32) -> Self {
        Self {
            num: None,
            tick,
            seed,
        }
    }
}
