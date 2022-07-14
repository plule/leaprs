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
