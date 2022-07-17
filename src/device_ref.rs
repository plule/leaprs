use leap_sys::LEAP_DEVICE_REF;

use crate::Device;

crate::leap_struct!(
    #[doc = " A reference to a Leap device."]
    #[doc = ""]
    #[doc = " Get a LEAP_DEVICE_REF by calling LeapGetDeviceList(). Access a device by"]
    #[doc = " calling LeapOpenDevice() with this reference. LeapOpenDevice() provides a"]
    #[doc = " LEAP_DEVICE struct, which is a handle to an open device."]
    DeviceRef,
    LEAP_DEVICE_REF
);

impl DeviceRef {
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
    pub fn open(&self) -> Result<Device, crate::Error> {
        Device::open(self.handle)
    }

    #[doc = " a generic identifier. @since 3.0.0"]
    pub fn id(&self) -> u32 {
        self.handle.id
    }
}
