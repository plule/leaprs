use leap_sys::LEAP_PALM;

use crate::{LeapVector, Quaternion};

crate::leap_ref_struct!(Palm, LEAP_PALM);

impl<'a> Palm<'a> {
    pub fn position(&self) -> LeapVector {
        self.handle.position.into()
    }

    pub fn stabilized_position(&self) -> LeapVector {
        self.handle.stabilized_position.into()
    }

    pub fn velocity(&self) -> LeapVector {
        self.handle.velocity.into()
    }

    pub fn normal(&self) -> LeapVector {
        self.handle.normal.into()
    }

    pub fn width(&self) -> f32 {
        self.handle.width
    }

    pub fn orientation(&self) -> Quaternion {
        self.handle.orientation.into()
    }

    pub fn direction(&self) -> LeapVector {
        self.handle.direction.into()
    }
}
