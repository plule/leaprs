use derive_deref::Deref;
use leap_sys::LEAP_DIGIT;

use crate::Bone;

#[doc = " Describes the digit of a hand."]
#[doc = " Digits are members of the LEAP_HAND struct."]
#[doc = " @since 3.0.0"]
#[derive(Deref)]
pub struct Digit<'a>(pub(crate) &'a LEAP_DIGIT);

impl<'a> Digit<'a> {
    #[doc = " All the bones of a digit as an iterable collection. @since 3.0.0"]
    pub fn bones(&self) -> [Bone; 4] {
        let bones;
        unsafe { bones = &self.__bindgen_anon_1.bones }
        [
            Bone(&bones[0]),
            Bone(&bones[1]),
            Bone(&bones[2]),
            Bone(&bones[3]),
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
        unsafe { Bone(&self.__bindgen_anon_1.__bindgen_anon_1.metacarpal) }
    }

    #[doc = " The phalange extending from the knuckle. @since 3.0.0"]
    pub fn proximal(&self) -> Bone {
        unsafe { Bone(&self.__bindgen_anon_1.__bindgen_anon_1.proximal) }
    }

    pub fn intermediate(&self) -> Bone {
        unsafe { Bone(&self.__bindgen_anon_1.__bindgen_anon_1.intermediate) }
    }

    #[doc = " The bone between the proximal phalange and the distal phalange. @since 3.0.0"]
    pub fn distal(&self) -> Bone {
        unsafe { Bone(&self.__bindgen_anon_1.__bindgen_anon_1.distal) }
    }

    #[doc = " Reports whether the finger is more or less straight. @since 3.0.0"]
    pub fn is_extended(&self) -> bool {
        self.is_extended == 1
    }
}
