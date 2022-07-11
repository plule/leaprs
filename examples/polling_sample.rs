use leaprs::*;

fn main() {
    let mut connection = Connection::create().expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    loop {
        if let Ok(message) = connection.poll(100) {
            match message.event() {
                Event::DeviceEvent(e) => {
                    let mut device = Device::open(e.device).expect("Failed to open device");
                    let serial_length = device
                        .get_info_serial_length()
                        .expect("Failed to get serial length");
                    println!("{}", serial_length);
                    let info = device
                        .get_info(serial_length)
                        .expect("Failed to get device info");
                    let serial = info.get_serial();
                    println!("device {:#?}", serial);
                }
                Event::TrakingEvent(e) => {
                    for hand in e.get_hands() {
                        let id = hand.id;
                        let grb = hand.grab_angle;
                        println!("hand {} grab {}", id, grb)
                    }
                }
                _ => {}
            };
        }
    }
}
