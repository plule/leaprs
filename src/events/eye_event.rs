use leap_sys::LEAP_EYE_EVENT;

use crate::LeapVector;

crate::leap_ref_struct!(EyeEvent, LEAP_EYE_EVENT);

impl<'a> EyeEvent<'a> {
    #[doc = " The ID of the frame corresponding to the source of the currently tracked"]
    #[doc = " eye positions."]
    #[doc = " @since 4.1.0"]
    pub fn frame_id(&self) -> i64 {
        self.handle.frame_id
    }

    #[doc = " The timestamp for this image, in microseconds, referenced against"]
    #[doc = " LeapGetNow()."]
    #[doc = " @since 4.1.0"]
    pub fn timestamp(&self) -> i64 {
        self.handle.timestamp
    }

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

    #[doc = " An error estimate of the tracked left eye position. Higher values indicate"]
    #[doc = " uncertain tracking and a higher likelihood of there being no such eye in"]
    #[doc = " view of the sensor."]
    #[doc = " @since 4.1.0"]
    pub fn left_eye_estimated_error(&self) -> f32 {
        self.handle.left_eye_estimated_error
    }

    #[doc = " An error estimate of the tracked right eye position. Higher values indicate"]
    #[doc = " uncertain tracking and a higher likelihood of there being no such eye in"]
    #[doc = " view of the sensor."]
    #[doc = " @since 4.1.0"]
    pub fn right_eye_estimated_error(&self) -> f32 {
        self.handle.right_eye_estimated_error
    }
}
