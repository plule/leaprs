use leaprs::*;
use throbber::Throbber;

fn main() {
    let mut throbber = Throbber::new();
    throbber.start_with_msg("Connecting to the service...".to_string());
    let mut connection = Connection::create().expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    connection.wait_for(|e| match e {
        Event::ConnectionEvent(_) => true,
        _ => false,
    });

    throbber.success("Connected".to_string());

    throbber.start_with_msg("Waiting for a device...".to_string());

    connection.wait_for(|e| match e {
        Event::DeviceEvent(_) => true,
        _ => false,
    });

    throbber.success("Got a device".to_string());

    throbber.start_with_msg("Waiting for a hand...".to_string());

    connection.wait_for(|e| match e {
        Event::TrakingEvent(e) => e.get_hands().len() > 0,
        _ => false,
    });

    throbber.success("Got a hand".to_string());

    throbber.start_with_msg("Close the hand".to_string());

    connection.wait_for(|e| match e {
        Event::TrakingEvent(e) => e.get_hands().iter().any(|hand| hand.grab_strength >= 0.999),
        _ => false,
    });

    throbber.success("The hand is closed".to_string());

    throbber.start_with_msg("Open the hand".to_string());

    connection.wait_for(|e| match e {
        Event::TrakingEvent(e) => e.get_hands().iter().any(|hand| hand.grab_strength <= 0.001),
        _ => false,
    });

    throbber.success("The hand is opened".to_string());

    throbber.end();
}

trait WaitFor {
    fn wait_for<F>(&mut self, condition: F)
    where
        F: Fn(&Event) -> bool;
}

impl WaitFor for Connection {
    fn wait_for<F>(&mut self, condition: F)
    where
        F: Fn(&Event) -> bool,
    {
        loop {
            if let Ok(message) = self.poll(100) {
                if condition(&message.event()) {
                    break;
                }
            }
        }
    }
}
