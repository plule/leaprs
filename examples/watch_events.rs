use leaprs::*;

fn main() {
    let mut connection =
        Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    loop {
        if let Ok(message) = connection.poll(1000) {
            match message.event() {
                Event::None => println!("None"),
                Event::Connection(_) => println!("Connection"),
                Event::ConnectionLost(_) => println!("ConnectionLost"),
                Event::Device(_) => println!("Device"),
                Event::DeviceFailure(_) => println!("DeviceFailure"),
                Event::Policy(_) => println!("Policy"),
                Event::Tracking(_) => {} // spam
                Event::ImageRequestError => println!("ImageRequestError"),
                Event::ImageComplete => println!("ImageComplete"),
                Event::LogEvent(_) => println!("Log"),
                Event::DeviceLost => println!("DeviceLost"),
                Event::ConfigResponse(_) => println!("ConfigResponse"),
                Event::ConfigChange(_) => println!("ConfigChange"),
                Event::DeviceStatusChange(_) => println!("DeviceStatusChange"),
                Event::DroppedFrame(_) => println!("DroppedFrame"),
                Event::Image(_) => println!("Image"),
                Event::PointMappingChange(_) => println!("PointMappingChange"),
                #[cfg(feature = "gemini")]
                Event::TrackingMode(_) => println!("TrackingMode"),
                Event::LogEvents(_) => println!("LogEvents"),
                Event::HeadPose(_) => println!("HeadPose"),
                Event::Eyes(_) => println!("Eyes"),
                Event::IMU(_) => println!("IMU"),
                Event::Unknown(_) => println!("Unknown"),
            }
        }
    }
}
