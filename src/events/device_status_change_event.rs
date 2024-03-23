use derive_deref::Deref;
use leap_sys::LEAP_DEVICE_STATUS_CHANGE_EVENT;

use crate::{DeviceRef, DeviceStatus};

#[doc = " A notification that a device's status has changed. One of these messages is received by the client"]
#[doc = " as soon as the service is connected, or when a new device is attached."]
#[doc = " @since 3.1.3"]
#[derive(Deref)]
pub struct DeviceStatusChangeEvent<'a>(pub(crate) &'a LEAP_DEVICE_STATUS_CHANGE_EVENT);

impl<'a> DeviceStatusChangeEvent<'a> {
    #[doc = " A reference to the device whose status has changed"]
    pub fn device(&self) -> DeviceRef {
        DeviceRef(self.device)
    }

    #[doc = " The last known status of the device. This is a combination of eLeapDeviceStatus flags. @since 3.1.3"]
    pub fn status(&self) -> DeviceStatus {
        DeviceStatus::from_bits_truncate(self.status)
    }

    #[doc = " The current status of the device. This is a combination of eLeapDeviceStatus flags. @since 3.1.3"]
    pub fn last_status(&self) -> DeviceStatus {
        DeviceStatus::from_bits_truncate(self.last_status)
    }
}
