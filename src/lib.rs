#![allow(rustdoc::broken_intra_doc_links)] // The doc is mostly c/c from the origin, the links need repair.
#![doc = include_str!("../README.md")]
#![deny(clippy::all)]
mod bone;
mod capabilities;
mod clock_rebaser;
mod connection;
mod connection_config;
mod connection_info;
mod connection_message;
mod connection_status;
mod device;
mod device_info;
mod device_pid;
mod device_ref;
mod device_status;
mod digit;
mod distortion_matrix;
mod dropped_frame_type;
mod event;
mod events;
mod frame_header;
mod hand;
mod image;
mod image_format;
mod image_properties;
mod image_type;
mod imu_flag;
mod leap_rs;
mod leap_vector;
mod log_severity;
mod palm;
mod perspective_type;
mod point_mapping;
mod policy_flag;
mod quaternion;
mod service_state;
mod sized_with_trailing_data;
#[cfg(feature = "gemini")]
mod tracking_mode;
mod variant;
#[cfg(feature = "gemini")]
mod version;
mod version_part;
pub use crate::image::*;
pub use bone::*;
pub use capabilities::*;
pub use clock_rebaser::*;
pub use connection::*;
pub use connection_config::*;
pub use connection_info::*;
pub use connection_message::*;
pub use connection_status::*;
pub use device::*;
pub use device_info::*;
pub use device_pid::*;
pub use device_ref::*;
pub use device_status::*;
pub use digit::*;
pub use distortion_matrix::*;
pub use dropped_frame_type::*;
pub use event::*;
pub use events::*;
pub use frame_header::*;
pub use hand::*;
pub use image_format::*;
pub use image_properties::*;
pub use image_type::*;
pub use imu_flag::*;
pub use leap_rs::*;
use leap_sys::LeapGetNow;
pub use leap_vector::*;
pub use log_severity::*;
pub use palm::*;
pub use perspective_type::*;
pub use point_mapping::*;
pub use policy_flag::*;
pub use quaternion::*;
pub use service_state::*;
#[cfg(feature = "gemini")]
pub use tracking_mode::*;
pub use variant::*;
#[cfg(feature = "gemini")]
pub use version::*;
pub use version_part::*;

/// Reexport the leap_sys crate
mod leap_sys {
    pub use ::leap_sys::*;
}

pub fn leap_get_now() -> i64 {
    unsafe { LeapGetNow() }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn test_leap_get_now() {
        assert!(leap_get_now() > 0)
    }

    pub fn initialize_connection_config(config: ConnectionConfig) -> Connection {
        let mut connection = Connection::create(config).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");

        connection
            .wait_for(|e| match e {
                EventRef::Connection(_) => Some(()),
                _ => None,
            })
            .expect("Did not receive connection message");

        connection
            .wait_for(|e| match e {
                EventRef::Device(_) => Some(()),
                _ => None,
            })
            .expect("Did not receive device connection");

        connection
    }

    #[cfg(feature = "gemini")]
    pub fn initialize_connection_ex() -> (Connection, Device) {
        let config = ConnectionConfig::new(ConnectionConfigFlags::MULTI_DEVICE_AWARE, None);
        let mut connection = initialize_connection_config(config);
        let first_device = connection
            .get_device_list()
            .unwrap()
            .first()
            .unwrap()
            .open()
            .unwrap();
        connection.subscribe_events(&first_device).unwrap();
        (connection, first_device)
    }

    /// Connect to the service and wait for the first events necessary for LeapC to be functional
    pub fn initialize_connection() -> Connection {
        initialize_connection_config(ConnectionConfig::default())
    }

    pub trait ConnectionTestExtensions {
        fn wait_for<F, T>(&mut self, condition: F) -> Result<T, &str>
        where
            F: Fn(&EventRef) -> Option<T>;
    }

    impl ConnectionTestExtensions for Connection {
        fn wait_for<F, T>(&mut self, condition: F) -> Result<T, &str>
        where
            F: Fn(&EventRef) -> Option<T>,
        {
            for _ in 0..200 {
                if let Ok(event_message) = self.poll(100) {
                    if let Some(ret) = condition(&event_message.event()) {
                        return Ok(ret);
                    }
                }
            }
            Err("Did not receive the event")
        }
    }
}
