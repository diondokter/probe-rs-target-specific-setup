use crate::{TargetSequence, TargetSequenceTraits};

pub mod nordic_semi;
pub mod stm;

#[derive(Debug)]
pub struct Arm {
    // All we need to know about generic Arm
}

impl Arm {
    pub fn identify_target<'a>(self) -> Option<(TargetSequence, TargetSequenceTraits<'a>)> {
        // Do stuff to figure out
        match 0 {
            0 => stm::Stm {}.identify_target().map(|(mut s, t)| {
                s.set_architecture(self);
                (s, t)
            }),
            1 => nordic_semi::NordicSemi {}.identify_target().map(|(mut s, t)| {
                s.set_architecture(self);
                (s, t)
            }),
            _ => None,
        }
    }
}
