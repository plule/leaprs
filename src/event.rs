use leap_sys::*;

use crate::{
    ConnectionEvent, ConnectionLostEvent, DeviceEvent, DeviceStatusChangeEvent, ImageEvent,
    TrackingEvent, TrackingModeEvent,
};

pub enum Event<'a> {
    //
    None,
    #[doc = " \\ingroup Structs"]
    #[doc = " Received from LeapPollConnection() when a connection to the Ultraleap Tracking Service is established."]
    #[doc = " @since 3.0.0"]
    Connection(ConnectionEvent<'a>),
    #[doc = " \\ingroup Structs"]
    #[doc = " Received from LeapPollConnection() when a connection to the Ultraleap Tracking Service is lost."]
    #[doc = ""]
    #[doc = " If a LeapC function that performs a transaction with the Ultraleap Tracking Service is called"]
    #[doc = " after the connection is lost, the next call to LeapPollConnection() will return"]
    #[doc = " this event. Otherwise, it can take up to 5 seconds of polling the connection to"]
    #[doc = " receive this event."]
    #[doc = " @since 3.0.0"]
    ConnectionLost(ConnectionLostEvent<'a>),
    //
    Device(DeviceEvent<'a>),
    DeviceStatusChange(DeviceStatusChangeEvent<'a>),
    Policy(&'a LEAP_POLICY_EVENT),
    DeviceFailure(&'a LEAP_DEVICE_FAILURE_EVENT),
    Tracking(TrackingEvent<'a>),
    TrackingMode(TrackingModeEvent<'a>),
    LogEvent(&'a LEAP_LOG_EVENT),
    LogEvents(&'a LEAP_LOG_EVENTS),
    ConfigResponse(&'a LEAP_CONFIG_RESPONSE_EVENT),
    DroppedFrame(&'a LEAP_DROPPED_FRAME_EVENT),
    Image(ImageEvent<'a>),
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
                Event::Policy(unsafe { &*event.policy_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_Tracking => {
                Event::Tracking(unsafe { &*event.tracking_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_ImageRequestError => Event::ImageRequestError,
            leap_sys::_eLeapEventType_eLeapEventType_ImageComplete => Event::ImageComplete,
            leap_sys::_eLeapEventType_eLeapEventType_LogEvent => {
                Event::LogEvent(unsafe { &*event.log_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceLost => Event::DeviceLost,
            leap_sys::_eLeapEventType_eLeapEventType_ConfigResponse => {
                Event::ConfigResponse(unsafe { &*event.config_response_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_ConfigChange => {
                Event::ConfigChange(unsafe { &*event.config_change_event })
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
            leap_sys::_eLeapEventType_eLeapEventType_TrackingMode => {
                Event::TrackingMode(unsafe { &*event.tracking_mode_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_LogEvents => {
                Event::LogEvents(unsafe { &*event.log_events })
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
