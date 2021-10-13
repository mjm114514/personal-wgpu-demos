mod vertex;
mod mesh;

pub use vertex::{Vertex, AsVertexPrimitive};
pub use mesh::Mesh;
pub(crate) use vertex::{get_middle, new_vertex};

pub struct Model {
    pub mesh: Mesh,
}