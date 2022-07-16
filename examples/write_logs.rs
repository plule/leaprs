use leaprs::*;

fn main() {
    let mut connection =
        Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    loop {
        if let Ok(message) = connection.poll(1000) {
            match message.event() {
                Event::LogEvent(event) => {
                    log(&event);
                }
                Event::LogEvents(event) => {
                    for event in event.events() {
                        log(&event);
                    }
                }
                _ => {}
            }
        }
    }
}

fn log(event: &LogEvent) {
    let message = event.message().expect("invalid message");
    match event.severity() {
        LogSeverity::Unknown => println!("Unknown: {}", message),
        LogSeverity::Critical => println!("Critical: {}", message),
        LogSeverity::Warning => println!("Warning: {}", message),
        LogSeverity::Information => println!("Information: {}", message),
    }
}
