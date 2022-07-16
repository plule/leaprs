use crate::{leap_try, DeviceInfo, Error};
use leap_sys::*;

crate::leap_struct!(Device, LEAP_DEVICE);

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { LeapCloseDevice(self.handle) };
    }
}

impl Device {
    pub fn open(device_ref: LEAP_DEVICE_REF) -> Result<Self, Error> {
        let mut handle: LEAP_DEVICE;
        unsafe {
            handle = std::mem::zeroed();
            leap_try(LeapOpenDevice(device_ref, &mut handle))?;
        }
        Ok(handle.into())
    }

    pub fn get_info_raw(&mut self, info: &mut LEAP_DEVICE_INFO, serial: &mut Vec<i8>) -> eLeapRS {
        info.serial_length = serial.len() as u32;
        info.serial = serial.as_mut_ptr();
        unsafe { LeapGetDeviceInfo(self.handle, info) }
    }

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
