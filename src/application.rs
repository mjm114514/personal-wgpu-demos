use futures::executor::block_on;
use crate::{camera::Camera, model::Mesh};

use winit::{dpi::LogicalSize, event::*, event_loop::{ControlFlow, EventLoop}, window::{self, Window, WindowBuilder}};

pub struct Application {
    pub meshs: Vec<Mesh>,
    pub camera: Camera,
    pub size: LogicalSize<u32>,
}

impl Application {
    pub fn run(&self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(self.size)
            .build(&event_loop)
            .unwrap();
        
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let (adapter, device, queue) = block_on(async {
            let adapter = instance.request_adapter(
                &Default::default(),
            ).await.unwrap();

            let (device, queue) = adapter.request_device(
                &Default::default(),
                None,
            ).await.unwrap();

            (adapter, device, queue)
        });
    }
}