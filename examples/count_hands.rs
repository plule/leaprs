use leaprs::*;

fn main() {
    let mut c = Connection::create(ConnectionConfig::default()).unwrap();
    c.open().unwrap();
    loop {
        if let Ok(msg) = c.poll(1000) {
            match msg.event() {
                EventRef::Tracking(e) => println!("{} hand(s)", e.hands().len()),
                _ => {}
            }
        }
    }
}
