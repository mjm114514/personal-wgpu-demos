mod texture;
mod camera;
mod controller;
mod timer;
mod model;
mod application;

use cgmath::{Decomposed, Deg, InnerSpace, Matrix4, One, PerspectiveFov, Quaternion, Rotation3, Vector3, Zero};
use controller::Controller;
use futures::executor::block_on;
use timer::Timer;
use wgpu::util::DeviceExt;
use winit::{dpi::LogicalSize, event::*, event_loop::{ControlFlow, EventLoop}, window::{WindowBuilder, Window}};
use camera::Camera;
use crate::{application::Application, model::Mesh};
use crate::model::{Vertex, AsVertexPrimitive};

fn main() {
    let meshs = vec![
        Mesh::geo_sphere(1.0, 10),
    ];

    let width = 800u32;
    let height = 600u32;

    let camera = Camera::new(width as f32 / height as f32);

    let app = Application {
        meshs,
        camera,
        size: LogicalSize {
            width,
            height
        },
    };

    app.run();
}
