use leap_sys::LEAP_DEVICE_INFO;

pub struct DeviceInfo {
    handle: LEAP_DEVICE_INFO,
    serial: Vec<i8>,
}

impl DeviceInfo {
    pub fn new(handle: LEAP_DEVICE_INFO, serial: Vec<i8>) -> Self {
        Self { handle, serial }
    }

    pub fn get_info(&self) -> &LEAP_DEVICE_INFO {
        &self.handle
    }
    pub fn get_serial(&self) -> &Vec<i8> {
        &self.serial
    }
}
