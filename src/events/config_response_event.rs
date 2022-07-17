use leap_sys::LEAP_CONFIG_RESPONSE_EVENT;

use crate::Variant;

crate::leap_ref_struct!(ConfigResponseEvent, LEAP_CONFIG_RESPONSE_EVENT);

impl<'a> ConfigResponseEvent<'a> {
    pub fn request_id(&self) -> u32 {
        self.handle.requestID
    }

    pub fn value(&self) -> Variant {
        self.handle.value.into()
    }
}
