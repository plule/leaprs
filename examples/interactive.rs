use std::time::Duration;

use leaprs::*;
use throbber::Throbber;

fn main() {
    let mut connection =
        Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    connection.wait_for("Connecting to the service...".to_string(), |e| match e {
        Event::Connection(e) => {
            let flags = e.get_flags().expect("Invalid service state flags");
            Msg::Success(format!("Connected. Service state: {:?}", flags))
        }
        _ => Msg::None,
    });

    connection.wait_for("Waiting for a device...".to_string(), |e| match e {
        Event::Device(e) => {
            let serial = e
                .device
                .open()
                .expect("Failed to open the device")
                .get_info()
                .expect("Failed to get device info")
                .get_serial_string()
                .expect("Failed to get the device serial");

            Msg::Success(format!("Got the device {serial}"))
        }
        _ => Msg::None,
    });

    connection.wait_for("Waiting for a hand...".to_string(), |e| match e {
        Event::Traking(e) => {
            if e.get_hands().len() > 0 {
                Msg::Success("Got a hand".to_string())
            } else {
                Msg::None
            }
        }
        _ => Msg::None,
    });

    connection.wait_for("Close the hand".to_string(), |e| match e {
        Event::Traking(e) => {
            if let Some(hand) = e.get_hands().first() {
                let grab_strength = hand.grab_strength;
                if grab_strength >= 1.0 {
                    Msg::Success("The hand is closed".to_string())
                } else {
                    Msg::Progress(format!("Close the hand {:.0}%", grab_strength * 100.0))
                }
            } else {
                Msg::Progress("Close the hand".to_string())
            }
        }
        _ => Msg::None,
    });

    connection.wait_for("Open the hand".to_string(), |e| match e {
        Event::Traking(e) => {
            if let Some(hand) = e.get_hands().first() {
                let ungrab_strength = 1.0 - hand.grab_strength;
                if ungrab_strength >= 0.999 {
                    Msg::Success("The hand is opened".to_string())
                } else {
                    Msg::Progress(format!("Open the hand {:.0}%", ungrab_strength * 100.0))
                }
            } else {
                Msg::Progress("Open the hand".to_string())
            }
        }
        _ => Msg::None,
    });

    connection
        .set_policy_flags(PolicyFlags::IMAGES, PolicyFlags::empty())
        .expect("Failed to set policy flags");

    connection.wait_for("Reading image".to_string(), |e| match e {
        Event::Image(e) => {
            let w = e.image[0].properties.width;
            let h = e.image[0].properties.height;
            let image_data = e.image[0].get_data();
            image::save_buffer("image.png", image_data, w, h, image::ColorType::L8)
                .expect("failed to save buffer");
            Msg::Success(format!("Saved image.png"))
        }
        _ => Msg::None,
    });

    connection
        .set_policy_flags(PolicyFlags::empty(), PolicyFlags::IMAGES)
        .expect("Failed to set policy flags");
}

pub enum Msg {
    None,
    Success(String),
    Progress(String),
    Failure(String),
}

trait WaitFor {
    fn wait_for<F>(&mut self, message: String, condition: F)
    where
        F: Fn(&Event) -> Msg;
}

impl WaitFor for Connection {
    fn wait_for<F>(&mut self, message: String, condition: F)
    where
        F: Fn(&Event) -> Msg,
    {
        let mut throbber = Throbber::new().interval(Duration::from_millis(100));

        throbber.start_with_msg(message);

        loop {
            if let Ok(message) = self.poll(100) {
                match condition(&message.get_event()) {
                    Msg::None => {}
                    Msg::Success(message) => {
                        throbber.success(message);
                        break;
                    }
                    Msg::Progress(message) => {
                        throbber.change_message(message);
                    }
                    Msg::Failure(message) => {
                        throbber.fail(message);
                        break;
                    }
                }
            }
        }
        throbber.end();
    }
}
