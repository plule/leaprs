mod connection_event;
mod device_event;
mod device_status_change_event;
mod tracking_event;
mod tracking_mode_event;
pub use connection_event::*;
pub use device_event::*;
pub use device_status_change_event::*;
pub use tracking_event::*;
pub use tracking_mode_event::*;

macro_rules! leap_event_struct {
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
pub(crate) use leap_event_struct;
