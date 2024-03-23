use derive_deref::Deref;
use leap_sys::LEAP_CONNECTION_INFO;

use crate::ConnectionStatus;

#[doc = " \\ingroup Structs"]
#[doc = " Information about a connection."]
#[doc = ""]
#[doc = " Call LeapCreateConnection() to generate the handle for the connection;"]
#[doc = " call LeapOpenConnection() to establish the connection; then call"]
#[doc = " LeapGetConnectionInfo(), which creates this struct, to check the connection status."]
#[doc = " @since 3.0.0"]
#[derive(Deref)]
pub struct ConnectionInfo(pub(crate) LEAP_CONNECTION_INFO);

impl ConnectionInfo {
    pub fn status(&self) -> ConnectionStatus {
        self.status.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    fn connection_info_test() {
        let mut connection = initialize_connection();
        let info = connection.info().expect("Failed to get connection info");
        assert_eq!(info.status(), ConnectionStatus::Connected);
    }
}
