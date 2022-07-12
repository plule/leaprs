use leap_sys::LEAP_IMAGE;

pub trait Image {
    fn get_data(&self) -> &[u8];
}

impl Image for LEAP_IMAGE {
    fn get_data(&self) -> &[u8] {
        let width = self.properties.width;
        let height = self.properties.height;
        let size = (width * height) as usize;
        unsafe {
            let start = self.data.offset(self.offset as isize) as *const u8;
            std::slice::from_raw_parts(start, size)
        }
    }
}
