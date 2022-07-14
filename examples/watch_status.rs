use leaprs::*;

fn main() {
    // BUG? No event received
    let mut connection =
        Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    loop {
        if let Ok(message) = connection.poll(1000) {
            match message.get_event() {
                Event::DeviceStatusChangeEvent(event) => {
                    println!("Device status changed.");
                    if let Some(status) = event.get_status() {
                        println!("{:?}", status);
                    }
                }
                _ => {}
            }
        }
    }
}
