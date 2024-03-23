use derive_deref::Deref;
use leap_sys::LEAP_DISTORTION_MATRIX;

#[doc = " A matrix containing lens distortion correction coordinates."]
#[doc = ""]
#[doc = " Each point in the grid contains the coordinates of the pixel in the image buffer that"]
#[doc = " contains the data for the pixel in the undistorted image corresponding"]
#[doc = " to that point in the grid."]
#[doc = " Interpolate between points in the matrix to correct image pixels that don't"]
#[doc = " fall directly underneath a point in the distortion grid."]
#[doc = ""]
#[doc = " Current devices use a 64x64 point distortion grid."]
#[doc = " @since 3.0.0"]
#[derive(Deref)]
pub struct DistortionMatrix<'a>(pub(crate) &'a LEAP_DISTORTION_MATRIX);

#[doc = " A point in the distortion grid. @since 3.0.0"]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl<'a> DistortionMatrix<'a> {
    #[doc = " A grid of 2D points. @since 3.0.0"]
    pub fn matrix(&self) -> [[Point; 64]; 64] {
        self.matrix.map(|v| v.map(|p| Point { x: p.x, y: p.y }))
    }
}
