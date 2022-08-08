use leap_sys::LEAP_IMU_EVENT;

use crate::{ImuFlag, LeapVector};

crate::leap_ref_struct!(ImuEvent, LEAP_IMU_EVENT);

impl<'a> ImuEvent<'a> {
    #[doc = " The timestamp for these measurements, in microseconds, referenced against"]
    #[doc = " LeapGetNow()."]
    #[doc = " @since 4.1.0"]
    pub fn timestamp(&self) -> i64 {
        self.handle.timestamp
    }

    #[doc = " The timestamp for these measurements, in microseconds, referenced against"]
    #[doc = " the device's internal clock."]
    #[doc = " @since 4.1.0"]
    pub fn timestamp_hw(&self) -> i64 {
        self.handle.timestamp_hw
    }

    #[doc = " A combination of eLeapIMUFlag flags."]
    #[doc = " @since 4.1.0"]
    pub fn flags(&self) -> ImuFlag {
        ImuFlag::from_bits_truncate(self.handle.flags)
    }

    #[doc = " The accelerometer measurements, in m/s^2."]
    #[doc = " @since 4.1.0"]
    pub fn accelerometer(&self) -> LeapVector {
        self.handle.accelerometer.into()
    }

    #[doc = " The gyroscope measurements, in rad/s."]
    #[doc = " @since 4.1.0"]
    pub fn gyroscope(&self) -> LeapVector {
        self.handle.gyroscope.into()
    }

    #[doc = " The measured temperature, in deg C."]
    #[doc = " @since 4.1.0"]
    pub fn temperature(&self) -> f32 {
        self.handle.temperature
    }
}
