use derive_deref::Deref;
use leap_sys::LEAP_BONE;

use crate::{LeapVectorRef, QuaternionRef};

#[doc = " Describes a bone's position and orientation."]
#[doc = ""]
#[doc = " Bones are members of the LEAP_DIGIT struct."]
#[doc = " @since 3.0.0"]
#[derive(Deref, Clone, Copy)]
pub struct BoneRef<'a>(pub(crate) &'a LEAP_BONE);

impl<'a> BoneRef<'a> {
    #[doc = " The base of the bone, closer to the heart. The bones origin. @since 3.0.0"]
    pub fn prev_joint(&self) -> LeapVectorRef {
        LeapVectorRef::from(&self.prev_joint)
    }

    #[doc = " The end of the bone, further from the heart. @since 3.0.0"]
    pub fn next_joint(&self) -> LeapVectorRef {
        LeapVectorRef::from(&self.next_joint)
    }

    #[doc = " Rotation in world space from the forward direction."]
    #[doc = " Convert the quaternion to a matrix to derive the basis vectors."]
    #[doc = " @since 3.1.2"]
    pub fn rotation(&self) -> QuaternionRef {
        QuaternionRef::from(&self.rotation)
    }
}
