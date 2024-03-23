use derive_deref::Deref;
use leap_sys::{_LEAP_QUATERNION__bindgen_ty_1__bindgen_ty_1, LEAP_QUATERNION};

#[doc = " A four element, floating point quaternion. @since 3.1.2"]
#[derive(Deref)]
pub struct Quaternion<'a>(pub(crate) &'a _LEAP_QUATERNION__bindgen_ty_1__bindgen_ty_1);

impl<'a> From<&'a LEAP_QUATERNION> for Quaternion<'a> {
    fn from(quaternion: &'a LEAP_QUATERNION) -> Self {
        Self(unsafe { &quaternion.__bindgen_anon_1.__bindgen_anon_1 })
    }
}

impl<'a> Quaternion<'a> {
    /// Convert to a [glam::Quat]
    #[cfg(feature = "glam")]
    pub fn into_glam(&self) -> glam::Quat {
        glam::Quat::from_xyzw(self.x, self.y, self.z, self.w)
    }

    /// Convert to a [nalgebra::UnitQuaternion]
    #[cfg(feature = "nalgebra")]
    pub fn to_nalgebra(&self) -> nalgebra::UnitQuaternion<f32> {
        nalgebra::UnitQuaternion::new_unchecked(nalgebra::Quaternion::new(
            self.w, self.x, self.y, self.z,
        ))
    }
}

#[cfg(feature = "glam")]
impl From<Quaternion<'_>> for glam::Quat {
    fn from(q: Quaternion) -> glam::Quat {
        q.into_glam()
    }
}

#[cfg(feature = "nalgebra")]
impl From<Quaternion<'_>> for nalgebra::UnitQuaternion<f32> {
    fn from(q: Quaternion) -> nalgebra::UnitQuaternion<f32> {
        q.to_nalgebra()
    }
}
