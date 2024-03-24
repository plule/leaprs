use std::ops::Deref;

use leap_sys::{_LEAP_VECTOR__bindgen_ty_1__bindgen_ty_1, LEAP_VECTOR};

#[doc = " A three element, floating-point vector."]
#[doc = " @since 3.0.0"]
pub struct LeapVectorRef<'a>(pub(crate) &'a LEAP_VECTOR);

impl<'a> LeapVectorRef<'a> {
    pub fn array(&self) -> [f32; 3] {
        unsafe { self.0.__bindgen_anon_1.v }
    }

    /// Convert to a [glam::Vec3]
    #[cfg(feature = "glam")]
    pub fn into_glam(&self) -> glam::Vec3 {
        glam::Vec3::new(self.x, self.y, self.z)
    }

    /// Convert to a [nalgebra::Vector3]
    #[cfg(feature = "nalgebra")]
    pub fn into_nalgebra(&self) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(self.x, self.y, self.z)
    }
}

impl<'a> Deref for LeapVectorRef<'a> {
    type Target = _LEAP_VECTOR__bindgen_ty_1__bindgen_ty_1;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.0.__bindgen_anon_1.__bindgen_anon_1 }
    }
}

impl Into<[f32; 3]> for LeapVectorRef<'_> {
    fn into(self) -> [f32; 3] {
        self.array()
    }
}

#[cfg(feature = "glam")]
impl From<LeapVectorRef<'_>> for glam::Vec3 {
    fn from(v: LeapVectorRef) -> glam::Vec3 {
        v.into_glam()
    }
}

#[cfg(feature = "nalgebra")]
impl From<LeapVectorRef<'_>> for nalgebra::Vector3<f32> {
    fn from(v: LeapVectorRef) -> nalgebra::Vector3<f32> {
        v.into_nalgebra()
    }
}

/// Build a native [LEAP_VECTOR].
/// Useful for method taking an owned [LEAP_VECTOR] as argument.
pub(crate) fn build_leap_vector<P: Into<[f32; 3]>>(v: P) -> LEAP_VECTOR {
    LEAP_VECTOR {
        __bindgen_anon_1: { leap_sys::_LEAP_VECTOR__bindgen_ty_1 { v: v.into() } },
    }
}

/// Convert a [LEAP_VECTOR] to an array.
pub(crate) fn leap_vector_to_array(v: LEAP_VECTOR) -> [f32; 3] {
    unsafe { v.__bindgen_anon_1.v }
}
