use std::{any::Any, fmt::Debug};

pub mod architecture;

/// The struct that stores all the probe info
pub struct TargetSequence {
    architecture: Box<dyn Any>,
    manufacturer: Box<dyn Any>,
    family: Box<dyn Any>,
    target: Box<dyn Any>,
}

impl TargetSequence {
    /// Walk over the options to identify the target
    pub fn identify<'a>() -> Option<(Self, TargetSequenceTraits<'a>)> {
        architecture::identify_target()
    }

    pub fn set_architecture<A: 'static>(&mut self, architecture: A) {
        self.architecture = Box::new(architecture);
    }
    pub fn set_manufacturer<M: 'static>(&mut self, manufacturer: M) {
        self.manufacturer = Box::new(manufacturer);
    }
    pub fn set_family<F: 'static>(&mut self, family: F) {
        self.family = Box::new(family);
    }
    pub fn set_target<T: 'static>(&mut self, target: T) {
        self.target = Box::new(target);
    }

    /// Borrow the loose components
    fn get_components(&mut self) -> (&mut dyn Any, &mut dyn Any, &mut dyn Any, &mut dyn Any) {
        use std::ops::DerefMut;
        (
            self.architecture.deref_mut(),
            self.manufacturer.deref_mut(),
            self.family.deref_mut(),
            self.target.deref_mut(),
        )
    }

    /// Try to create a temporary statically generic instance.
    /// This generic type then implements all the different chip implementations.
    pub fn get_as_generic<'a, A: 'static, M: 'static, F: 'static, T: 'static>(
        &'a mut self,
    ) -> Option<GenericTargetSequence<'a, A, M, F, T>> {
        let (architecture, manufacturer, family, target) = self.get_components();

        let architecture = architecture.downcast_mut::<A>()?;
        let manufacturer = manufacturer.downcast_mut::<M>()?;
        let family = family.downcast_mut::<F>()?;
        let target = target.downcast_mut::<T>()?;

        Some(GenericTargetSequence {
            architecture,
            manufacturer,
            family,
            target,
        })
    }
}

impl Default for TargetSequence {
    fn default() -> Self {
        TargetSequence {
            architecture: Box::new(()),
            manufacturer: Box::new(()),
            family: Box::new(()),
            target: Box::new(()),
        }
    }
}

/// The target sequence that contains concrete types.
pub struct GenericTargetSequence<'a, A, M, F, T> {
    pub architecture: &'a mut A,
    pub manufacturer: &'a mut M,
    pub family: &'a mut F,
    pub target: &'a mut T,
}

/// Example implementation.
/// If everything is debug, then we are as well.
/// This could be used 
impl<'a, A: Debug, M: Debug, F: Debug, T: Debug> Debug for GenericTargetSequence<'a, A, M, F, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericTargetSequence")
            .field("architecture", self.architecture)
            .field("manufacturer", self.manufacturer)
            .field("family", self.family)
            .field("target", self.target)
            .finish()
    }
}

/// All the traits we care about.
/// Every target now needs to return this.
/// The actual generic type parameters are store in code.
pub struct TargetSequenceTraits<'a> {
    pub debug: fn(&'a mut TargetSequence) -> Option<Box<(dyn Debug + 'a)>>,
}

impl<'a> Default for TargetSequenceTraits<'a> {
    fn default() -> Self {
        Self {
            debug: |_| None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {
        // Identify the current target
        let (mut sequence, traits) = TargetSequence::identify().unwrap();

        // We can cast it to the specific variant
        let generic_sequence = sequence.get_as_generic::<architecture::arm::Arm, architecture::arm::stm::Stm, architecture::arm::stm::f7x3::F7x3, architecture::arm::stm::f7x3::f743::F743>();
        assert!(generic_sequence.is_some());
        // We can't cast it to the wrong types
        let bad_generic_sequence = sequence.get_as_generic::<architecture::arm::Arm, architecture::arm::stm::Stm, architecture::arm::stm::f7x3::F7x3, architecture::arm::stm::f7x3::f753::F753>();
        assert!(bad_generic_sequence.is_none());

        // Normally we don't know what the type parameters are because of the dynamic nature.
        // We're protected against using the wrong types, but that doesn't make us know the correct ones.
        // So if we want to access our traits, we need to use the traits type where the typeparameters are abstracted away in the function pointers.
        // I've made the F743 debug and the F753 not. (Try switching what is returned in the F7x3 module to see this test fail by changing the match input to 1)
        let sequence_debug = (traits.debug)(&mut sequence).unwrap();
        // Now use it
        println!("{:?}", sequence_debug);
    }
}
