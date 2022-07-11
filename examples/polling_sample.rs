use leaprs::*;

fn main() {
    let mut connection = Connection::create().expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    loop {
        if let Ok(message) = connection.poll(100) {
            match message.event() {
                Event::TrakingEvent(e) => {
                    for hand in e.get_hands() {
                        let id = hand.id;
                        let grb = hand.grab_angle;
                        let index = hand.get_index();
                        println!("hand {} grab {}", id, grb)
                    }
                }
                _ => {}
            };
        }
    }
}
