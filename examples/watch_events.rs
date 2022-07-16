use leaprs::*;

fn main() {
    // BUG? No event received
    let mut connection =
        Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    loop {
        if let Ok(message) = connection.poll(1000) {
            match message.event() {
                Event::DeviceStatusChange(event) => {
                    println!("Device status changed.");
                    if let Some(status) = event.status() {
                        println!("{:?}", status);
                    }
                }
                _ => {}
            }
        }
    }
}
