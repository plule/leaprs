use crate::{FrameHeader, Image};
use derive_deref::Deref;
use leap_sys::LEAP_IMAGE_EVENT;

#[doc = " Streaming stereo image pairs from the device."]
#[doc = ""]
#[doc = " LeapPollConnection() produces this message when an image is available."]
#[doc = " The struct contains image properties, the distortion grid, and a pointer to"]
#[doc = " the buffer containing the image data -- which was allocated using the allocator"]
#[doc = " function passed to LeapC using the LeapSetAllocator."]
#[doc = " @since 4.0.0"]
#[derive(Deref)]
pub struct ImageEvent<'a>(pub(crate) &'a LEAP_IMAGE_EVENT);

impl<'a> ImageEvent<'a> {
    #[doc = " The information header identifying the images tracking frame."]
    pub fn info(&self) -> FrameHeader {
        FrameHeader(&self.info)
    }

    #[doc = " The left and right images."]
    pub fn images(&self) -> [Image; 2] {
        [(Image(&self.image[0])), (Image(&self.image[1]))]
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

        let (width, height, data, _frame_id) = connection
            .wait_for(|e| match e {
                Event::Image(e) => {
                    let right_image = &e.images()[1];
                    let data = right_image.data();
                    let width = right_image.properties().width;
                    let height = right_image.properties().height;
                    let _distortion_matrix = right_image.distorion_matrix();
                    Some((width, height, data.to_vec(), e.info().frame_id))
                }
                _ => None,
            })
            .expect("Did not receive an image");

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
