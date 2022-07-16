use leap_sys::LEAP_CONNECTION_MESSAGE;

use crate::event::Event;

crate::leap_struct!(ConnectionMessage, LEAP_CONNECTION_MESSAGE);

impl ConnectionMessage {
    pub fn get_event(&self) -> Event {
        // Combine the event type and the corresponding union to get a strongly typed enum
        match self.handle.type_ {
            leap_sys::_eLeapEventType_eLeapEventType_None => Event::None,
            leap_sys::_eLeapEventType_eLeapEventType_Connection => {
                Event::Connection(unsafe { &*self.handle.__bindgen_anon_1.connection_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_ConnectionLost => {
                Event::ConnectionLost(unsafe {
                    &*self.handle.__bindgen_anon_1.connection_lost_event
                })
            }
            leap_sys::_eLeapEventType_eLeapEventType_Device => {
                Event::Device(unsafe { &*self.handle.__bindgen_anon_1.device_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceFailure => {
                Event::DeviceFailure(unsafe { &*self.handle.__bindgen_anon_1.device_failure_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_Policy => {
                Event::Policy(unsafe { &*self.handle.__bindgen_anon_1.policy_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_Tracking => {
                Event::Tracking(unsafe { &*self.handle.__bindgen_anon_1.tracking_event }.into())
            }
            leap_sys::_eLeapEventType_eLeapEventType_ImageRequestError => Event::ImageRequestError,
            leap_sys::_eLeapEventType_eLeapEventType_ImageComplete => Event::ImageComplete,
            leap_sys::_eLeapEventType_eLeapEventType_LogEvent => {
                Event::LogEvent(unsafe { &*self.handle.__bindgen_anon_1.log_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceLost => Event::DeviceLost,
            leap_sys::_eLeapEventType_eLeapEventType_ConfigResponse => {
                Event::ConfigResponse(unsafe {
                    &*self.handle.__bindgen_anon_1.config_response_event
                })
            }
            leap_sys::_eLeapEventType_eLeapEventType_ConfigChange => {
                Event::ConfigChange(unsafe { &*self.handle.__bindgen_anon_1.config_change_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_DeviceStatusChange => {
                Event::DeviceStatusChange(unsafe {
                    &*self.handle.__bindgen_anon_1.device_status_change_event
                })
            }
            leap_sys::_eLeapEventType_eLeapEventType_DroppedFrame => {
                Event::DroppedFrame(unsafe { &*self.handle.__bindgen_anon_1.dropped_frame_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_Image => {
                Event::Image(unsafe { &*self.handle.__bindgen_anon_1.image_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_PointMappingChange => {
                Event::PointMappingChange(unsafe {
                    &*self.handle.__bindgen_anon_1.point_mapping_change_event
                })
            }
            leap_sys::_eLeapEventType_eLeapEventType_TrackingMode => {
                Event::TrackingMode(unsafe { &*self.handle.__bindgen_anon_1.tracking_mode_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_LogEvents => {
                Event::LogEvents(unsafe { &*self.handle.__bindgen_anon_1.log_events })
            }
            leap_sys::_eLeapEventType_eLeapEventType_HeadPose => {
                Event::HeadPose(unsafe { &*self.handle.__bindgen_anon_1.head_pose_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_Eyes => {
                Event::Eyes(unsafe { &*self.handle.__bindgen_anon_1.eye_event })
            }
            leap_sys::_eLeapEventType_eLeapEventType_IMU => {
                Event::IMU(unsafe { &*self.handle.__bindgen_anon_1.imu_event })
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
            let msg = connection.poll(1000).expect("Failed to poll");
            if let Event::Unknown(_) = msg.get_event() {
                panic!("Received an unknown event");
            }
        }
    }
}
