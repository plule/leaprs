use crate::{leap_try, DeviceInfo, Error};
use leap_sys::*;

pub struct Device {
    handle: LEAP_DEVICE,
}

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
        Ok(Self { handle })
    }

    pub fn get_info_serial_length(&mut self) -> Result<u32, Error> {
        unsafe {
            let mut info: LEAP_DEVICE_INFO = std::mem::zeroed();

            let res = LeapGetDeviceInfo(self.handle, &mut info);
            if res == _eLeapRS_eLeapRS_InsufficientBuffer || res == _eLeapRS_eLeapRS_Success {
                let len = info.serial_length;
                let hfov = info.h_fov;
                println!("{}", hfov);
                return Ok(info.serial_length);
            }
            Err(res.into())
        }
    }

    pub fn get_info(&mut self, serial_length: u32) -> Result<DeviceInfo, Error> {
        unsafe {
            let mut serial: Vec<i8> = vec![0; serial_length as usize];
            let mut info: LEAP_DEVICE_INFO = LEAP_DEVICE_INFO {
                size: 0,
                status: 0,
                caps: 0,
                pid: 0,
                baseline: 0,
                serial_length,
                serial: serial.as_mut_ptr(),
                h_fov: 0.0,
                v_fov: 0.0,
                range: 0,
            };
            leap_try(LeapGetDeviceInfo(self.handle, &mut info))?;
            Ok(DeviceInfo::new(info, serial))
        }
    }
}
