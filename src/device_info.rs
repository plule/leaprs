use leap_sys::LEAP_DEVICE_INFO;
use num_enum::FromPrimitive;
use std::ffi::CStr;

use crate::{DevicePID, DeviceStatus};

pub struct DeviceInfo {
    handle: LEAP_DEVICE_INFO,
    #[allow(dead_code)] // handle contains a pointer to serial
    serial: Vec<i8>,
}

impl DeviceInfo {
    pub(crate) fn new(handle: LEAP_DEVICE_INFO, serial: Vec<i8>) -> Self {
        Self { handle, serial }
    }

    pub fn status(&self) -> Option<DeviceStatus> {
        DeviceStatus::from_bits(self.handle.status)
    }

    // TODO get_caps

    pub fn pid(&self) -> DevicePID {
        DevicePID::from_primitive(self.handle.pid)
    }

    pub fn baseline(&self) -> u32 {
        self.handle.baseline
    }

    pub fn serial(&self) -> Option<&str> {
        let serial = unsafe { CStr::from_ptr(self.handle.serial) };
        serial.to_str().ok()
    }

    pub fn h_fov(&self) -> f32 {
        self.handle.h_fov
    }

    pub fn v_fov(&self) -> f32 {
        self.handle.v_fov
    }

    pub fn range(&self) -> u32 {
        self.handle.range
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    fn get_device_info() {
        let mut connection = initialize_connection();
        let devices = connection
            .get_device_list()
            .expect("Failed to list devices");
        let device_info = devices.first().expect("No devices plugged for tests.");
        let mut device = device_info.open().expect("Failed to open the device");
        let device_info = device.get_info().expect("Failed to get device info");
        assert_ne!(device_info.pid(), DevicePID::Unknown);
        let serial = device_info.serial().expect("Failed to get serial");
        assert!(serial.len() > 0);
        device_info.status().expect("Failed to get device status");
    }
}
