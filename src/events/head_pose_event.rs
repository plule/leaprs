use leap_sys::LEAP_HEAD_POSE_EVENT;

use crate::{LeapVector, Quaternion};

crate::leap_ref_struct!(HeadPoseEvent, LEAP_HEAD_POSE_EVENT);

impl<'a> HeadPoseEvent<'a> {
    #[doc = " The position and orientation of the user's head. Positional tracking must be enabled."]
    #[doc = " @since 4.1.0"]
    pub fn head_position(&self) -> LeapVector {
        self.handle.head_position.into()
    }

    pub fn head_orientation(&self) -> Quaternion {
        self.handle.head_orientation.into()
    }

    #[doc = " The linear and angular velocity of the user's head. Positional tracking must be enabled."]
    #[doc = " @since 4.1.0"]
    pub fn head_linear_velocity(&self) -> LeapVector {
        self.handle.head_linear_velocity.into()
    }

    pub fn head_angular_velocity(&self) -> LeapVector {
        self.handle.head_angular_velocity.into()
    }
}
