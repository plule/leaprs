use derive_deref::Deref;
use leap_sys::LEAP_PALM;

use crate::{LeapVectorRef, QuaternionRef};

#[derive(Deref, Clone, Copy)]
pub struct PalmRef<'a>(pub(crate) &'a LEAP_PALM);

impl<'a> PalmRef<'a> {
    pub fn position(&self) -> LeapVectorRef {
        LeapVectorRef::from(&self.position)
    }

    pub fn stabilized_position(&self) -> LeapVectorRef {
        LeapVectorRef::from(&self.stabilized_position)
    }

    pub fn velocity(&self) -> LeapVectorRef {
        LeapVectorRef::from(&self.velocity)
    }

    pub fn normal(&self) -> LeapVectorRef {
        LeapVectorRef::from(&self.normal)
    }

    pub fn orientation(&self) -> QuaternionRef {
        QuaternionRef::from(&self.orientation)
    }

    pub fn direction(&self) -> LeapVectorRef {
        LeapVectorRef::from(&self.direction)
    }
}
