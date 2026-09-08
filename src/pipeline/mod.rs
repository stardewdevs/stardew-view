use wgpu::{Device, RenderPipeline, RenderPipelineDescriptor, PipelineLayout};

pub struct PipelineManager;

impl PipelineManager {
    pub fn create_terminal_pipeline(
        device: &Device,
        layout: &PipelineLayout,
        vertex_shader: &wgpu::ShaderModule,
        fragment_shader: &wgpu::ShaderModule,
    ) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("terminal_pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: vertex_shader,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: fragment_shader,
                entry_point: "main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }
}
