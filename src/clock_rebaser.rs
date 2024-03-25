use derive_deref::Deref;
use leap_sys::{
    LeapCreateClockRebaser, LeapDestroyClockRebaser, LeapRebaseClock, LeapUpdateRebase,
    LEAP_CLOCK_REBASER,
};

use crate::{leap_try, Error};

#[doc = " \\ingroup Structs"]
#[doc = " \\struct LEAP_CLOCK_REBASER"]
#[doc = " An opaque clock rebase state structure. @since 3.1.2"]
/// # Fields
/// Available via dereference: [LEAP_CLOCK_REBASER].
#[derive(Deref)]
pub struct ClockRebaser(pub(crate) LEAP_CLOCK_REBASER);

impl Drop for ClockRebaser {
    fn drop(&mut self) {
        unsafe {
            LeapDestroyClockRebaser(self.0);
        }
    }
}

impl ClockRebaser {
    #[doc = " Initializes a new Leap clock-rebaser handle object."]
    #[doc = ""]
    #[doc = " Pass the filled-in LEAP_CLOCK_REBASER object to calls to LeapUpdateRebase(),"]
    #[doc = " LeapRebaseClock(), and LeapDestroyClockRebaser()."]
    #[doc = ""]
    #[doc = " @param[out] phClockRebaser The pointer to the clock-rebaser object to be initialized."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.1.2"]
    pub fn create() -> Result<Self, Error> {
        unsafe {
            let mut handle: LEAP_CLOCK_REBASER = std::mem::zeroed();
            leap_try(LeapCreateClockRebaser(&mut handle))?;
            Ok(Self(handle))
        }
    }

    #[doc = " Updates the relationship between the Ultraleap Tracking Service clock and the user clock."]
    #[doc = ""]
    #[doc = " When using LeapInterpolateFrame(), call this function for every graphics frame"]
    #[doc = " rendered by your application. The function should be called as close to the"]
    #[doc = " actual point of rendering as possible."]
    #[doc = ""]
    #[doc = " The relationship between the application clock and the Ultraleap Tracking Service clock is"]
    #[doc = " neither fixed nor stable. Simulation restarts can cause user clock values to change"]
    #[doc = " instantaneously. Certain systems simulate slow motion, or respond to heavy load, by reducing the tick rate"]
    #[doc = " of the user clock. As a result, the LeapUpdateRebase() function must be called for every rendered frame."]
    #[doc = ""]
    #[doc = " @param hClockRebaser The handle to a rebaser object created by LeapCreateClockRebaser()."]
    #[doc = " @param userClock A clock value supplied by the application, sampled at about the same time as LeapGetNow() was sampled."]
    #[doc = " @param leapClock The Ultraleap Tracking Service clock value sampled by a call to LeapGetNow()."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.1.2"]
    pub fn update_rebase(&mut self, user_clock: i64, leap_clock: i64) -> Result<(), Error> {
        unsafe {
            leap_try(LeapUpdateRebase(self.0, user_clock, leap_clock))?;
        }
        Ok(())
    }

    #[doc = " Computes the Ultraleap Tracking Service clock corresponding to a specified application clock value."]
    #[doc = ""]
    #[doc = " Use this function to translate your application clock to the Ultraleap Tracking Service clock"]
    #[doc = " when interpolating frames. LeapUpdateRebase() must be called for every rendered"]
    #[doc = " frame for the relationship between the two clocks to remain synchronized."]
    #[doc = ""]
    #[doc = " @param hClockRebaser The handle to a rebaser object created by LeapCreateClockRebaser()."]
    #[doc = " @param userClock The clock in microseconds referenced to the application clock."]
    #[doc = " @param[out] pLeapClock The corresponding Ultraleap Tracking Service clock value."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.1.2"]
    pub fn rebase_clock(&mut self, user_clock: i64) -> Result<i64, Error> {
        let mut leap_clock: i64 = 0;
        unsafe {
            leap_try(LeapRebaseClock(self.0, user_clock, &mut leap_clock))?;
        }
        Ok(leap_clock)
    }
}
