use crate::at::Command;

use super::Identifier;

pub struct PullUpDownDirection {
    dio4_pull_up: bool,
    dio3_pull_up: bool,
    dio2_pull_up: bool,
    dio1_pull_up: bool,
    dio0_pull_up: bool,
    dio6_pull_up: bool,
    dio8_pull_up: bool,
    dio14_pull_up: bool,
    dio5_pull_up: bool,
    dio9_pull_up: bool,
    dio12_pull_up: bool,
    dio10_pull_up: bool,
    dio11_pull_up: bool,
    dio7_pull_up: bool,
    dio13_pull_up: bool,
    dio15_pull_up: bool, // not on through-hole
    dio16_pull_up: bool, // not on through-hole
    dio17_pull_up: bool, // not on through-hole
    dio18_pull_up: bool, // not on through-hole
    dio19_pull_up: bool, // not on through-hole
}

impl PullUpDownDirection {
    // TODO: is bit field math correct
    pub fn bitfield(&self) -> u32 {
        let mut val: u32 = 0;

        if self.dio4_pull_up {
            val |= 1 << 0;
        }

        if self.dio3_pull_up {
            val |= 1 << 1;
        }

        if self.dio2_pull_up {
            val |= 1 << 2;
        }

        if self.dio1_pull_up {
            val |= 1 << 3;
        }

        if self.dio0_pull_up {
            val |= 1 << 4;
        }

        if self.dio6_pull_up {
            val |= 1 << 5;
        }

        if self.dio8_pull_up {
            val |= 1 << 6;
        }

        if self.dio14_pull_up {
            val |= 1 << 7;
        }

        if self.dio5_pull_up {
            val |= 1 << 8;
        }

        if self.dio9_pull_up {
            val |= 1 << 9;
        }

        if self.dio12_pull_up {
            val |= 1 << 10;
        }

        if self.dio10_pull_up {
            val |= 1 << 11;
        }

        if self.dio11_pull_up {
            val |= 1 << 12;
        }

        if self.dio7_pull_up {
            val |= 1 << 13;
        }

        if self.dio13_pull_up {
            val |= 1 << 14;
        }

        if self.dio15_pull_up {
            val |= 1 << 15;
        }

        if self.dio16_pull_up {
            val |= 1 << 16;
        }

        if self.dio17_pull_up {
            val |= 1 << 17;
        }

        if self.dio18_pull_up {
            val |= 1 << 18;
        }

        if self.dio19_pull_up {
            val |= 1 << 19;
        }

        val
    }
}

impl super::Command for PullUpDownDirection {
    fn identifier(&self) -> Identifier {
        Identifier::PullUpDownDirection
    }
}

impl From<PullUpDownDirection> for Command<0> {
    fn from(cmd: PullUpDownDirection) -> Command<0> {
        Command{
            identifier: Identifier::PullUpDownDirection,
            payload: None,
            carriage_returns: 1,
        }
    }
}
