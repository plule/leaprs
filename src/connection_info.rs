use leap_sys::LEAP_CONNECTION_INFO;

use crate::ConnectionStatus;

crate::leap_struct!(ConnectionInfo, LEAP_CONNECTION_INFO);

impl ConnectionInfo {
    pub fn size(&self) -> u32 {
        self.handle.size
    }

    pub fn status(&self) -> ConnectionStatus {
        self.handle.status.into()
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
