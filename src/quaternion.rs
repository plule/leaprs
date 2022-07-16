use leap_sys::LEAP_QUATERNION;

pub trait Quaternion {
    fn array(&self) -> [f32; 4];
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn w(&self) -> f32;
}

impl Quaternion for LEAP_QUATERNION {
    fn array(&self) -> [f32; 4] {
        unsafe { self.__bindgen_anon_1.v }
    }

    fn x(&self) -> f32 {
        unsafe { self.__bindgen_anon_1.__bindgen_anon_1.x }
    }

    fn y(&self) -> f32 {
        unsafe { self.__bindgen_anon_1.__bindgen_anon_1.y }
    }

    fn z(&self) -> f32 {
        unsafe { self.__bindgen_anon_1.__bindgen_anon_1.z }
    }

    fn w(&self) -> f32 {
        unsafe { self.__bindgen_anon_1.__bindgen_anon_1.w }
    }
}
