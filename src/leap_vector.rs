use leap_sys::LEAP_VECTOR;

pub trait LeapVector {
    fn get_array(&self) -> [f32; 3];
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_z(&self) -> f32;
}

impl LeapVector for LEAP_VECTOR {
    fn get_array(&self) -> [f32; 3] {
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
}
