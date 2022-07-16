use leap_sys::LEAP_QUATERNION;

crate::leap_struct!(Quaternion, LEAP_QUATERNION);

impl Quaternion {
    pub fn array(&self) -> [f32; 4] {
        unsafe { self.handle.__bindgen_anon_1.v }
    }

    pub fn x(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.x }
    }

    pub fn y(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.y }
    }

    pub fn z(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.z }
    }

    pub fn w(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.w }
    }
}
