use leaprs::*;

fn main() {
    let mut connection =
        Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    loop {
        if let Ok(message) = connection.poll(1000) {
            match message.event() {
                EventRef::None => println!("None"),
                EventRef::Connection(_) => println!("Connection"),
                EventRef::ConnectionLost(_) => println!("ConnectionLost"),
                EventRef::Device(_) => println!("Device"),
                EventRef::DeviceFailure(_) => println!("DeviceFailure"),
                EventRef::Policy(_) => println!("Policy"),
                EventRef::Tracking(_) => {} // spam
                EventRef::ImageRequestError => println!("ImageRequestError"),
                EventRef::ImageComplete => println!("ImageComplete"),
                EventRef::LogEvent(_) => println!("Log"),
                EventRef::DeviceLost => println!("DeviceLost"),
                EventRef::ConfigResponse(_) => println!("ConfigResponse"),
                EventRef::ConfigChange(_) => println!("ConfigChange"),
                EventRef::DeviceStatusChange(_) => println!("DeviceStatusChange"),
                EventRef::DroppedFrame(_) => println!("DroppedFrame"),
                EventRef::Image(_) => println!("Image"),
                EventRef::PointMappingChange(_) => println!("PointMappingChange"),
                #[cfg(feature = "gemini")]
                EventRef::TrackingMode(_) => println!("TrackingMode"),
                EventRef::LogEvents(_) => println!("LogEvents"),
                EventRef::HeadPose(_) => println!("HeadPose"),
                EventRef::Eyes(_) => println!("Eyes"),
                EventRef::IMU(_) => println!("IMU"),
                EventRef::Unknown(_) => println!("Unknown"),
            }
        }
    }
}
