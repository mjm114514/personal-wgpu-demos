use std::ops::Range;

pub struct RenderItem {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

pub trait DrawRenderItem<'a> {
    fn draw_item(&mut self, item: &'a RenderItem);
    fn draw_item_instanced(&mut self, item: &'a RenderItem, instances: Range<u32>);
}

impl<'a> DrawRenderItem<'a> for wgpu::RenderPass<'a> {

    fn draw_item(&mut self, item: &'a RenderItem) {
        self.draw_item_instanced(item, 0..1);
    }

    fn draw_item_instanced(&mut self, item: &'a RenderItem, instances: Range<u32>) {
        self.set_vertex_buffer(0, item.vertex_buffer.slice(..));
        self.set_index_buffer(item.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..item.num_indices, 0, instances);
    }
}