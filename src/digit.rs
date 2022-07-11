use leap_sys::*;

pub trait Digit {
    fn get_bones(&self) -> &[LEAP_BONE; 4];

    fn get_metacarpal(&self) -> &LEAP_BONE;

    fn get_proximal(&self) -> &LEAP_BONE;

    fn get_intermediate(&self) -> &LEAP_BONE;

    fn get_distal(&self) -> &LEAP_BONE;
}

impl Digit for LEAP_DIGIT {
    fn get_bones(&self) -> &[LEAP_BONE; 4] {
        unsafe { &self.__bindgen_anon_1.bones }
    }

    fn get_metacarpal(&self) -> &LEAP_BONE {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.metacarpal }
    }

    fn get_proximal(&self) -> &LEAP_BONE {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.metacarpal }
    }

    fn get_intermediate(&self) -> &LEAP_BONE {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.intermediate }
    }

    fn get_distal(&self) -> &LEAP_BONE {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.distal }
    }
}
