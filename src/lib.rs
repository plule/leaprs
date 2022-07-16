mod connection;
mod connection_config;
mod connection_message;
mod device;
mod device_info;
mod device_pid;
mod device_ref;
mod device_status;
mod digit;
mod event;
mod events;
mod hand;
mod image;
mod leap_rs;
mod leap_vector;
mod policy_flag;
mod quaternion;
mod service_state;
mod tracking_mode;
mod version;
mod version_part;
pub use crate::image::*;
pub use connection::*;
pub use connection_config::*;
pub use connection_message::*;
pub use device::*;
pub use device_info::*;
pub use device_pid::*;
pub use device_ref::*;
pub use device_status::*;
pub use digit::*;
pub use event::*;
pub use events::*;
pub use hand::*;
pub use leap_rs::*;
pub use leap_vector::*;
pub use policy_flag::*;
pub use quaternion::*;
pub use service_state::*;
pub use tracking_mode::*;
pub use version::*;
pub use version_part::*;

/// Declare a leap struct wrapper that owns it.
macro_rules! leap_struct {
    ($struct_name:ident, $sys_name:ident) => {
        pub struct $struct_name {
            pub handle: $sys_name,
        }

        impl From<$sys_name> for $struct_name {
            fn from(handle: $sys_name) -> Self {
                Self { handle }
            }
        }
    };
}

pub(crate) use leap_struct;

/// Declare a leap struct wrapper that does not own it.
macro_rules! leap_ref_struct {
    ($struct_name:ident, $sys_name:ident) => {
        pub struct $struct_name<'a> {
            pub handle: &'a $sys_name,
        }

        impl<'a> From<&'a $sys_name> for $struct_name<'a> {
            fn from(handle: &'a $sys_name) -> Self {
                Self { handle }
            }
        }
    };
}

pub(crate) use leap_ref_struct;

#[cfg(test)]
mod tests {
    use crate::*;

    /// Connect to the service and wait for the first events necessary for LeapC to be functional
    pub fn initialize_connection() -> Connection {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");

        connection.expect_event(
            "Did not receive connection message".to_string(),
            |e| match e {
                Event::Connection(_) => Some(()),
                _ => None,
            },
        );

        connection.expect_event(
            "Did not receive device connection".to_string(),
            |e| match e {
                Event::Device(_) => Some(()),
                _ => None,
            },
        );

        connection
    }

    pub trait ConnectionTestExtensions {
        fn expect_event<F, T>(&mut self, message: String, condition: F) -> T
        where
            F: Fn(&Event) -> Option<T>;
    }

    impl ConnectionTestExtensions for Connection {
        fn expect_event<F, T>(&mut self, message: String, condition: F) -> T
        where
            F: Fn(&Event) -> Option<T>,
        {
            for _ in 0..10 {
                if let Ok(event_message) = self.poll(100) {
                    if let Some(ret) = condition(&event_message.get_event()) {
                        return ret;
                    }
                }
            }
            panic!("{}", message);
        }
    }
}
