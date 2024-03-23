use leap_sys::LEAP_EYE_EVENT;

use crate::LeapVector;

crate::leap_ref_struct!(EyeEvent, LEAP_EYE_EVENT);

impl<'a> EyeEvent<'a> {
    #[doc = " The position of the user's left eye."]
    #[doc = " @since 4.1.0"]
    pub fn left_eye_position(&self) -> LeapVector {
        self.handle.left_eye_position.into()
    }

    #[doc = " The position of the user's right eye."]
    #[doc = " @since 4.1.0"]
    pub fn right_eye_position(&self) -> LeapVector {
        self.handle.right_eye_position.into()
    }
}
