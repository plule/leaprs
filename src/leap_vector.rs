use leap_sys::LEAP_VECTOR;

crate::leap_struct!(
    #[doc = " A three element, floating-point vector."]
    #[doc = " @since 3.0.0"]
    LeapVector,
    LEAP_VECTOR
);

impl LeapVector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            handle: LEAP_VECTOR {
                __bindgen_anon_1: { leap_sys::_LEAP_VECTOR__bindgen_ty_1 { v: [x, y, z] } },
            },
        }
    }

    #[doc = " The vector as an array. @since 3.0.0"]
    pub fn array(&self) -> [f32; 3] {
        unsafe { self.handle.__bindgen_anon_1.v }
    }

    #[doc = " The x spatial coordinate. @since 3.0.0"]
    pub fn x(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.x }
    }

    #[doc = " The y spatial coordinate. @since 3.0.0"]
    pub fn y(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.y }
    }

    #[doc = " The z spatial coordinate. @since 3.0.0"]
    pub fn z(&self) -> f32 {
        unsafe { self.handle.__bindgen_anon_1.__bindgen_anon_1.z }
    }
}
