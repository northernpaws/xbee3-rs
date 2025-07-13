use crate::at::Command;

use super::Identifier;

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.g. 193)
pub struct PullUpDownResistorEnable {
    dio4: bool,
    dio3: bool,
    dio2: bool,
    dio1: bool,
    dio0: bool,
    dio6: bool,
    dio8: bool,
    dio14: bool,
    dio5: bool,
    dio9: bool,
    dio12: bool,
    dio10: bool,
    dio11: bool,
    dio7: bool,
    dio13: bool,
    dio15: bool, // not on through-hole
    dio16: bool, // not on through-hole
    dio17: bool, // not on through-hole
    dio18: bool, // not on through-hole
    dio19: bool, // not on through-hole
}

impl PullUpDownResistorEnable {
    // TODO: is bit field math correct
    pub fn bitfield(&self) -> u32 {
        let mut val: u32 = 0;

        if self.dio4 == true {
            val |= 1 << 0;
        }

        if self.dio3 == true {
            val |= 1 << 1;
        }

        if self.dio2 == true {
            val |= 1 << 2;
        }

        if self.dio1 == true {
            val |= 1 << 3;
        }

        if self.dio0 == true {
            val |= 1 << 4;
        }

        if self.dio6 == true {
            val |= 1 << 5;
        }

        if self.dio8 == true {
            val |= 1 << 6;
        }

        if self.dio14 == true {
            val |= 1 << 7;
        }

        if self.dio5 == true {
            val |= 1 << 8;
        }

        if self.dio9 == true {
            val |= 1 << 9;
        }

        if self.dio12 == true {
            val |= 1 << 10;
        }

        if self.dio10 == true {
            val |= 1 << 11;
        }

        if self.dio11 == true {
            val |= 1 << 12;
        }

        if self.dio7 == true {
            val |= 1 << 13;
        }

        if self.dio13 == true {
            val |= 1 << 14;
        }

        if self.dio15 == true {
            val |= 1 << 15;
        }

        if self.dio16 == true {
            val |= 1 << 16;
        }

        if self.dio17 == true {
            val |= 1 << 17;
        }

        if self.dio18 == true {
            val |= 1 << 18;
        }

        if self.dio19 == true {
            val |= 1 << 19;
        }

        val
    }
}

impl super::Command for PullUpDownResistorEnable {
    fn identifier(&self) -> Identifier {
        Identifier::PullUpDownResistorEnable
    }
}

impl From<PullUpDownResistorEnable> for Command<0> {
    fn from(cmd: PullUpDownResistorEnable) -> Command<0> {
        Command{
            identifier: Identifier::PullUpDownResistorEnable,
            payload: None,
            carriage_returns: 1,
        }
    }
}
