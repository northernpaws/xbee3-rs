use crate::at::Command;

use super::Identifier;

pub struct DIOChangeDetect {
    dio0: bool, // Micro pin 31/SMT pin 33/TH pin 20
    dio1: bool, // Micro pin 30/SMT pin 32/TH pin 19
    dio2: bool, // Micro pin 29/SMT pin 31/TH pin 18
    dio3: bool, // Micro pin 28/SMT pin 30/TH pin 17
    dio4: bool, // Micro pin 23/SMT pin 24/TH pin 11
    dio5: bool, // Micro pin 26/SMT pin 28/TH pin 15
    dio6: bool, // Micro pin 27/SMT pin 29/TH pin 16
    dio7: bool, // Micro pin 24/SMT pin 25/TH pin 12
    dio8: bool, // Micro pin 9/SMT pin 10/TH pin 9
    dio9: bool, // Micro pin 25/SMT pin 26/TH pin 13
    dio10: bool, // Micro pin 7/SMT pin 7/TH pin 6
    dio11: bool, // Micro pin 8/SMT pin 8/TH pin 7
    dio12: bool, // Micro pin 5/SMT pin 5/TH pin 4
    dio13: bool, // Micro pin 3/SMT pin 3/TH pin 2
    dio14: bool, // Micro pin 4/SMT pin 4/TH pin 3
}

impl DIOChangeDetect {
    pub fn bitfield(&self) -> u16 {
        let mut val: u16 = 0;

        if self.dio0 {
            val |= 1 << 0;
        }

        if self.dio1 {
            val |= 1 << 1;
        }

        if self.dio2 {
            val |= 1 << 2;
        }

        if self.dio3 {
            val |= 1 << 3;
        }

        if self.dio4 {
            val |= 1 << 4;
        }

        if self.dio5 {
            val |= 1 << 5;
        }

        if self.dio6 {
            val |= 1 << 6;
        }

        if self.dio7 {
            val |= 1 << 7;
        }

        if self.dio8 {
            val |= 1 << 8;
        }

        if self.dio9 {
            val |= 1 << 9;
        }

        if self.dio10 {
            val |= 1 << 10;
        }

        if self.dio11 {
            val |= 1 << 11;
        }

        if self.dio12 {
            val |= 1 << 12;
        }

        if self.dio13 {
            val |= 1 << 13;
        }

        if self.dio14 {
            val |= 1 << 14;
        }

        val
    }
}

impl super::Command for DIOChangeDetect {
    fn identifier(&self) -> Identifier {
        Identifier::DIOChangeDetect
    }
}

impl From<DIOChangeDetect> for Command<0> {
    fn from(cmd: DIOChangeDetect) -> Command<0> {
        Command{
            identifier: Identifier::DIOChangeDetect,
            payload: None,
            carriage_returns: 1,
        }
    }
}
