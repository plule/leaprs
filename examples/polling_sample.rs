use leaprs::*;

fn main() {
    let mut connection = Connection::create().expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    loop {
        if let Ok(message) = connection.poll(100) {
            let msg = match message.event() {
                Event::NoEvent => "no event",
                Event::ConnectionEvent(_) => "connection",
                Event::ConnectionLostEvent(_) => "connection lost",
                Event::DeviceEvent(_) => "device",
                Event::DeviceStatusChangeEvent(_) => "device status change",
                Event::PolicyEvent(_) => "policy event",
                Event::DeviceFailureEvent(_) => "device failure",
                Event::TrakingEvent(_) => "tracking",
                Event::TrackingModeEvent(_) => "tracking mode",
                Event::LogEvent(_) => "log event",
                Event::LogEvents(_) => "log events",
                Event::ConfigResponseEvent(_) => "config response",
                Event::DroppedFrameEvent(_) => "dropped frame",
                Event::ImageEvent(_) => "image event",
                Event::PointMappingChangeEvent(_) => "point mapping",
                Event::HeadPoseEvent(_) => "head pose",
                Event::EyeEvent(_) => "eye",
                Event::IMUEvent(_) => "imu",
                Event::LeapConfigChangeEvent(_) => "config",
                Event::ImageComplete => "image complete",
                Event::ImageRequestError => "image request error",
                Event::DeviceLost => "device lost",
            };
            println!("{}", msg);
        }
    }
}
