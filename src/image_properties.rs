use leap_sys::LEAP_IMAGE_PROPERTIES;

use crate::{ImageFormat, ImageType};

crate::leap_ref_struct!(ImageProperties, LEAP_IMAGE_PROPERTIES);

impl<'a> ImageProperties<'a> {
    pub fn image_type(&self) -> ImageType {
        self.handle.type_.into()
    }

    pub fn image_format(&self) -> ImageFormat {
        self.handle.format.into()
    }

    pub fn bpp(&self) -> u32 {
        self.handle.bpp
    }

    pub fn width(&self) -> u32 {
        self.handle.width
    }

    pub fn height(&self) -> u32 {
        self.handle.height
    }
}
