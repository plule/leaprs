use leap_sys::LEAP_TRACKING_MODE_EVENT;

use crate::TrackingMode;

crate::leap_ref_struct!(TrackingModeEvent, LEAP_TRACKING_MODE_EVENT);

impl<'a> TrackingModeEvent<'a> {
    pub fn current_tracking_mode(&self) -> TrackingMode {
        self.handle.current_tracking_mode.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    fn get_tracking_mode() {
        let mut connection = initialize_connection();
        connection
            .set_tracking_mode(TrackingMode::Unknown)
            .expect_err("Set tracking mode unknown did not create an error");
        let mode =
            connection.expect_event(
                "Did not receive the tracking mode".to_string(),
                |e| match e {
                    Event::TrackingMode(mode) => Some(mode.current_tracking_mode()),
                    _ => None,
                },
            );

        assert_ne!(mode, TrackingMode::Unknown);
    }
}
