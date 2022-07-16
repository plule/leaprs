use leap_sys::LEAP_DEVICE_EVENT;

use crate::{DeviceRef, DeviceStatus};

pub struct DeviceEvent<'a> {
    pub handle: &'a LEAP_DEVICE_EVENT,
}

impl<'a> From<&'a LEAP_DEVICE_EVENT> for DeviceEvent<'a> {
    fn from(handle: &'a LEAP_DEVICE_EVENT) -> Self {
        Self { handle }
    }
}

impl<'a> DeviceEvent<'a> {
    pub fn get_device(&self) -> DeviceRef {
        self.handle.device.into()
    }

    pub fn get_status(&self) -> Option<DeviceStatus> {
        DeviceStatus::from_bits(self.handle.status)
    }
}
