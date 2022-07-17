use leap_sys::LEAP_BONE;

use crate::{LeapVector, Quaternion};

crate::leap_ref_struct!(
    #[doc = " Describes a bone's position and orientation."]
    #[doc = ""]
    #[doc = " Bones are members of the LEAP_DIGIT struct."]
    #[doc = " @since 3.0.0"]
    Bone,
    LEAP_BONE
);

impl<'a> Bone<'a> {
    #[doc = " The base of the bone, closer to the heart. The bones origin. @since 3.0.0"]
    pub fn prev_joint(&self) -> LeapVector {
        self.handle.prev_joint.into()
    }

    #[doc = " The end of the bone, further from the heart. @since 3.0.0"]
    pub fn next_joint(&self) -> LeapVector {
        self.handle.next_joint.into()
    }

    #[doc = " The average width of the flesh around the bone in millimeters. @since 3.0.0"]
    pub fn width(&self) -> f32 {
        self.handle.width
    }

    #[doc = " Rotation in world space from the forward direction."]
    #[doc = " Convert the quaternion to a matrix to derive the basis vectors."]
    #[doc = " @since 3.1.2"]
    pub fn rotation(&self) -> Quaternion {
        self.handle.rotation.into()
    }
}
