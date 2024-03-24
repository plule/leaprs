use leap_sys::{_LEAP_QUATERNION__bindgen_ty_1__bindgen_ty_1, LEAP_QUATERNION};

#[doc = " A four element, floating point quaternion. @since 3.1.2"]
pub struct QuaternionRef<'a>(pub(crate) &'a LEAP_QUATERNION);

impl<'a> QuaternionRef<'a> {
    pub fn array(&self) -> [f32; 4] {
        unsafe { self.0.__bindgen_anon_1.v }
    }

    /// Convert to a [glam::Quat]
    #[cfg(feature = "glam")]
    pub fn into_glam(&self) -> glam::Quat {
        glam::Quat::from_xyzw(self.x, self.y, self.z, self.w)
    }

    /// Convert to a [nalgebra::UnitQuaternion]
    #[cfg(feature = "nalgebra")]
    pub fn into_nalgebra(&self) -> nalgebra::UnitQuaternion<f32> {
        nalgebra::UnitQuaternion::new_unchecked(nalgebra::Quaternion::new(
            self.w, self.x, self.y, self.z,
        ))
    }
}

impl<'a> core::ops::Deref for QuaternionRef<'a> {
    type Target = _LEAP_QUATERNION__bindgen_ty_1__bindgen_ty_1;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.0.__bindgen_anon_1.__bindgen_anon_1 }
    }
}

#[cfg(feature = "glam")]
impl From<QuaternionRef<'_>> for glam::Quat {
    fn from(q: QuaternionRef) -> glam::Quat {
        q.into_glam()
    }
}

#[cfg(feature = "nalgebra")]
impl From<QuaternionRef<'_>> for nalgebra::UnitQuaternion<f32> {
    fn from(q: QuaternionRef) -> nalgebra::UnitQuaternion<f32> {
        q.into_nalgebra()
    }
}
