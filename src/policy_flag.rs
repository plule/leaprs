use bitflags::bitflags;
use leap_sys::*;

bitflags! {
    #[doc = " Enumerates flags for the service policies."]
    pub struct PolicyFlags: u64 {
        #[doc = " The policy allowing an application to receive frames in the background. @since 3.0.0"]
        const BACKGROUND_FRAMES = _eLeapPolicyFlag_eLeapPolicyFlag_BackgroundFrames as u64;
        #[doc = " The policy specifying whether to automatically stream images from the device. @since 4.0.0"]
        const IMAGES = _eLeapPolicyFlag_eLeapPolicyFlag_Images as u64;
        #[doc = " The policy specifying whether to optimize tracking for head-mounted device. @since 3.0.0"]
        const OPTIMIZE_HMD = _eLeapPolicyFlag_eLeapPolicyFlag_OptimizeHMD as u64;
        #[doc = " The policy allowing an application to pause or resume service tracking. @since 3.0.0"]
        const ALLOW_PAUSE_RESUME = _eLeapPolicyFlag_eLeapPolicyFlag_AllowPauseResume as u64;
        #[doc = " The policy allowing an application to receive per-frame map points. @since 4.0.0"]
        const MAP_POINTS = _eLeapPolicyFlag_eLeapPolicyFlag_MapPoints as u64;
        #[doc = " The policy specifying whether to optimize tracking for screen-top device. @since 5.0.0"]
        const OPTIMIZE_SCREEN_TOP = _eLeapPolicyFlag_eLeapPolicyFlag_OptimizeScreenTop as u64;
    }
}
