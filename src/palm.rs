use derive_deref::Deref;
use leap_sys::LEAP_PALM;

use crate::{LeapVector, Quaternion};

#[derive(Deref)]
pub struct Palm<'a>(pub(crate) &'a LEAP_PALM);

impl<'a> Palm<'a> {
    pub fn position(&self) -> LeapVector {
        LeapVector::from(&self.position)
    }

    pub fn stabilized_position(&self) -> LeapVector {
        LeapVector::from(&self.stabilized_position)
    }

    pub fn velocity(&self) -> LeapVector {
        LeapVector::from(&self.velocity)
    }

    pub fn normal(&self) -> LeapVector {
        LeapVector::from(&self.normal)
    }

    pub fn orientation(&self) -> Quaternion {
        Quaternion::from(&self.orientation)
    }

    pub fn direction(&self) -> LeapVector {
        LeapVector::from(&self.direction)
    }
}
