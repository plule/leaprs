use leap_sys::LEAP_DEVICE_STATUS_CHANGE_EVENT;

use crate::{DeviceRef, DeviceStatus};

crate::leap_ref_struct!(DeviceStatusChangeEvent, LEAP_DEVICE_STATUS_CHANGE_EVENT);

impl<'a> DeviceStatusChangeEvent<'a> {
    pub fn device(&self) -> DeviceRef {
        self.handle.device.into()
    }

    pub fn status(&self) -> Option<DeviceStatus> {
        DeviceStatus::from_bits(self.handle.status)
    }

    pub fn last_status(&self) -> Option<DeviceStatus> {
        DeviceStatus::from_bits(self.handle.last_status)
    }
}
