use derive_deref::Deref;
use leap_sys::LEAP_HEAD_POSE_EVENT;

use crate::{LeapVector, Quaternion};

#[derive(Deref)]
pub struct HeadPoseEvent<'a>(pub(crate) &'a LEAP_HEAD_POSE_EVENT);

impl<'a> HeadPoseEvent<'a> {
    #[doc = " The position and orientation of the user's head. Positional tracking must be enabled."]
    #[doc = " @since 4.1.0"]
    pub fn head_position(&self) -> LeapVector {
        LeapVector::from(&self.head_position)
    }

    pub fn head_orientation(&self) -> Quaternion {
        Quaternion::from(&self.head_orientation)
    }

    #[doc = " The linear and angular velocity of the user's head. Positional tracking must be enabled."]
    #[doc = " @since 4.1.0"]
    pub fn head_linear_velocity(&self) -> LeapVector {
        LeapVector::from(&self.head_linear_velocity)
    }

    pub fn head_angular_velocity(&self) -> LeapVector {
        LeapVector::from(&self.head_angular_velocity)
    }
}
