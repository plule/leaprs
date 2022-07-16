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

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    fn connection_event_test() {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");
        let flags = connection.expect_event(
            "Did not receive a connection event.".to_string(),
            |e| match e {
                Event::Connection(e) => Some(e.get_flags()),
                _ => None,
            },
        );
        flags.expect("Connection event with unknown flags.");
    }
}
