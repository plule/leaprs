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
