use crate::{leap_try, DeviceInfo, Error};
use leap_sys::*;

#[doc = " A handle to a Leap device object."]
#[doc = " Use this handle to specify the device for an operation."]
#[doc = " @since 3.0.0"]
pub struct Device {
    pub(crate) handle: LEAP_DEVICE,
    drop: bool,
}

impl From<LEAP_DEVICE> for Device {
    fn from(handle: LEAP_DEVICE) -> Self {
        Self { handle, drop: true }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.drop {
            unsafe { LeapCloseDevice(self.handle) };
        }
    }
}

impl Device {
    #[doc = " Opens a device reference and retrieves a handle to the device."]
    #[doc = ""]
    #[doc = " To ensure resources are properly freed, users must call LeapCloseDevice()"]
    #[doc = " when finished with the device, even if the retrieved device has problems"]
    #[doc = " or cannot stream."]
    #[doc = ""]
    #[doc = " @param rDevice A device reference."]
    #[doc = " @param[out] phDevice A pointer that receives the opened device handle."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn open(device_ref: LEAP_DEVICE_REF) -> Result<Self, Error> {
        let mut handle: LEAP_DEVICE;
        unsafe {
            handle = std::mem::zeroed();
            leap_try(LeapOpenDevice(device_ref, &mut handle))?;
        }
        Ok(handle.into())
    }

    pub(crate) fn attach(handle: LEAP_DEVICE) -> Self {
        Self {
            handle,
            drop: false,
        }
    }

    #[doc = " Gets device properties."]
    #[doc = ""]
    #[doc = " To get the device serial number, you must supply a LEAP_DEVICE_INFO struct whose"]
    #[doc = " serial member points to a char array large enough to hold the null-terminated"]
    #[doc = " serial number string. To get the required length, call LeapGetDeviceInfo() using"]
    #[doc = " a LEAP_DEVICE_INFO struct that has serial set to NULL. LeapC sets serial_length field of"]
    #[doc = " the struct to the required length. You can then allocate memory for the string,"]
    #[doc = " set the serial field, and call this function again."]
    #[doc = ""]
    #[doc = " @param hDevice A handle to the device to be queried."]
    #[doc = " @param[out] info The struct to receive the device property data."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn get_info_raw(&mut self, info: &mut LEAP_DEVICE_INFO, serial: &mut Vec<i8>) -> eLeapRS {
        info.serial_length = serial.len() as u32;
        info.serial = serial.as_mut_ptr();
        unsafe { LeapGetDeviceInfo(self.handle, info) }
    }

    #[doc = " Gets device properties."]
    pub fn get_info(&mut self) -> Result<DeviceInfo, Error> {
        let mut serial: Vec<i8> = vec![0];
        let mut info: LEAP_DEVICE_INFO = LEAP_DEVICE_INFO {
            size: std::mem::size_of::<LEAP_DEVICE_INFO>() as u32,
            status: 0,
            caps: 0,
            pid: 0,
            baseline: 0,
            serial_length: 0,
            serial: serial.as_mut_ptr(),
            h_fov: 0.0,
            v_fov: 0.0,
            range: 0,
        };

        // First call to get serial length
        let mut res = self.get_info_raw(&mut info, &mut serial);

        if res == _eLeapRS_eLeapRS_InsufficientBuffer {
            // Second call to get serial
            serial.resize(info.serial_length as usize, 0);
            res = self.get_info_raw(&mut info, &mut serial);
        }

        // Don't return the struct on error
        leap_try(res)?;

        Ok(DeviceInfo::new(info, serial))
    }
}
