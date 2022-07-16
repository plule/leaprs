use leap_sys::LEAP_DIGIT;

use crate::Bone;

crate::leap_ref_struct!(Digit, LEAP_DIGIT);

impl<'a> Digit<'a> {
    pub fn finger_id(&self) -> i32 {
        self.handle.finger_id
    }

    pub fn bones(&self) -> [Bone; 4] {
        let bones;
        unsafe { bones = &self.handle.__bindgen_anon_1.bones }
        [
            (&bones[0]).into(),
            (&bones[1]).into(),
            (&bones[2]).into(),
            (&bones[3]).into(),
        ]
    }

    pub fn metacarpal(&self) -> Bone {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.metacarpal }.into()
    }

    pub fn proximal(&self) -> Bone {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.metacarpal }.into()
    }

    pub fn intermediate(&self) -> Bone {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.intermediate }.into()
    }

    pub fn distal(&self) -> Bone {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.distal }.into()
    }

    pub fn is_extended(&self) -> u32 {
        self.handle.is_extended
    }
}
