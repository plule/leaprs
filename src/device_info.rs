use leap_sys::LEAP_DEVICE_INFO;
use num_enum::FromPrimitive;
use std::ffi::CStr;

use crate::{DevicePID, DeviceStatus};

#[doc = " Properties of a Leap device."]
#[doc = " Get a LEAP_DEVICE_INFO by calling LeapGetDeviceInfo() with the handle for"]
#[doc = " device. The device must be open."]
#[doc = " @since 3.0.0"]
pub struct DeviceInfo {
    handle: LEAP_DEVICE_INFO,
    #[allow(dead_code)] // handle contains a pointer to serial
    serial: Vec<i8>,
}

impl DeviceInfo {
    pub(crate) fn new(handle: LEAP_DEVICE_INFO, serial: Vec<i8>) -> Self {
        Self { handle, serial }
    }

    #[doc = " A combination of eLeapDeviceStatus flags. @since 3.0.0"]
    pub fn status(&self) -> Option<DeviceStatus> {
        DeviceStatus::from_bits(self.handle.status)
    }

    // TODO get_caps

    #[doc = " One of the eLeapDevicePID members. @since 3.0.0"]
    pub fn pid(&self) -> DevicePID {
        DevicePID::from_primitive(self.handle.pid)
    }

    #[doc = " The device baseline, in micrometers."]
    #[doc = ""]
    #[doc = " The baseline is defined as the distance between the center axis of each lens in a stereo camera"]
    #[doc = " system. For other camera systems, this value is set to zero."]
    #[doc = " @since 3.0.0"]
    pub fn baseline(&self) -> u32 {
        self.handle.baseline
    }

    #[doc = " A pointer to the null-terminated device serial number string. @since 3.0.0"]
    pub fn serial(&self) -> Option<&str> {
        let serial = unsafe { CStr::from_ptr(self.handle.serial) };
        serial.to_str().ok()
    }

    #[doc = " The horizontal field of view of this device in radians. @since 3.0.0"]
    pub fn h_fov(&self) -> f32 {
        self.handle.h_fov
    }

    #[doc = " The vertical field of view of this device in radians. @since 3.0.0"]
    pub fn v_fov(&self) -> f32 {
        self.handle.v_fov
    }

    #[doc = " The maximum range for this device, in micrometers. @since 3.0.0"]
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
