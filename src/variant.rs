use leap_sys::{
    _eLeapValueType_eLeapValueType_Boolean, _eLeapValueType_eLeapValueType_Float,
    _eLeapValueType_eLeapValueType_Int32, _eLeapValueType_eLeapValueType_String, LEAP_VARIANT,
};
use std::ffi::CString;

#[doc = " A variant data type used to get and set service configuration values."]
#[doc = " @since 3.0.0"]
pub enum Variant {
    #[doc = " A boolean value. @since 3.0.0"]
    Boolean(bool),
    #[doc = " An integer value. @since 3.0.0"]
    Int32(i32),
    #[doc = " A floating point value. @since 3.0.0"]
    Float(f32),
    #[doc = " A string value. @since 3.0.0"]
    String(CString),
}

impl From<Variant> for LEAP_VARIANT {
    fn from(variant: Variant) -> Self {
        match variant {
            Variant::Boolean(v) => LEAP_VARIANT {
                type_: _eLeapValueType_eLeapValueType_Boolean,
                __bindgen_anon_1: leap_sys::_LEAP_VARIANT__bindgen_ty_1 { boolValue: v },
            },
            Variant::Int32(v) => LEAP_VARIANT {
                type_: _eLeapValueType_eLeapValueType_Int32,
                __bindgen_anon_1: leap_sys::_LEAP_VARIANT__bindgen_ty_1 { iValue: v },
            },
            Variant::Float(v) => LEAP_VARIANT {
                type_: _eLeapValueType_eLeapValueType_Float,
                __bindgen_anon_1: leap_sys::_LEAP_VARIANT__bindgen_ty_1 { fValue: v },
            },
            Variant::String(v) => LEAP_VARIANT {
                type_: _eLeapValueType_eLeapValueType_String,
                __bindgen_anon_1: leap_sys::_LEAP_VARIANT__bindgen_ty_1 {
                    strValue: v.as_ptr(),
                },
            },
        }
    }
}

impl From<LEAP_VARIANT> for Variant {
    fn from(v: LEAP_VARIANT) -> Self {
        match v.type_ {
            leap_sys::_eLeapValueType_eLeapValueType_Boolean => {
                Variant::Boolean(unsafe { v.__bindgen_anon_1.boolValue })
            }
            leap_sys::_eLeapValueType_eLeapValueType_Int32 => {
                Variant::Int32(unsafe { v.__bindgen_anon_1.iValue })
            }
            leap_sys::_eLeapValueType_eLeapValueType_Float => {
                Variant::Float(unsafe { v.__bindgen_anon_1.fValue })
            }
            leap_sys::_eLeapValueType_eLeapValueType_String => Variant::String(unsafe {
                // Could not find a way to create a const cstring, create a clone
                CString::from_raw(v.__bindgen_anon_1.strValue as *mut i8)
            }),
            _ => todo!(),
        }
    }
}
