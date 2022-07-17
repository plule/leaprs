use leap_sys::LEAP_QUATERNION;

crate::leap_struct!(
    #[doc = " A four element, floating point quaternion. @since 3.1.2"]
    Quaternion,
    LEAP_QUATERNION
);

impl Quaternion {
    #[doc = " The quaternion as an array. @since 3.1.3"]
    pub fn array(&self) -> [f32; 4] {
        unsafe { self.handle.__bindgen_anon_1.v }
    }

    #[doc = " The x coefficient of the vector portion of the quaternion. @since 3.1.2"]
    pub fn x(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.x }
    }

    #[doc = " The y coefficient of the vector portion of the quaternion. @since 3.1.2"]
    pub fn y(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.y }
    }

    #[doc = " The z coefficient of the vector portion of the quaternion. @since 3.1.2"]
    pub fn z(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.z }
    }

    #[doc = " The scalar portion of the quaternion. @since 3.1.2"]
    pub fn w(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.w }
    }
}
