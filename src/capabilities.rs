use bitflags::bitflags;
use leap_sys::*;

bitflags! {
    #[doc = " Flags enumerating Leap device capabilities. @since 3.0.0"]
    pub struct Capabilities: u32 {
        #[doc = " The device can send color images. @since 3.0.0"]
        const COLOR = eLeapDeviceCaps_eLeapDeviceCaps_Color as u32;
    }
}
