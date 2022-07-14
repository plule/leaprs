use leap_sys::*;

pub enum Event<'a> {
    //
    None,
    //
    Connection(&'a LEAP_CONNECTION_EVENT),
    //
    ConnectionLost(&'a LEAP_CONNECTION_LOST_EVENT),
    //
    Device(&'a LEAP_DEVICE_EVENT),
    DeviceStatusChangeEvent(&'a LEAP_DEVICE_STATUS_CHANGE_EVENT),
    Policy(&'a LEAP_POLICY_EVENT),
    DeviceFailure(&'a LEAP_DEVICE_FAILURE_EVENT),
    Traking(&'a LEAP_TRACKING_EVENT),
    TrackingMode(&'a LEAP_TRACKING_MODE_EVENT),
    LogEvent(&'a LEAP_LOG_EVENT),
    LogEvents(&'a LEAP_LOG_EVENTS),
    ConfigResponse(&'a LEAP_CONFIG_RESPONSE_EVENT),
    DroppedFrame(&'a LEAP_DROPPED_FRAME_EVENT),
    Image(&'a LEAP_IMAGE_EVENT),
    PointMappingChange(&'a LEAP_POINT_MAPPING_CHANGE_EVENT),
    HeadPose(&'a LEAP_HEAD_POSE_EVENT),
    Eyes(&'a LEAP_EYE_EVENT),
    IMU(&'a LEAP_IMU_EVENT),
    ConfigChange(&'a LEAP_CONFIG_CHANGE_EVENT),
    ImageComplete,
    ImageRequestError,
    DeviceLost,
    Unknown(eLeapEventType),
}

pub trait TrakingEvent {
    fn get_hands(&self) -> Vec<&LEAP_HAND>;
}

impl TrakingEvent for LEAP_TRACKING_EVENT {
    fn get_hands(&self) -> Vec<&LEAP_HAND> {
        let n_hand = self.nHands as isize;
        unsafe {
            (0..n_hand)
                .map(|hand_index| &*self.pHands.offset(hand_index))
                .collect()
        }
    }
}
