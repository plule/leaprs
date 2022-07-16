use leap_sys::{LEAP_BONE, LEAP_DIGIT};

pub trait Digit {
    fn bones(&self) -> &[LEAP_BONE; 4];

    fn metacarpal(&self) -> &LEAP_BONE;

    fn proximal(&self) -> &LEAP_BONE;

    fn intermediate(&self) -> &LEAP_BONE;

    fn distal(&self) -> &LEAP_BONE;
}

impl Digit for LEAP_DIGIT {
    fn bones(&self) -> &[LEAP_BONE; 4] {
        unsafe { &self.__bindgen_anon_1.bones }
    }

    fn metacarpal(&self) -> &LEAP_BONE {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.metacarpal }
    }

    fn proximal(&self) -> &LEAP_BONE {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.metacarpal }
    }

    fn intermediate(&self) -> &LEAP_BONE {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.intermediate }
    }

    fn distal(&self) -> &LEAP_BONE {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.distal }
    }
}
