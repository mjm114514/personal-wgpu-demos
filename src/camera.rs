use cgmath::{Matrix4, One, PerspectiveFov, Quaternion, Rotation3, Vector3, Rad};
use cgmath::{Decomposed, Deg};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub transform: Decomposed<Vector3<f32>, Quaternion<f32>>,
    pub projection_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            transform: Decomposed::one(),
            projection_matrix: OPENGL_TO_WGPU_MATRIX * cgmath::perspective(Deg(45.0), aspect, 0.1, 100.0),
        }
    }

    pub fn get_view_proj(&self) -> Matrix4<f32> {
        self.get_proj() * self.get_view()
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        self.transform.into()
    }

    pub fn get_proj(&self) -> Matrix4<f32> {
        self.projection_matrix
    }
    
    pub fn set_lens(&mut self, perspective: PerspectiveFov<f32>) {
        self.projection_matrix = perspective.into();
        self.projection_matrix = OPENGL_TO_WGPU_MATRIX * self.projection_matrix;
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.transform.rot * Vector3::unit_z()
    }

    pub fn right(&self) -> Vector3<f32> {
        self.transform.rot * Vector3::unit_x()
    }

    pub fn walk(&mut self, distance: f32) {
        self.transform.disp += distance * self.forward();
    }

    pub fn strafe(&mut self, distance: f32) {
        self.transform.disp += distance * self.right();
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.transform.rot = Quaternion::from_angle_y(Deg(angle)) * self.transform.rot;
    }

    pub fn Pitch(&mut self, angle: f32) {
        self.transform.rot = Quaternion::from_angle_x(Deg(angle)) * self.transform.rot;
    }
}