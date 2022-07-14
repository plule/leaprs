use leap_sys::LEAP_DEVICE_INFO;
use num_enum::FromPrimitive;
use std::ffi::CStr;

use crate::{DevicePID, DeviceStatus};

pub trait DeviceInfo {
    fn get_serial(&self) -> Option<&str>;

    fn get_status(&self) -> Option<DeviceStatus>;

    fn get_pid(&self) -> DevicePID;
}

impl DeviceInfo for LEAP_DEVICE_INFO {
    fn get_serial(&self) -> Option<&str> {
        let serial = unsafe { CStr::from_ptr(self.serial) };
        serial.to_str().ok()
    }

    fn get_status(&self) -> Option<DeviceStatus> {
        DeviceStatus::from_bits(self.status)
    }

    fn get_pid(&self) -> DevicePID {
        DevicePID::from_primitive(self.pid)
    }
}

pub struct DeviceInfoWrapper {
    handle: LEAP_DEVICE_INFO,
    #[allow(dead_code)] // handle contains a pointer to serial
    serial: Vec<i8>,
}

impl DeviceInfoWrapper {
    pub fn new(handle: LEAP_DEVICE_INFO, serial: Vec<i8>) -> Self {
        Self { handle, serial }
    }

    pub fn get_info(&self) -> &LEAP_DEVICE_INFO {
        &self.handle
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
        let device_info_wrapper = device.get_info().expect("Failed to get device info");
        let device_info = device_info_wrapper.get_info();
        assert_ne!(device_info.get_pid(), DevicePID::Unknown);
        let serial = device_info.get_serial().expect("Failed to get serial");
        assert!(serial.len() > 0);
        device_info
            .get_status()
            .expect("Failed to get device status");
    }
}
