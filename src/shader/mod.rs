use wgpu::{Device, ShaderModule, ShaderModuleDescriptor};

pub struct ShaderManager {
    device: Device,
}

impl ShaderManager {
    pub fn new(device: Device) -> Self {
        Self { device }
    }

    pub fn compile_vertex(&self, source: &str) -> ShaderModule {
        self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("vertex"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    }

    pub fn compile_fragment(&self, source: &str) -> ShaderModule {
        self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("fragment"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    }
}
