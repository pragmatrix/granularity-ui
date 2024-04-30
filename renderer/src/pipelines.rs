use wgpu::VertexBufferLayout;

use crate::{
    pods::{InstanceColor, TextureColorVertex, TextureVertex},
    primitives::Pipeline,
    text_layer, texture,
    tools::BindGroupLayoutBuilder,
};

pub fn create(
    device: &wgpu::Device,
    view_projection_bind_group_layout: &wgpu::BindGroupLayout,
    texture_bind_group_layout: &texture::BindGroupLayout,
    text_layer_bind_group_layout: &text_layer::BindGroupLayout,
    shape_bind_group_layout: &wgpu::BindGroupLayout,
    targets: &[Option<wgpu::ColorTargetState>],
) -> Vec<(Pipeline, wgpu::RenderPipeline)> {
    let glyph_shader = &device.create_shader_module(wgpu::include_wgsl!("texture/glyph.wgsl"));

    let glyph_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Glyph Pipeline Layout"),
        bind_group_layouts: &[view_projection_bind_group_layout, texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    let text_layer_shader =
        &device.create_shader_module(wgpu::include_wgsl!("text_layer/text_layer.wgsl"));

    let text_layer_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Text Layer Pipeline Layout"),
            bind_group_layouts: &[
                view_projection_bind_group_layout,
                text_layer_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

    let shape_shader = &device.create_shader_module(wgpu::include_wgsl!("shape/shape.wgsl"));

    let shape_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Shape Pipeline Layout"),
        bind_group_layouts: &[view_projection_bind_group_layout, shape_bind_group_layout],
        push_constant_ranges: &[],
    });

    let texture_vertex_layout = [TextureVertex::layout()];
    let text_layer_vertex_layout = [TextureColorVertex::layout()];

    [
        (
            Pipeline::PlanarGlyph,
            create_pipeline(
                "Planar Glyph Pipeline",
                device,
                glyph_shader,
                "fs_planar",
                &texture_vertex_layout,
                &glyph_pipeline_layout,
                targets,
            ),
        ),
        (
            Pipeline::SdfGlyph,
            create_pipeline(
                "SDF Glyph Pipeline",
                device,
                glyph_shader,
                "fs_sdf_glyph",
                &texture_vertex_layout,
                &glyph_pipeline_layout,
                targets,
            ),
        ),
        (
            Pipeline::TextLayer,
            create_pipeline(
                "Text Layer Pipeline",
                device,
                text_layer_shader,
                "fs_sdf_glyph",
                &text_layer_vertex_layout,
                &text_layer_pipeline_layout,
                targets,
            ),
        ),
        (
            Pipeline::Circle,
            create_pipeline(
                "Circle Pipeline",
                device,
                shape_shader,
                "fs_sdf_circle",
                &texture_vertex_layout,
                &shape_pipeline_layout,
                targets,
            ),
        ),
        (
            Pipeline::RoundedRect,
            create_pipeline(
                "Rounded Rect Pipeline",
                device,
                shape_shader,
                "fs_sdf_rounded_rect",
                &texture_vertex_layout,
                &shape_pipeline_layout,
                targets,
            ),
        ),
    ]
    .into()
}

pub fn create_view_projection_bind_group(
    device: &wgpu::Device,
    view_projection_buffer: &wgpu::Buffer,
) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let layout = BindGroupLayoutBuilder::vertex()
        .uniform()
        .build("Camera Bind Group Layout", device);

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: view_projection_buffer.as_entire_binding(),
        }],
        label: Some("Camera Bind Group"),
    });

    (layout, bind_group)
}

fn create_pipeline(
    label: &str,
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    fragment_shader_entry: &str,
    vert_layout: &[VertexBufferLayout],
    render_pipeline_layout: &wgpu::PipelineLayout,
    targets: &[Option<wgpu::ColorTargetState>],
) -> wgpu::RenderPipeline {
    let pipeline = wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: vert_layout,
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: fragment_shader_entry,
            targets,
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    };

    device.create_render_pipeline(&pipeline)
}