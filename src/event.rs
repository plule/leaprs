use crate::events::*;
use leap_sys::*;

#[doc = " The types of event messages resulting from calling LeapPollConnection()."]
pub enum EventRef<'a> {
    #[doc = " No event has occurred within the timeout period specified when calling LeapPollConnection()."]
    #[doc = " @since 3.0.0"]
    None,
    #[doc = " A connection to the Ultraleap Tracking Service has been established."]
    #[doc = " @since 3.0.0"]
    Connection(ConnectionEventRef<'a>),
    #[doc = " The connection to the Ultraleap Tracking Service has been lost."]
    #[doc = " @since 3.0.0"]
    ConnectionLost(ConnectionLostEventRef<'a>),
    #[doc = " A device has been detected or plugged-in."]
    #[doc = " A device event is dispatched after a connection is established for any"]
    #[doc = " devices already plugged in. (The system currently only supports one"]
    #[doc = " streaming device at a time.)"]
    #[doc = " @since 3.0.0"]
    Device(DeviceEventRef<'a>),
    #[doc = " A device has failed."]
    #[doc = " Device failure could be caused by hardware failure, USB controller issues, or"]
    #[doc = " other system instability. Note that unplugging a device generates an"]
    #[doc = " eLeapEventType_DeviceLost event message, not a failure message."]
    #[doc = " @since 3.0.0"]
    DeviceFailure(DeviceFailureEventRef<'a>),
    #[doc = " A policy change has occurred."]
    #[doc = " This can be due to setting a policy with LeapSetPolicyFlags() or due to changing"]
    #[doc = " or policy-related config settings, including images_mode."]
    #[doc = " (A user can also change these policies using the Ultraleap Tracking Control Panel.)"]
    #[doc = " @since 3.0.0"]
    Policy(PolicyEventRef<'a>),
    #[doc = " A tracking frame. The message contains the tracking data for the frame."]
    #[doc = " @since 3.0.0"]
    Tracking(TrackingEventRef<'a>),
    #[doc = " The request for an image has failed."]
    #[doc = " The message contains information about the failure. The client application"]
    #[doc = " will not receive the requested image set."]
    #[doc = " @since 3.0.0"]
    ImageRequestError, // TODO where is the message?
    #[doc = " The request for an image is complete."]
    #[doc = " The image data has been completely written to the application-provided"]
    #[doc = " buffer."]
    #[doc = " @since 3.0.0"]
    ImageComplete,
    #[doc = " A system message. @since 3.0.0"]
    LogEvent(LogEventRef<'a>),
    #[doc = " The device connection has been lost."]
    #[doc = ""]
    #[doc = " This event is generally asserted when the device has been detached from the system, when the"]
    #[doc = " connection to the service has been lost, or if the device is closed while streaming. Generally,"]
    #[doc = " any event where the system can conclude no further frames will be received will result in this"]
    #[doc = " message. The DeviceEvent field will be filled with the id of the formerly attached device."]
    #[doc = " @since 3.0.0"]
    DeviceLost,

    #[doc = " The asynchronous response to a call to LeapRequestConfigValue()."]
    #[doc = " Contains the value of requested configuration item."]
    #[doc = " @since 3.0.0"]
    ConfigResponse(ConfigResponseEventRef<'a>),
    #[doc = " The asynchronous response to a call to LeapSaveConfigValue()."]
    #[doc = " Reports whether the change succeeded or failed."]
    #[doc = " @since 3.0.0"]
    ConfigChange(ConfigChangeEventRef<'a>),
    #[doc = " Notification that a status change has been detected on an attached device"]
    #[doc = ""]
    #[doc = " @since 3.1.3"]
    DeviceStatusChange(DeviceStatusChangeEventRef<'a>),
    #[doc = " Notification that a status change has been detected on an attached device"]
    #[doc = ""]
    #[doc = " @since 3.1.3"]
    DroppedFrame(DroppedFrameEventRef<'a>),
    #[doc = " Notification that an unrequested stereo image pair is available"]
    #[doc = ""]
    #[doc = " @since 4.0.0"]
    Image(ImageEventRef<'a>),
    #[doc = " Notification that point mapping has changed"]
    #[doc = ""]
    #[doc = " @since 4.0.0"]
    PointMappingChange(PointMappingChangeEventRef<'a>),
    #[doc = " A tracking mode change has occurred."]
    #[doc = " This can be due to changing the hmd or screentop policy with LeapSetPolicyFlags()."]
    #[doc = " or setting the tracking mode using LeapSetTrackingMode()."]
    #[doc = " @since 5.0.0"]
    #[cfg(feature = "gemini")]
    TrackingMode(crate::TrackingModeEventRef<'a>),
    #[doc = " An array of system messages. @since 4.0.0"]
    LogEvents(LogEventsRef<'a>),
    #[doc = " A head pose. The message contains the timestamped head position and orientation."]
    #[doc = " @since 4.1.0"]
    HeadPose(HeadPoseEventRef<'a>),
    #[doc = " Tracked eye positions. @since 4.1.0"]
    Eyes(EyeEventRef<'a>),
    #[doc = " An IMU reading. @since 4.1.0"]
    IMU(ImuEventRef<'a>),

    Unknown(eLeapEventType),
}

impl<'a> From<(eLeapEventType, &'a _LEAP_CONNECTION_MESSAGE__bindgen_ty_1)> for EventRef<'a> {
    fn from(
        (event_type, event): (eLeapEventType, &'a _LEAP_CONNECTION_MESSAGE__bindgen_ty_1),
    ) -> Self {
        // Combine the event type and the corresponding union to get a strongly typed enum
        match event_type {
            leap_sys::_eLeapEventType_eLeapEventType_None => EventRef::None,
            leap_sys::_eLeapEventType_eLeapEventType_Connection => {
                EventRef::Connection(ConnectionEventRef(unsafe { &*event.connection_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_ConnectionLost => {
                EventRef::ConnectionLost(ConnectionLostEventRef(unsafe {
                    &*event.connection_lost_event
                }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_Device => {
                EventRef::Device(DeviceEventRef(unsafe { &*event.device_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceFailure => {
                EventRef::DeviceFailure(DeviceFailureEventRef(unsafe {
                    &*event.device_failure_event
                }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_Policy => {
                EventRef::Policy(PolicyEventRef(unsafe { &*event.policy_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_Tracking => {
                EventRef::Tracking(TrackingEventRef(unsafe { &*event.tracking_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_ImageRequestError => {
                EventRef::ImageRequestError
            }
            leap_sys::_eLeapEventType_eLeapEventType_ImageComplete => EventRef::ImageComplete,
            leap_sys::_eLeapEventType_eLeapEventType_LogEvent => {
                EventRef::LogEvent(LogEventRef(unsafe { &*event.log_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceLost => EventRef::DeviceLost,
            leap_sys::_eLeapEventType_eLeapEventType_ConfigResponse => {
                EventRef::ConfigResponse(ConfigResponseEventRef(unsafe {
                    &*event.config_response_event
                }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_ConfigChange => {
                EventRef::ConfigChange(ConfigChangeEventRef(unsafe { &*event.config_change_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceStatusChange => {
                EventRef::DeviceStatusChange(DeviceStatusChangeEventRef(unsafe {
                    &*event.device_status_change_event
                }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_DroppedFrame => {
                EventRef::DroppedFrame(DroppedFrameEventRef(unsafe { &*event.dropped_frame_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_Image => {
                EventRef::Image(ImageEventRef(unsafe { &*event.image_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_PointMappingChange => {
                EventRef::PointMappingChange(PointMappingChangeEventRef(unsafe {
                    &*event.point_mapping_change_event
                }))
            }
            #[cfg(feature = "gemini")]
            leap_sys::_eLeapEventType_eLeapEventType_TrackingMode => {
                EventRef::TrackingMode(TrackingModeEventRef(unsafe { &*event.tracking_mode_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_LogEvents => {
                EventRef::LogEvents(LogEventsRef(unsafe { &*event.log_events }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_HeadPose => {
                EventRef::HeadPose(HeadPoseEventRef(unsafe { &*event.head_pose_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_Eyes => {
                EventRef::Eyes(EyeEventRef(unsafe { &*event.eye_event }))
            }
            leap_sys::_eLeapEventType_eLeapEventType_IMU => {
                EventRef::IMU(ImuEventRef(unsafe { &*event.imu_event }))
            }
            event_code => EventRef::Unknown(event_code),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "detect-unknown-events")]
    fn detect_unknown_events() {
        use std::collections::HashSet;

        use crate::tests::*;
        use crate::*;
        let mut connection = initialize_connection();
        let unknown_events: HashSet<_> = (0..100)
            .filter_map(|_| match connection.poll(100).map(|c| c.event()) {
                Ok(EventRef::Unknown(ev)) => Some(ev),
                _ => None,
            })
            .collect();
        assert_eq!(
            unknown_events.len(),
            0,
            "Received unknown events: {:?}",
            unknown_events
        );
    }
}
