use leap_sys::*;

pub trait Hand {
    fn get_digits(&self) -> &[LEAP_DIGIT; 5];
    fn get_thumb(&self) -> &LEAP_DIGIT;
    fn get_index(&self) -> &LEAP_DIGIT;
    fn get_middle(&self) -> &LEAP_DIGIT;
    fn get_ring(&self) -> &LEAP_DIGIT;
    fn get_pinky(&self) -> &LEAP_DIGIT;
}

impl Hand for LEAP_HAND {
    fn get_digits(&self) -> &[LEAP_DIGIT; 5] {
        unsafe { &self.__bindgen_anon_1.digits }
    }

    fn get_thumb(&self) -> &LEAP_DIGIT {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.thumb }
    }

    fn get_index(&self) -> &LEAP_DIGIT {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.index }
    }

    fn get_middle(&self) -> &LEAP_DIGIT {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.middle }
    }

    fn get_ring(&self) -> &LEAP_DIGIT {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.ring }
    }

    fn get_pinky(&self) -> &LEAP_DIGIT {
        unsafe { &self.__bindgen_anon_1.__bindgen_anon_1.pinky }
    }
}
