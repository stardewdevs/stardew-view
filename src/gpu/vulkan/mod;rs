use wgpu::{Backends, Instance, Device, Surface, SurfaceConfiguration, Adapter, PowerPreference};

pub struct VulkanBackend {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    surface: Surface,
    config: SurfaceConfiguration,
}

impl VulkanBackend {
    pub async fn new(surface: Surface, width: u32, height: u32) -> Self {
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::VULKAN,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find Vulkan adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            })
            .await
            .expect("Failed to create Vulkan device");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        Self {
            instance,
            adapter,
            device,
            surface,
            config,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }
}
