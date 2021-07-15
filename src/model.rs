use std::f32;
use wgpu::util::DeviceExt;
use crate::render_item::RenderItem;

pub trait AsVertexBuffer {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub tex_coord: [f32; 2],
}

// TODO: use macro to reduce the redundancy code.
impl Vertex {
    fn get_middle(v0: &Self, v1: &Self) -> Self {
        Self {
            position: [
                (v0.position[0] + v1.position[0]) / 2.0,
                (v0.position[1] + v1.position[1]) / 2.0,
                (v0.position[2] + v1.position[2]) / 2.0,
            ],
            normal: [
                (v0.normal[0] + v1.normal[0]) / 2.0,
                (v0.normal[1] + v1.normal[1]) / 2.0,
                (v0.normal[2] + v1.normal[2]) / 2.0,
            ],
            tangent: [
                (v0.tangent[0] + v1.tangent[0]) / 2.0,
                (v0.tangent[1] + v1.tangent[1]) / 2.0,
                (v0.tangent[2] + v1.tangent[2]) / 2.0,
            ],
            tex_coord: [
                (v0.tex_coord[0] + v1.tex_coord[0]) / 2.0,
                (v0.tex_coord[1] + v1.tex_coord[1]) / 2.0,
            ]
        }
    }
}

impl AsVertexBuffer for Vertex {
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

macro_rules! vertex {
    ( 
        $px:expr, $py:expr, $pz:expr,
        $nx:expr, $ny:expr, $nz:expr,
        $tx:expr, $ty:expr, $tz:expr,
        $u:expr,  $v:expr
    ) => {
        Vertex {
            position: [$px, $py, $pz],
            normal: [$nx, $ny, $nz],
            tangent: [$tx, $ty, $tz],
            tex_coord: [$u, $v],
        }
    };
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn brick(width: f32, height: f32, depth: f32, subdivision: u32) -> Self {
        let w2 = 0.5 * width;
        let h2 = 0.5 * height;
        let d2 = 0.5 * depth;
        let mut mesh = Self {
            vertices: vec![
                // Front face
                vertex!(-w2, -h2, -d2, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0),
                vertex!(-w2,  h2, -d2, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0),
                vertex!( w2,  h2, -d2, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0),
                vertex!( w2, -h2, -d2, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0),
                // Back face
                vertex!(-w2, -h2,  d2, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 1.0),
                vertex!( w2, -h2,  d2, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0),
                vertex!( w2,  h2,  d2, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0),
                vertex!(-w2,  h2,  d2, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0),
                // top face
                vertex!(-w2,  h2, -d2, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
                vertex!(-w2,  h2,  d2, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
                vertex!( w2,  h2,  d2, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0),
                vertex!( w2,  h2, -d2, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0),
                // bottom face
                vertex!(-w2, -h2, -d2, 0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, 1.0),
                vertex!( w2, -h2, -d2, 0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0),
                vertex!( w2, -h2,  d2, 0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0),
                vertex!(-w2, -h2,  d2, 0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0),
                // left face
                vertex!(-w2, -h2,  d2, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0),
                vertex!(-w2,  h2,  d2, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0),
                vertex!(-w2,  h2, -d2, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0),
                vertex!(-w2, -h2, -d2, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0),
                // right face
                vertex!( w2, -h2, -d2, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0),
                vertex!( w2,  h2, -d2, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
                vertex!( w2,  h2,  d2, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0),
                vertex!( w2, -h2,  d2, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            ],
            indices: vec![
                // front face
                0, 1, 2,
                0, 2, 3,
                // back face
                4, 5, 6,
                4, 6, 7,
                // top face
                8, 9, 10,
                8, 10, 11,
                // bottom face
                12, 13, 14,
                12, 14, 15,
                // left face
                16, 17, 18,
                16, 18, 19,
                // right face
                20, 21, 22,
                20, 22, 23,
            ],
        };
        for i in 0..subdivision {
            mesh.subdivide();
        }
        mesh
    }

    pub fn sphere(radius: f32, slice: u32, stack: u32) -> Self {
        let vertex_count = slice * (stack - 1) + 2;
        let mut mesh = Self {
            vertices: Vec::with_capacity(vertex_count as usize),
            indices: Vec::with_capacity(vertex_count as usize * 3),
        };

        // From north pole moving down by stacks.
        let top = vertex!(0.0, radius, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let bottom = vertex!(0.0, -radius, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);

        mesh.vertices.push(top);

        let phi_step = f32::consts::PI / stack as f32;
        let theta_step = 2.0 * f32::consts::PI / slice as f32;

        for i in 1..stack {
            let phi = i as f32 * phi_step;
            for j in 0..slice {
                let theta = j as f32 * theta_step;

                let position = [
                    radius * phi.sin() * theta.cos(),
                    radius * phi.cos(),
                    radius * phi.sin() * theta.sin(),
                ];

                let tangent = [
                    -radius * phi.sin() * theta.sin(),
                    0.0,
                    radius * phi.sin() * theta.cos(),
                ];

                let tex_coord = [theta / (f32::consts::PI * 2.0f32), phi / f32::consts::PI];
            }
        }

        todo!()
    }

    fn subdivide(&mut self) {
        /*
         * Subdivide a mesh by subdivide each triangle.
         * 
         *        v1
         *        *
         *       / \
         *      /   \
         *   m0*-----*m1
         *    / \   / \
         *   /   \ /   \
         *  *-----*-----*
         *  v0    m2     v2
         */
        let num_triangle = self.indices.len() / 3;
        self.vertices.reserve(num_triangle * 3);
        self.indices.reserve(num_triangle * 3);
        for i in 0..num_triangle {
            let v0_index = self.indices[i * 3];
            let v1_index = self.indices[i * 3 + 1];
            let v2_index = self.indices[i * 3 + 2];

            let m0_index = self.vertices.len() as u32;
            let m1_index = self.vertices.len() as u32 + 1;
            let m2_index = self.vertices.len() as u32 + 2;

            let v0 = self.vertices.get(v0_index as usize).unwrap();
            let v1 = self.vertices.get(v1_index as usize).unwrap();
            let v2 = self.vertices.get(v2_index as usize).unwrap();

            let m0 = Vertex::get_middle(v0, v1);
            let m1 = Vertex::get_middle(v1, v2);
            let m2 = Vertex::get_middle(v0, v2);

            self.vertices.push(m0);
            self.vertices.push(m1);
            self.vertices.push(m2);

            // Update v0-v1-v2 triangle to v0-m0-m2.
            self.indices[i * 3 + 1] = m0_index;
            self.indices[i * 3 + 2] = m2_index;

            // Append following triangles.
            self.indices.extend_from_slice(&[
                m0_index, v1_index, m1_index,
                m0_index, m1_index, m2_index,
                m2_index, m1_index, v2_index,
            ]);
        }
    }
}

pub trait LoadMesh {
    fn load_mesh<'a>(&self, mesh: &Mesh) -> RenderItem;
}

impl LoadMesh for wgpu::Device {
    fn load_mesh<'a>(&self, mesh: &Mesh) -> RenderItem {
        let vertex_buffer = self.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("load mesh vertex"),
                contents: bytemuck::cast_slice(&mesh.vertices),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let index_buffer = self.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("load mesh index"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsage::INDEX,
            }
        );

        RenderItem {
            vertex_buffer,
            index_buffer,
            num_indices: mesh.indices.len() as u32,
        }
    }
}
