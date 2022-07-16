use leap_sys::LEAP_VERSION;

crate::leap_struct!(Version, LEAP_VERSION);

impl Version {
    pub fn major(&self) -> i32 {
        self.handle.major
    }

    pub fn minor(&self) -> i32 {
        self.handle.minor
    }

    pub fn patch(&self) -> i32 {
        self.handle.patch
    }
}
