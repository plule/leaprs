use leaprs::*;

fn main() {
    let mut connection = Connection::create().expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    let mut devices = Vec::new();
    for _ in 0..=7 {
        let res = connection.poll(1000);

        if let Err(_) = res {
            continue;
        }

        devices = connection
            .get_device_list()
            .expect("Failed to list devices");

        println!("Number of devices available: {}", devices.len());
        if devices.len() > 0 {
            break;
        }
    }

    for device in devices {
        let serial = Device::open(device)
            .expect("Failed to open the device")
            .get_info()
            .expect("Failed to get info about the device")
            .get_serial_string()
            .expect("Failed to decode the serial");
        println!("{serial}");
    }

    connection.close();
}
