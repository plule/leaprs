use derive_deref::Deref;
use leap_sys::LEAP_IMU_EVENT;

use crate::{ImuFlag, LeapVector};

#[derive(Deref)]
pub struct ImuEvent<'a>(pub(crate) &'a LEAP_IMU_EVENT);

impl<'a> ImuEvent<'a> {
    #[doc = " A combination of eLeapIMUFlag flags."]
    #[doc = " @since 4.1.0"]
    pub fn flags(&self) -> ImuFlag {
        ImuFlag::from_bits_truncate(self.flags)
    }

    #[doc = " The accelerometer measurements, in m/s^2."]
    #[doc = " @since 4.1.0"]
    pub fn accelerometer(&self) -> LeapVector {
        LeapVector(self.accelerometer)
    }

    #[doc = " The gyroscope measurements, in rad/s."]
    #[doc = " @since 4.1.0"]
    pub fn gyroscope(&self) -> LeapVector {
        LeapVector(self.gyroscope)
    }
}
