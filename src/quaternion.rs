use leap_sys::LEAP_QUATERNION;

pub trait Quaternion {
    fn get_array(&self) -> [f32; 4];
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_z(&self) -> f32;
    fn get_w(&self) -> f32;
}

impl Quaternion for LEAP_QUATERNION {
    fn get_array(&self) -> [f32; 4] {
        unsafe { self.__bindgen_anon_1.v }
    }

    fn get_x(&self) -> f32 {
        unsafe { self.__bindgen_anon_1.__bindgen_anon_1.x }
    }

    fn get_y(&self) -> f32 {
        unsafe { self.__bindgen_anon_1.__bindgen_anon_1.y }
    }

    fn get_z(&self) -> f32 {
        unsafe { self.__bindgen_anon_1.__bindgen_anon_1.z }
    }

    fn get_w(&self) -> f32 {
        unsafe { self.__bindgen_anon_1.__bindgen_anon_1.w }
    }
}
