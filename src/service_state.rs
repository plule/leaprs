use bitflags::bitflags;
use leap_sys::*;

bitflags! {
    pub struct ServiceState: u32 {
        #[doc = " The service cannot receive frames fast enough from the underlying hardware."]
        #[doc = " @since 3.1.3"]
        const LOW_FPS_DETECTED = _eLeapServiceDisposition_eLeapServiceState_LowFpsDetected as u32;
        #[doc = " The service has paused itself due to an insufficient frame rate from the hardware."]
        #[doc = " @since 3.1.3"]
        const POOR_PERFORMANCE_PAUSE = _eLeapServiceDisposition_eLeapServiceState_PoorPerformancePause as u32;
        #[doc = " The service has failed to start tracking due to unknown reasons."]
        #[doc = " @since 5.1.16"]
        const TRACKING_ERROR_UNKNOWN = _eLeapServiceDisposition_eLeapServiceState_TrackingErrorUnknown as u32;
    }
}
