use leap_sys::LEAP_DEVICE_REF;

use crate::Device;

pub struct DeviceRef {
    handle: LEAP_DEVICE_REF,
}

impl From<LEAP_DEVICE_REF> for DeviceRef {
    fn from(handle: LEAP_DEVICE_REF) -> Self {
        Self { handle }
    }
}

impl DeviceRef {
    pub fn open(&self) -> Result<Device, crate::Error> {
        Device::open(self.handle)
    }

    pub fn get_id(&self) -> u32 {
        self.handle.id
    }
}
