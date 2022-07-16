use leap_sys::LEAP_DEVICE_REF;

use crate::Device;

crate::leap_struct!(DeviceRef, LEAP_DEVICE_REF);

impl DeviceRef {
    pub fn open(&self) -> Result<Device, crate::Error> {
        Device::open(self.handle)
    }

    pub fn id(&self) -> u32 {
        self.handle.id
    }
}
