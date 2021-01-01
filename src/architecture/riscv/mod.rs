use crate::{TargetSequence, TargetSequenceTraits};

pub struct RiscV {
    // All we need to know about generic RiscV
}

impl RiscV {
    pub fn identify_target<'a>(self) -> Option<(TargetSequence, TargetSequenceTraits<'a>)> {
        todo!()
    }
}
