use leap_sys::*;

use bitflags::bitflags;

bitflags! {
    #[doc = " Enumerates the device status codes."]
    #[doc = " @since 3.0.0"]
    pub struct DeviceStatus: u32 {
        #[doc = " The device is sending out frames. @since 3.0.0"]
        const STREAMING = _eLeapDeviceStatus_eLeapDeviceStatus_Streaming as u32;
        #[doc = " Device streaming has been paused. @since 3.0.0"]
        const PAUSED = _eLeapDeviceStatus_eLeapDeviceStatus_Paused as u32;
        #[doc = " There are known sources of infrared interference. Device has transitioned to"]
        #[doc = " robust mode in order to compensate.  @since 3.1.3"]
        const ROBUST = _eLeapDeviceStatus_eLeapDeviceStatus_Robust as u32;
        #[doc = " The device's window is smudged, tracking may be degraded.  @since 3.1.3"]
        const SMUDGED = _eLeapDeviceStatus_eLeapDeviceStatus_Smudged as u32;
        #[doc = " The device has entered low-resource mode. @since 4.0.0"]
        const LOW_RESOURCE = _eLeapDeviceStatus_eLeapDeviceStatus_LowResource as u32;
        #[doc = " The device has failed, but the failure reason is not known. @since 3.0.0"]
        const UNKNOWN_FAILURE = _eLeapDeviceStatus_eLeapDeviceStatus_UnknownFailure as u32;
        #[doc = " The device has a bad calibration record and cannot send frames. @since 3.0.0"]
        const BAD_CALIBRATION = _eLeapDeviceStatus_eLeapDeviceStatus_BadCalibration as u32;
        #[doc = " The device reports corrupt firmware or cannot install a required firmware update. @since 3.0.0"]
        const BAD_FIRMWARE = _eLeapDeviceStatus_eLeapDeviceStatus_BadFirmware as u32;
        #[doc = " The device USB connection is faulty. @since 3.0.0"]
        const BAD_TRANSPORT = _eLeapDeviceStatus_eLeapDeviceStatus_BadTransport as u32;
        #[doc = " The device USB control interfaces failed to initialize. @since 3.0.0"]
        const BAD_CONTROL = _eLeapDeviceStatus_eLeapDeviceStatus_BadControl as u32;
    }
}
