use bitflags::bitflags;
use leap_sys::{
    _eLeapIMUFlag_eLeapIMUFlag_HasAccelerometer, _eLeapIMUFlag_eLeapIMUFlag_HasGyroscope,
    _eLeapIMUFlag_eLeapIMUFlag_HasTemperature,
};

bitflags! {
    pub struct ImuFlag: u32 {
        #[doc = " Has accelerometer measurements. @since 4.1.0"]
        const HAS_ACCELEROMETER = _eLeapIMUFlag_eLeapIMUFlag_HasAccelerometer as u32;
        #[doc = " Has gyroscope measurements. @since 4.1.0"]
        const HAS_GYROSCOPE = _eLeapIMUFlag_eLeapIMUFlag_HasGyroscope as u32;
        #[doc = " Has a temperature measurement. @since 4.1.0"]
        const HAS_TEMPERATURE = _eLeapIMUFlag_eLeapIMUFlag_HasTemperature as u32;
    }
}
