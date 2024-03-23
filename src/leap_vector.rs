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

impl<'a> LeapVector<'a> {
    /// Convert to a [glam::Vec3]
    #[cfg(feature = "glam")]
    pub fn into_glam(&self) -> glam::Vec3 {
        glam::Vec3::new(self.x, self.y, self.z)
    }

    /// Convert to a [nalgebra::Vector3]
    #[cfg(feature = "nalgebra")]
    pub fn to_nalgebra(&self) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(self.x, self.y, self.z)
    }
}

#[cfg(feature = "glam")]
impl From<LeapVector<'_>> for glam::Vec3 {
    fn from(v: LeapVector) -> glam::Vec3 {
        v.into_glam()
    }
}

#[cfg(feature = "nalgebra")]
impl From<LeapVector<'_>> for nalgebra::Vector3<f32> {
    fn from(v: LeapVector) -> nalgebra::Vector3<f32> {
        v.to_nalgebra()
    }
}

/// Build a native [LEAP_VECTOR].
/// Useful for method taking an owned [LEAP_VECTOR] as argument.
pub(crate) fn build_leap_vector(v: [f32; 3]) -> LEAP_VECTOR {
    LEAP_VECTOR {
        __bindgen_anon_1: { leap_sys::_LEAP_VECTOR__bindgen_ty_1 { v } },
    }
}

/// Convert a [LEAP_VECTOR] to an array.
pub(crate) fn leap_vector_to_array(v: LEAP_VECTOR) -> [f32; 3] {
    unsafe { v.__bindgen_anon_1.v }
}
