use derive_deref::Deref;
use leap_sys::LEAP_EYE_EVENT;

use crate::LeapVector;

#[derive(Deref)]
pub struct EyeEvent<'a>(pub(crate) &'a LEAP_EYE_EVENT);

impl<'a> EyeEvent<'a> {
    #[doc = " The position of the user's left eye."]
    #[doc = " @since 4.1.0"]
    pub fn left_eye_position(&self) -> LeapVector {
        LeapVector(self.left_eye_position)
    }

    #[doc = " The position of the user's right eye."]
    #[doc = " @since 4.1.0"]
    pub fn right_eye_position(&self) -> LeapVector {
        LeapVector(self.right_eye_position)
    }
}
