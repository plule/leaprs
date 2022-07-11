use leap_sys::*;

pub enum Event<'a> {
    NoEvent,
    ConnectionEvent(&'a LEAP_CONNECTION_EVENT),
    ConnectionLostEvent(&'a LEAP_CONNECTION_LOST_EVENT),
    DeviceEvent(&'a LEAP_DEVICE_EVENT),
    DeviceStatusChangeEvent(&'a LEAP_DEVICE_STATUS_CHANGE_EVENT),
    PolicyEvent(&'a LEAP_POLICY_EVENT),
    DeviceFailureEvent(&'a LEAP_DEVICE_FAILURE_EVENT),
    TrakingEvent(&'a LEAP_TRACKING_EVENT),
    TrackingModeEvent(&'a LEAP_TRACKING_MODE_EVENT),
    LogEvent(&'a LEAP_LOG_EVENT),
    LogEvents(&'a LEAP_LOG_EVENTS),
    ConfigResponseEvent(&'a LEAP_CONFIG_RESPONSE_EVENT),
    DroppedFrameEvent(&'a LEAP_DROPPED_FRAME_EVENT),
    ImageEvent(&'a LEAP_IMAGE_EVENT),
    PointMappingChangeEvent(&'a LEAP_POINT_MAPPING_CHANGE_EVENT),
    HeadPoseEvent(&'a LEAP_HEAD_POSE_EVENT),
    EyeEvent(&'a LEAP_EYE_EVENT),
    IMUEvent(&'a LEAP_IMU_EVENT),
    LeapConfigChangeEvent(&'a LEAP_CONFIG_CHANGE_EVENT),
    ImageComplete,
    ImageRequestError,
    DeviceLost,
}

pub trait TrakingEvent {
    fn get_hands(&self) -> Vec<&LEAP_HAND>;
}

impl TrakingEvent for LEAP_TRACKING_EVENT {
    fn get_hands(&self) -> Vec<&LEAP_HAND> {
        let n_hand = self.nHands as isize;
        unsafe {
            return (0..n_hand)
                .map(|hand_index| &*self.pHands.offset(hand_index))
                .collect();
        }
    }
}
