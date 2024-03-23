use leap_sys::LEAP_DIGIT;

use crate::Bone;

crate::leap_ref_struct!(
    #[doc = " Describes the digit of a hand."]
    #[doc = " Digits are members of the LEAP_HAND struct."]
    #[doc = " @since 3.0.0"]
    Digit,
    LEAP_DIGIT
);

impl<'a> Digit<'a> {
    #[doc = " All the bones of a digit as an iterable collection. @since 3.0.0"]
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

    #[doc = " The finger bone wholly inside the hand."]
    #[doc = " For thumbs, this bone is set to have zero length and width, an identity basis matrix,"]
    #[doc = " and its joint positions are equal."]
    #[doc = " Note that this is anatomically incorrect; in anatomical terms, the intermediate phalange"]
    #[doc = " is absent in a real thumb, rather than the metacarpal bone. In the Ultraleap Tracking model,"]
    #[doc = " however, we use a \"zero\" metacarpal bone instead for ease of programming."]
    #[doc = " @since 3.0.0"]
    pub fn metacarpal(&self) -> Bone {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.metacarpal }.into()
    }

    #[doc = " The phalange extending from the knuckle. @since 3.0.0"]
    pub fn proximal(&self) -> Bone {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.proximal }.into()
    }

    pub fn intermediate(&self) -> Bone {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.intermediate }.into()
    }

    #[doc = " The bone between the proximal phalange and the distal phalange. @since 3.0.0"]
    pub fn distal(&self) -> Bone {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.distal }.into()
    }

    #[doc = " Reports whether the finger is more or less straight. @since 3.0.0"]
    pub fn is_extended(&self) -> bool {
        self.handle.is_extended == 1
    }
}
