use leap_sys::LEAP_VECTOR;

crate::leap_struct!(LeapVector, LEAP_VECTOR);

impl LeapVector {
    pub fn array(&self) -> [f32; 3] {
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
}
