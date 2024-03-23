use derive_deref::Deref;

use leap_sys::{_LEAP_VECTOR__bindgen_ty_1__bindgen_ty_1, LEAP_VECTOR};

#[doc = " A three element, floating-point vector."]
#[doc = " @since 3.0.0"]
#[derive(Deref)]
pub struct LeapVector<'a>(pub(crate) &'a _LEAP_VECTOR__bindgen_ty_1__bindgen_ty_1);

impl<'a> From<&'a LEAP_VECTOR> for LeapVector<'a> {
    fn from(vector: &'a LEAP_VECTOR) -> Self {
        Self(unsafe { &vector.__bindgen_anon_1.__bindgen_anon_1 })
    }
}

/// Build a native [LEAP_VECTOR].
/// Useful for method taking an owned [LEAP_VECTOR] as argument.
pub fn build_leap_vector(v: [f32; 3]) -> LEAP_VECTOR {
    LEAP_VECTOR {
        __bindgen_anon_1: { leap_sys::_LEAP_VECTOR__bindgen_ty_1 { v } },
    }
}

/// Convert a [LEAP_VECTOR] to an array.
pub fn leap_vector_to_array(v: LEAP_VECTOR) -> [f32; 3] {
    unsafe { v.__bindgen_anon_1.v }
}
