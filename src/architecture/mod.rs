use crate::{TargetSequence, TargetSequenceTraits};

pub mod arm;
pub mod riscv;

pub fn identify_target<'a>() -> Option<(TargetSequence, TargetSequenceTraits<'a>)> {
    // Do stuff to figure out
    match 0 {
        0 => arm::Arm {}.identify_target(),
        1 => riscv::RiscV {}.identify_target(),
        _ => None,
    }
}

