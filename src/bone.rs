use leap_sys::LEAP_BONE;

use crate::{LeapVector, Quaternion};

crate::leap_ref_struct!(Bone, LEAP_BONE);

impl<'a> Bone<'a> {
    pub fn prev_joint(&self) -> LeapVector {
        self.handle.prev_joint.into()
    }

    pub fn next_joint(&self) -> LeapVector {
        self.handle.next_joint.into()
    }

    pub fn width(&self) -> f32 {
        self.handle.width
    }

    pub fn rotation(&self) -> Quaternion {
        self.handle.rotation.into()
    }
}
