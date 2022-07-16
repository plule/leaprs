use leap_sys::LEAP_DEVICE_STATUS_CHANGE_EVENT;

use crate::DeviceStatus;

pub trait DeviceStatusChangeEvent {
    fn status(&self) -> Option<DeviceStatus>;

    fn last_status(&self) -> Option<DeviceStatus>;
}

impl DeviceStatusChangeEvent for LEAP_DEVICE_STATUS_CHANGE_EVENT {
    fn status(&self) -> Option<DeviceStatus> {
        DeviceStatus::from_bits(self.status)
    }

    fn last_status(&self) -> Option<DeviceStatus> {
        DeviceStatus::from_bits(self.last_status)
    }
}
