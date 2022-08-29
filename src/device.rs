use crate::{leap_try, DeviceInfo, Error};
use leap_sys::*;

#[doc = " A handle to a Leap device object."]
#[doc = " Use this handle to specify the device for an operation."]
#[doc = " @since 3.0.0"]
pub struct Device {
    pub(crate) handle: LEAP_DEVICE,
    drop: bool,
}

impl From<LEAP_DEVICE> for Device {
    fn from(handle: LEAP_DEVICE) -> Self {
        Self { handle, drop: true }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.drop {
            unsafe { LeapCloseDevice(self.handle) };
        }
    }
}

impl Device {
    #[doc = " Opens a device reference and retrieves a handle to the device."]
    #[doc = ""]
    #[doc = " To ensure resources are properly freed, users must call LeapCloseDevice()"]
    #[doc = " when finished with the device, even if the retrieved device has problems"]
    #[doc = " or cannot stream."]
    #[doc = ""]
    #[doc = " @param rDevice A device reference."]
    #[doc = " @param[out] phDevice A pointer that receives the opened device handle."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn open(device_ref: LEAP_DEVICE_REF) -> Result<Self, Error> {
        let mut handle: LEAP_DEVICE;
        unsafe {
            handle = std::mem::zeroed();
            leap_try(LeapOpenDevice(device_ref, &mut handle))?;
        }
        Ok(handle.into())
    }

    pub(crate) fn attach(handle: LEAP_DEVICE) -> Self {
        Self {
            handle,
            drop: false,
        }
    }

    #[doc = " Gets device properties."]
    #[doc = ""]
    #[doc = " To get the device serial number, you must supply a LEAP_DEVICE_INFO struct whose"]
    #[doc = " serial member points to a char array large enough to hold the null-terminated"]
    #[doc = " serial number string. To get the required length, call LeapGetDeviceInfo() using"]
    #[doc = " a LEAP_DEVICE_INFO struct that has serial set to NULL. LeapC sets serial_length field of"]
    #[doc = " the struct to the required length. You can then allocate memory for the string,"]
    #[doc = " set the serial field, and call this function again."]
    #[doc = ""]
    #[doc = " @param hDevice A handle to the device to be queried."]
    #[doc = " @param[out] info The struct to receive the device property data."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn get_info_raw(&mut self, info: &mut LEAP_DEVICE_INFO, serial: &mut Vec<i8>) -> eLeapRS {
        info.serial_length = serial.len() as u32;
        info.serial = serial.as_mut_ptr();
        unsafe { LeapGetDeviceInfo(self.handle, info) }
    }

    #[doc = " Gets device properties."]
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

    #[doc = " Get the transform to world coordinates from 3D Leap coordinates."]
    #[doc = ""]
    #[doc = " To get the transform, you must supply an array of 16 elements."]
    #[doc = ""]
    #[doc = " The function will return a an array representing a 4 x 4 matrix of the form:"]
    #[doc = ""]
    #[doc = " R, t"]
    #[doc = " 0, 1"]
    #[doc = ""]
    #[doc = " where:"]
    #[doc = " R is a 3 x 3 rotation matrix"]
    #[doc = " t is a 3 x 1 translation vector"]
    #[doc = ""]
    #[doc = " Note that the matrix is in column major, e.g. transform[12] corresponds to the x coordinate of the"]
    #[doc = " translation vector t."]
    #[doc = ""]
    #[doc = " A possible pipeline would be, for example:"]
    #[doc = " 1) Get \"palm_pos\" the position of the center of the palm (as a 3x1 vector)"]
    #[doc = " 2) Construct a 4x1 vector using the palm_position: palm_pos_4 = (palm_pos.x; palm_pos.y; palm_pos.z; 1.0f)"]
    #[doc = " 3) Create a 4x4 matrix \"trans_mat\" as illustrated above using the returned transform"]
    #[doc = " 4) Get the position of the center of the palm in world coordinates by multiplying trans_mat and palm_pos_4:"]
    #[doc = "    center_world_4 = trans_mat * palm_pos_4"]
    #[doc = ""]
    #[doc = " This function returns eLeapRS_Unsupported in the case where this functionality is not yet supported."]
    #[doc = ""]
    #[doc = " @param hDevice A handle to the device to be queried."]
    #[doc = " @param[out] transform A pointer to a single-precision float array of size 16, containing"]
    #[doc = "  the coefficients of the 4x4 matrix in Column Major order."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.4.0"]
    #[cfg(feature = "gemini")]
    pub fn get_transform(&mut self, transform: &mut [f32; 16]) -> Result<(), Error> {
        unsafe { leap_try(LeapGetDeviceTransform(self.handle, transform.as_mut_ptr())) }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    #[cfg(feature = "gemini")]
    fn get_device_transform() {
        let mut connection = initialize_connection();
        let devices = connection
            .get_device_list()
            .expect("Failed to list devices");
        let device_info = devices.first().expect("No devices plugged for tests.");
        let mut device = device_info.open().expect("Failed to open the device");
        let mut transform = [0.0; 16];
        device.get_transform(&mut transform).unwrap();
    }
}
