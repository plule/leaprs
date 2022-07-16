use crate::{FrameHeader, Image};
use leap_sys::LEAP_IMAGE_EVENT;

crate::leap_ref_struct!(ImageEvent, LEAP_IMAGE_EVENT);

impl<'a> ImageEvent<'a> {
    pub fn info(&self) -> FrameHeader {
        (&self.handle.info).into()
    }

    pub fn images(&self) -> [Image; 2] {
        [
            (&self.handle.image[0]).into(),
            (&self.handle.image[1]).into(),
        ]
    }
}

#[cfg(test)]
mod tests {
    pub use crate::tests::*;
    pub use crate::*;
    #[test]
    pub fn receive_image() {
        let mut connection = initialize_connection();

        connection
            .set_policy_flags(PolicyFlags::IMAGES, PolicyFlags::empty())
            .expect("Failed to set policy flags");

        let (width, height, data) =
            connection.expect_event("Did not receive an image".to_string(), |e| match e {
                Event::Image(e) => {
                    let right_image = &e.images()[1];
                    let data = right_image.data();
                    let width = right_image.properties().width();
                    let height = right_image.properties().height();
                    let _distortion_matrix = right_image.distorion_matrix();
                    Some((width, height, data.to_vec()))
                }
                _ => None,
            });

        let temp_dir = tempfile::tempdir().expect("Failed to create a temp dir");
        let mut path = temp_dir.into_path();
        path.push("image.png");
        ::image::save_buffer(
            path.to_str().unwrap(),
            &data,
            width,
            height,
            ::image::ColorType::L8,
        )
        .expect("failed to save buffer");

        connection
            .set_policy_flags(PolicyFlags::empty(), PolicyFlags::IMAGES)
            .expect("Failed to set policy flags");
    }
}
