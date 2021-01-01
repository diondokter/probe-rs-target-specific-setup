use crate::{TargetSequence, TargetSequenceTraits, architecture::arm::Arm};
use super::Stm;

pub mod f743;
pub mod f753;

#[derive(Debug)]
pub struct F7x3 {
    // All we need to know about generic F7x3
}

impl F7x3 {
    pub fn identify_target<'a>(self) -> Option<(TargetSequence, TargetSequenceTraits<'a>)> {
        // Do stuff to figure out
        match 0 {
            0 => {
                let mut sequence = TargetSequence::default();
                sequence.set_target(f743::F743 {});
                sequence.set_family(self);

                let traits = TargetSequenceTraits {
                    debug: |sequence| Some(Box::new(sequence.get_as_generic::<Arm, Stm, F7x3, f743::F743>().unwrap())),           
                };

                Some((sequence, traits))
            }
            1 => {
                let mut sequence = TargetSequence::default();
                sequence.set_target(f753::F753 {});
                sequence.set_family(self);

                let traits = TargetSequenceTraits {
                    ..Default::default()           
                };

                Some((sequence, traits))
            }
            _ => None,
        }
    }
}
