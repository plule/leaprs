use leap_sys::LEAP_CONNECTION_EVENT;

use crate::ServiceState;

pub trait ConnectionEvent {
    /// Get the service state flags.
    ///
    /// None if contains unknown bits.
    fn get_flags(&self) -> Option<ServiceState>;
}

impl ConnectionEvent for LEAP_CONNECTION_EVENT {
    fn get_flags(&self) -> Option<ServiceState> {
        ServiceState::from_bits(self.flags)
    }
}
