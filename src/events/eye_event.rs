use derive_deref::Deref;
use leap_sys::LEAP_EYE_EVENT;

use crate::LeapVectorRef;

/// # Fields
/// Available via dereference: [LEAP_EYE_EVENT].
#[derive(Deref, Clone, Copy)]
pub struct EyeEventRef<'a>(pub(crate) &'a LEAP_EYE_EVENT);

impl<'a> EyeEventRef<'a> {
    #[doc = " The position of the user's left eye."]
    #[doc = " @since 4.1.0"]
    pub fn left_eye_position(&self) -> LeapVectorRef {
        LeapVectorRef(&self.left_eye_position)
    }

    #[doc = " The position of the user's right eye."]
    #[doc = " @since 4.1.0"]
    pub fn right_eye_position(&self) -> LeapVectorRef {
        LeapVectorRef(&self.right_eye_position)
    }
}
