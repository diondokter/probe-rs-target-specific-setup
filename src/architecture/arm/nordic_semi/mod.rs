use crate::{TargetSequence, TargetSequenceTraits};

pub struct NordicSemi {
    // All we need to know about generic NordicSemi
}

impl NordicSemi {
    pub fn identify_target<'a>(&self) -> Option<(TargetSequence, TargetSequenceTraits<'a>)> {
        todo!()
    }
}
