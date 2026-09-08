use wgpu::{Device, Buffer, BufferUsages, BufferDescriptor};

pub struct VertexBuffer {
    pub buffer: Buffer,
    pub size: u64,
}

impl VertexBuffer {
    pub fn new(device: &Device, data: &[u8], label: &str) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size: data.len() as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self { buffer, size: data.len() as u64 }
    }
}
