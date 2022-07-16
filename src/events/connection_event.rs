use leap_sys::LEAP_CONNECTION_EVENT;

use crate::ServiceState;

crate::leap_ref_struct!(ConnectionEvent, LEAP_CONNECTION_EVENT);

impl<'a> ConnectionEvent<'a> {
    pub fn flags(&self) -> Option<ServiceState> {
        ServiceState::from_bits(self.handle.flags)
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
                Event::Connection(e) => Some(e.flags()),
                _ => None,
            },
        );
        flags.expect("Connection event with unknown flags.");
    }
}
