use wgpu::Backends;

pub struct WebGpuBackend;

impl WebGpuBackend {
    pub fn new() -> Self {
        let _instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::BROWSER_WEBGPU,
            ..Default::default()
        });
        Self
    }
}
