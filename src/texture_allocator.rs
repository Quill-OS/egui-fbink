use epi::TextureAllocator;
use egui::{Color32, TextureId};

pub struct FbinkTextureAllocator{

}

impl TextureAllocator for FbinkTextureAllocator {
    fn alloc_srgba_premultiplied(&mut self, size: (usize, usize), srgba_pixels: &[Color32]) -> TextureId {
        todo!()
    }

    fn free(&mut self, id: TextureId) {
        todo!()
    }
}