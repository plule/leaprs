use leap_sys::LEAP_CONNECTION_MESSAGE;

use crate::{event::Event, EventType};

pub struct ConnectionMessage {
    handle: LEAP_CONNECTION_MESSAGE,
}

impl From<LEAP_CONNECTION_MESSAGE> for ConnectionMessage {
    fn from(handle: LEAP_CONNECTION_MESSAGE) -> Self {
        Self { handle }
    }
}

impl ConnectionMessage {
    pub fn event(&self) -> Event {
        unsafe {
            return match self.handle.type_.into() {
                EventType::None => Event::NoEvent,
                EventType::Connection => {
                    Event::ConnectionEvent(&*self.handle.__bindgen_anon_1.connection_event)
                }
                EventType::ConnectionLost => {
                    Event::ConnectionLostEvent(&*self.handle.__bindgen_anon_1.connection_lost_event)
                }
                EventType::Device => {
                    Event::DeviceEvent(&*self.handle.__bindgen_anon_1.device_event)
                }
                EventType::DeviceFailure => {
                    Event::DeviceFailureEvent(&*self.handle.__bindgen_anon_1.device_failure_event)
                }
                EventType::Policy => {
                    Event::PolicyEvent(&*self.handle.__bindgen_anon_1.policy_event)
                }
                EventType::Tracking => {
                    Event::TrakingEvent(&*self.handle.__bindgen_anon_1.tracking_event)
                }
                EventType::ImageRequestError => Event::ImageRequestError,
                EventType::ImageComplete => Event::ImageComplete,
                EventType::LogEvent => Event::LogEvent(&*self.handle.__bindgen_anon_1.log_event),
                EventType::DeviceLost => Event::DeviceLost,
                EventType::ConfigResponse => {
                    Event::ConfigResponseEvent(&*self.handle.__bindgen_anon_1.config_response_event)
                }
                EventType::ConfigChange => {
                    Event::LeapConfigChangeEvent(&*self.handle.__bindgen_anon_1.config_change_event)
                }
                EventType::DeviceStatusChange => Event::DeviceStatusChangeEvent(
                    &*self.handle.__bindgen_anon_1.device_status_change_event,
                ),
                EventType::DroppedFrame => {
                    Event::DroppedFrameEvent(&*self.handle.__bindgen_anon_1.dropped_frame_event)
                }
                EventType::Image => Event::ImageEvent(&*self.handle.__bindgen_anon_1.image_event),
                EventType::PointMappingChange => Event::PointMappingChangeEvent(
                    &*self.handle.__bindgen_anon_1.point_mapping_change_event,
                ),
                EventType::TrackingMode => {
                    Event::TrackingModeEvent(&*self.handle.__bindgen_anon_1.tracking_mode_event)
                }
                EventType::LogEvents => Event::LogEvents(&*self.handle.__bindgen_anon_1.log_events),
                EventType::HeadPose => {
                    Event::HeadPoseEvent(&*self.handle.__bindgen_anon_1.head_pose_event)
                }
                EventType::Eyes => Event::EyeEvent(&*self.handle.__bindgen_anon_1.eye_event),
                EventType::IMU => Event::IMUEvent(&*self.handle.__bindgen_anon_1.imu_event),
                EventType::NotAnEvent => todo!(),
            };
        };
    }
}
