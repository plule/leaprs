use leap_sys::LEAP_DEVICE_REF;

pub struct DeviceRef {
    handle: LEAP_DEVICE_REF,
}

impl From<LEAP_DEVICE_REF> for DeviceRef {
    fn from(device: LEAP_DEVICE_REF) -> Self {
        Self { handle: device }
    }
}
