use std::fmt::Debug;

use cgmath::{Matrix4, One, PerspectiveFov, Quaternion, Rad, Rotation3, Transform, Vector3};
use cgmath::{Decomposed, Deg};

use crate::controller::{ControllerUpdate, Controller};

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
        let inverse_view: Matrix4<f32> = self.transform.into();
        inverse_view.inverse_transform().unwrap()
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

    pub fn pitch(&mut self, angle: f32) {
        self.transform.rot = Quaternion::from_angle_x(Deg(angle)) * self.transform.rot;
    }
}

impl ControllerUpdate for Camera {
    fn update(&mut self, controller: &Controller, duration: f32) {
        controller.up_pressed.then(|| self.walk(controller.speed * duration));
        controller.down_pressed.then(|| self.walk(-controller.speed * duration));
        controller.right_pressed.then(|| self.strafe(controller.speed * duration));
        controller.left_pressed.then(|| self.strafe(-controller.speed * duration));

        controller.dragged.then(|| {
            let theta = controller.current_cursor.0 - controller.last_cursor.0;
            let phi = controller.current_cursor.1 - controller.last_cursor.1;

            self.pitch(phi as f32);
            self.rotate_y(theta as f32);
        });
    }
}