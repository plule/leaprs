use leap_sys::LEAP_CONFIG_CHANGE_EVENT;

crate::leap_ref_struct!(ConfigChangeEvent, LEAP_CONFIG_CHANGE_EVENT);

impl<'a> ConfigChangeEvent<'a> {
    fn request_id(&self) -> u32 {
        self.handle.requestID
    }

    fn status(&self) -> bool {
        self.handle.status
    }
}
