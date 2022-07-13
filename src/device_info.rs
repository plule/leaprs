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

    pub fn get_serial_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.serial.iter().map(|&c| c as u8).collect())
    }
}
