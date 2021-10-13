use cgmath::{Vector3, Vector2};

pub trait AsVertexPrimitive {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tangent: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
}

unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

impl AsVertexPrimitive for Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float2,
                },
            ]
        }
    }
}

macro_rules! new_vertex {
    ( 
        $px:expr, $py:expr, $pz:expr,
        $nx:expr, $ny:expr, $nz:expr,
        $tx:expr, $ty:expr, $tz:expr,
        $u:expr,  $v:expr
    ) => {
        Vertex {
            position: [$px, $py, $pz].into(),
            normal: [$nx, $ny, $nz].into(),
            tangent: [$tx, $ty, $tz].into(),
            tex_coord: [$u, $v].into(),
        }
    };
    ($px:expr, $py:expr, $pz:expr) => {
        Vertex {
            position: [$px, $py, $pz].into(),
            normal: [0.0, 0.0, 0.0].into(),
            tangent: [0.0, 0.0, 0.0].into(),
            tex_coord: [0.0, 0.0].into(),
        }
    };
}

macro_rules! get_middle {
    ($v0:expr, $v1:expr) => {
        Vertex {
            position: ($v0.position + $v1.position) / 2.0,
            normal: ($v0.normal + $v1.normal) / 2.0,
            tangent: ($v0.tangent + $v1.tangent) / 2.0,
            tex_coord: ($v0.tex_coord + $v1.tex_coord) / 2.0,
        }
    };
}

pub(crate) use new_vertex;
pub(crate) use get_middle;