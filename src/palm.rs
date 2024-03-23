use derive_deref::Deref;
use leap_sys::LEAP_PALM;

use crate::{LeapVector, Quaternion};

#[derive(Deref)]
pub struct Palm<'a>(pub(crate) &'a LEAP_PALM);

impl<'a> Palm<'a> {
    pub fn position(&self) -> LeapVector {
        LeapVector(self.position)
    }

    pub fn stabilized_position(&self) -> LeapVector {
        LeapVector(self.stabilized_position)
    }

    pub fn velocity(&self) -> LeapVector {
        LeapVector(self.velocity)
    }

    pub fn normal(&self) -> LeapVector {
        LeapVector(self.normal)
    }

    pub fn orientation(&self) -> Quaternion {
        Quaternion(self.orientation)
    }

    pub fn direction(&self) -> LeapVector {
        LeapVector(self.direction)
    }
}
