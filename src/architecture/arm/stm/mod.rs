use crate::{TargetSequence, TargetSequenceTraits};

pub mod f7x3;
pub mod l0x0;

#[derive(Debug)]
pub struct Stm {
    // All we need to know about generic Stm
}

impl Stm {
    pub fn identify_target<'a>(self) -> Option<(TargetSequence, TargetSequenceTraits<'a>)> {
        // Do stuff to figure out
        match 0 {
            0 => f7x3::F7x3 {}.identify_target().map(|(mut s, t)| {
                s.set_manufacturer(self);
                (s, t)
            }),
            1 => l0x0::L0x0 {}.identify_target().map(|(mut s, t)| {
                s.set_manufacturer(self);
                (s, t)
            }),
            _ => None,
        }
    }
}
