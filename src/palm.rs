use derive_deref::Deref;
use leap_sys::LEAP_PALM;

use crate::{LeapVectorRef, QuaternionRef};

#[derive(Deref, Clone, Copy)]
pub struct PalmRef<'a>(pub(crate) &'a LEAP_PALM);

impl<'a> PalmRef<'a> {
    pub fn position(&self) -> LeapVectorRef {
        LeapVectorRef(&self.position)
    }

    pub fn stabilized_position(&self) -> LeapVectorRef {
        LeapVectorRef(&self.stabilized_position)
    }

    pub fn velocity(&self) -> LeapVectorRef {
        LeapVectorRef(&self.velocity)
    }

    pub fn normal(&self) -> LeapVectorRef {
        LeapVectorRef(&self.normal)
    }

    pub fn orientation(&self) -> QuaternionRef {
        QuaternionRef(&self.orientation)
    }

    pub fn direction(&self) -> LeapVectorRef {
        LeapVectorRef(&self.direction)
    }
}
