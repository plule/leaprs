use leap_sys::LEAP_DEVICE_REF;

use crate::Device;

pub trait DeviceRef {
    fn open(&self) -> Result<Device, crate::Error>;
}

impl DeviceRef for LEAP_DEVICE_REF {
    fn open(&self) -> Result<Device, crate::Error> {
        Device::open(*self)
    }
}
