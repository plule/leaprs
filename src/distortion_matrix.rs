use leap_sys::LEAP_DISTORTION_MATRIX;

crate::leap_ref_struct!(DistortionMatrix, LEAP_DISTORTION_MATRIX);

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl<'a> DistortionMatrix<'a> {
    pub fn get_matrix(&self) -> [[Point; 64]; 64] {
        self.handle
            .matrix
            .map(|v| v.map(|p| Point { x: p.x, y: p.y }))
    }
}
