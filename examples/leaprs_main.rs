use leaprs::*;

fn main() {
    let mut connection = Connection::create().expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    let mut device_count = 0;
    for _ in 0..=7 {
        let res = connection.poll(1000);

        if let Err(_) = res {
            continue;
        }

        device_count = connection
            .get_device_list_count()
            .expect("Failed to count devices");

        println!("Number of devices available: {}", device_count);
        if device_count > 0 {
            break;
        }
    }

    if device_count > 0 {
        let _devices = connection
            .get_device_list(device_count)
            .expect("Failed to get devices");
    }

    connection.close();
}
