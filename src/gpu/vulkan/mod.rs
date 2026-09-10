use wgpu::{
    Adapter, Backends, Device, Instance, PowerPreference, Queue, RequestAdapterOptions,
    Surface, SurfaceConfiguration,
};

pub struct VulkanBackend {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
}

impl VulkanBackend {
    pub async fn new(surface: Surface<'static>, width: u32, height: u32) -> Self {
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::VULKAN,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find Vulkan adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create Vulkan device");

        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let alpha_mode = caps.alpha_modes[0];

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            config,
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    pub fn config(&self) -> &SurfaceConfiguration {
        &self.config
    }
}
