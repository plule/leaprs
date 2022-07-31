use leap_sys::*;

use crate::{
    ConfigChangeEvent, ConfigResponseEvent, ConnectionEvent, ConnectionLostEvent, DeviceEvent,
    DeviceStatusChangeEvent, ImageEvent, LogEvent, LogEvents, PolicyEvent, TrackingEvent,
};

#[doc = " The types of event messages resulting from calling LeapPollConnection()."]
pub enum Event<'a> {
    #[doc = " No event has occurred within the timeout period specified when calling LeapPollConnection()."]
    #[doc = " @since 3.0.0"]
    None,
    #[doc = " A connection to the Ultraleap Tracking Service has been established."]
    #[doc = " @since 3.0.0"]
    Connection(ConnectionEvent<'a>),
    #[doc = " The connection to the Ultraleap Tracking Service has been lost."]
    #[doc = " @since 3.0.0"]
    ConnectionLost(ConnectionLostEvent<'a>),
    #[doc = " A device has been detected or plugged-in."]
    #[doc = " A device event is dispatched after a connection is established for any"]
    #[doc = " devices already plugged in. (The system currently only supports one"]
    #[doc = " streaming device at a time.)"]
    #[doc = " @since 3.0.0"]
    Device(DeviceEvent<'a>),
    #[doc = " A device has failed."]
    #[doc = " Device failure could be caused by hardware failure, USB controller issues, or"]
    #[doc = " other system instability. Note that unplugging a device generates an"]
    #[doc = " eLeapEventType_DeviceLost event message, not a failure message."]
    #[doc = " @since 3.0.0"]
    DeviceFailure(&'a LEAP_DEVICE_FAILURE_EVENT),
    #[doc = " A policy change has occurred."]
    #[doc = " This can be due to setting a policy with LeapSetPolicyFlags() or due to changing"]
    #[doc = " or policy-related config settings, including images_mode."]
    #[doc = " (A user can also change these policies using the Ultraleap Tracking Control Panel.)"]
    #[doc = " @since 3.0.0"]
    Policy(PolicyEvent<'a>),
    #[doc = " A tracking frame. The message contains the tracking data for the frame."]
    #[doc = " @since 3.0.0"]
    Tracking(TrackingEvent<'a>),
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
    LogEvent(LogEvent<'a>),
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
    ConfigResponse(ConfigResponseEvent<'a>),
    #[doc = " The asynchronous response to a call to LeapSaveConfigValue()."]
    #[doc = " Reports whether the change succeeded or failed."]
    #[doc = " @since 3.0.0"]
    ConfigChange(ConfigChangeEvent<'a>),
    #[doc = " Notification that a status change has been detected on an attached device"]
    #[doc = ""]
    #[doc = " @since 3.1.3"]
    DeviceStatusChange(DeviceStatusChangeEvent<'a>),
    #[doc = " Notification that a status change has been detected on an attached device"]
    #[doc = ""]
    #[doc = " @since 3.1.3"]
    DroppedFrame(&'a LEAP_DROPPED_FRAME_EVENT),
    #[doc = " Notification that an unrequested stereo image pair is available"]
    #[doc = ""]
    #[doc = " @since 4.0.0"]
    Image(ImageEvent<'a>),
    #[doc = " Notification that point mapping has changed"]
    #[doc = ""]
    #[doc = " @since 4.0.0"]
    PointMappingChange(&'a LEAP_POINT_MAPPING_CHANGE_EVENT),
    #[doc = " A tracking mode change has occurred."]
    #[doc = " This can be due to changing the hmd or screentop policy with LeapSetPolicyFlags()."]
    #[doc = " or setting the tracking mode using LeapSetTrackingMode()."]
    #[doc = " @since 5.0.0"]
    #[cfg(feature = "gemini")]
    TrackingMode(crate::TrackingModeEvent<'a>),
    #[doc = " An array of system messages. @since 4.0.0"]
    LogEvents(LogEvents<'a>),
    #[doc = " A head pose. The message contains the timestamped head position and orientation."]
    #[doc = " @since 4.1.0"]
    HeadPose(&'a LEAP_HEAD_POSE_EVENT),
    #[doc = " Tracked eye positions. @since 4.1.0"]
    Eyes(&'a LEAP_EYE_EVENT),
    #[doc = " An IMU reading. @since 4.1.0"]
    IMU(&'a LEAP_IMU_EVENT),

    Unknown(eLeapEventType),
}

impl<'a> From<(eLeapEventType, &'a _LEAP_CONNECTION_MESSAGE__bindgen_ty_1)> for Event<'a> {
    fn from(
        (event_type, event): (eLeapEventType, &'a _LEAP_CONNECTION_MESSAGE__bindgen_ty_1),
    ) -> Self {
        // Combine the event type and the corresponding union to get a strongly typed enum
        match event_type {
            leap_sys::_eLeapEventType_eLeapEventType_None => Event::None,
            leap_sys::_eLeapEventType_eLeapEventType_Connection => {
                Event::Connection(unsafe { &*event.connection_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_ConnectionLost => {
                Event::ConnectionLost(unsafe { &*event.connection_lost_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_Device => {
                Event::Device(unsafe { &*event.device_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceFailure => {
                Event::DeviceFailure(unsafe { &*event.device_failure_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_Policy => {
                Event::Policy(unsafe { &*event.policy_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_Tracking => {
                Event::Tracking(unsafe { &*event.tracking_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_ImageRequestError => Event::ImageRequestError,
            leap_sys::_eLeapEventType_eLeapEventType_ImageComplete => Event::ImageComplete,
            leap_sys::_eLeapEventType_eLeapEventType_LogEvent => {
                Event::LogEvent(unsafe { &*event.log_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceLost => Event::DeviceLost,
            leap_sys::_eLeapEventType_eLeapEventType_ConfigResponse => {
                Event::ConfigResponse(unsafe { &*event.config_response_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_ConfigChange => {
                Event::ConfigChange(unsafe { &*event.config_change_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceStatusChange => {
                Event::DeviceStatusChange(unsafe { &*event.device_status_change_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_DroppedFrame => {
                Event::DroppedFrame(unsafe { &*event.dropped_frame_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_Image => {
                Event::Image(unsafe { &*event.image_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_PointMappingChange => {
                Event::PointMappingChange(unsafe { &*event.point_mapping_change_event })
            }
            #[cfg(feature = "gemini")]
            leap_sys::_eLeapEventType_eLeapEventType_TrackingMode => {
                Event::TrackingMode(unsafe { &*event.tracking_mode_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_LogEvents => {
                Event::LogEvents(unsafe { &*event.log_events }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_HeadPose => {
                Event::HeadPose(unsafe { &*event.head_pose_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_Eyes => {
                Event::Eyes(unsafe { &*event.eye_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_IMU => {
                Event::IMU(unsafe { &*event.imu_event })
            }
            event_code => Event::Unknown(event_code),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    fn detect_unknown_events() {
        let mut connection = initialize_connection();
        for _ in 0..100 {
            let msg = connection.poll(5000).expect("Failed to poll");
            if let Event::Unknown(_) = msg.event() {
                panic!("Received an unknown event");
            }
        }
    }
}
