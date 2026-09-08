use wgpu::{Device, Texture, TextureDescriptor, TextureUsages, Extent3d};

pub struct TextureManager;

impl TextureManager {
    pub fn create_font_atlas(device: &Device, width: u32, height: u32) -> Texture {
        device.create_texture(&TextureDescriptor {
            label: Some("font_atlas"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        })
    }
}
