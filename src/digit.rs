use derive_deref::Deref;
use leap_sys::LEAP_DIGIT;

use crate::BoneRef;

#[doc = " Describes the digit of a hand."]
#[doc = " Digits are members of the LEAP_HAND struct."]
#[doc = " @since 3.0.0"]
/// # Fields
/// Available via dereference: [LEAP_DIGIT].
#[derive(Deref, Clone, Copy)]
pub struct DigitRef<'a>(pub(crate) &'a LEAP_DIGIT);

impl<'a> DigitRef<'a> {
    #[doc = " All the bones of a digit as an iterable collection. @since 3.0.0"]
    pub fn bones(&self) -> [BoneRef; 4] {
        let bones;
        unsafe { bones = &self.__bindgen_anon_1.bones }
        [
            BoneRef(&bones[0]),
            BoneRef(&bones[1]),
            BoneRef(&bones[2]),
            BoneRef(&bones[3]),
        ]
    }

    #[doc = " The finger bone wholly inside the hand."]
    #[doc = " For thumbs, this bone is set to have zero length and width, an identity basis matrix,"]
    #[doc = " and its joint positions are equal."]
    #[doc = " Note that this is anatomically incorrect; in anatomical terms, the intermediate phalange"]
    #[doc = " is absent in a real thumb, rather than the metacarpal bone. In the Ultraleap Tracking model,"]
    #[doc = " however, we use a \"zero\" metacarpal bone instead for ease of programming."]
    #[doc = " @since 3.0.0"]
    pub fn metacarpal(&self) -> BoneRef {
        unsafe { BoneRef(&self.__bindgen_anon_1.__bindgen_anon_1.metacarpal) }
    }

    #[doc = " The phalange extending from the knuckle. @since 3.0.0"]
    pub fn proximal(&self) -> BoneRef {
        unsafe { BoneRef(&self.__bindgen_anon_1.__bindgen_anon_1.proximal) }
    }

    pub fn intermediate(&self) -> BoneRef {
        unsafe { BoneRef(&self.__bindgen_anon_1.__bindgen_anon_1.intermediate) }
    }

    #[doc = " The bone between the proximal phalange and the distal phalange. @since 3.0.0"]
    pub fn distal(&self) -> BoneRef {
        unsafe { BoneRef(&self.__bindgen_anon_1.__bindgen_anon_1.distal) }
    }

    #[doc = " Reports whether the finger is more or less straight. @since 3.0.0"]
    pub fn is_extended(&self) -> bool {
        self.is_extended == 1
    }
}
