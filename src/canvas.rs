use wgpu::*;

use lyon_path::builder::{
    PathBuilder,
    BorderRadii
};
use lyon_path::Winding;
use lyon_tessellation::{
    math::{
        Point,
        Box2D,
        Angle,
        Vector
    },
    BuffersBuilder,
    VertexBuffers,
    FillTessellator,
    FillOptions,
    FillVertexConstructor,
    FillVertex,
};

use std::collections::HashMap;

//How precise the circles are
const TOLERANCE: f32 = 0.0001;

struct LyonVertexConstructor;
impl FillVertexConstructor<Vertex> for LyonVertexConstructor {
    fn new_vertex(&mut self, mut vertex: FillVertex) -> Vertex {
        let position = vertex.position();
        let attrs = vertex.interpolated_attributes();
        Vertex::new(
            [position.x, position.y, 0.0],
            [attrs[0], attrs[1], attrs[2]]
        )
    }
}

pub enum Shape {
    //Triangle(Vec2, Vec2, Vec2),
    Text(&'static str, u32, String),//text, scale, font
    RoundedRectangle(u32, u32, u32),
    Rectangle(u32, u32),
    Circle(u32)
}

pub struct Mesh {
    pub shape: Shape,
    pub offset: (u32, u32),
    pub color: Color
}

impl Mesh {
    fn draw(
        &self, buffers: &mut VertexBuffers<Vertex, u16>,
        ctp: impl Fn(u32, u32) -> Point,
        lr: impl Fn(u32) -> Vector
    ) -> Vec<Text<'static>> {
        let mut vertex_builder = BuffersBuilder::new(buffers, LyonVertexConstructor);
        let mut tessellator = FillTessellator::new();
        let options = FillOptions::default();
        let mut builder = tessellator.builder_with_attributes(
            3,
            &options,
            &mut vertex_builder
        ).flattened(TOLERANCE);
        match self.shape {
            Shape::Text(text, s, font) => {
                let p = lr(s);
                let text = Text::new(text)
                    .with_scale(wgpu_glyph::ab_glyph::PxScale::from(48.0))
                    .with_color([self.color.r as f32, self.color.g as f32, self.color.b as f32, 1.0]);
                return vec![text];
            },
            Shape::RoundedRectangle(w, h, r) => {
                builder.add_rounded_rectangle(
                    &Box2D::new(
                        ctp(self.offset.0, self.offset.1),
                        ctp(self.offset.0+w, self.offset.1+h)
                    ),
                    &BorderRadii::new(0.01),
                    Winding::Positive,
                    &[self.color.r as f32, self.color.g as f32, self.color.b as f32]
                )
            },
            Shape::Rectangle(w, h) => {
                builder.add_rectangle(
                    &Box2D::new(
                        ctp(self.offset.0, self.offset.1),
                        ctp(self.offset.0+w, self.offset.1+h)
                    ),
                    Winding::Positive,
                    &[self.color.r as f32, self.color.g as f32, self.color.b as f32]
                )
            },
            Shape::Circle(r) => {
                builder.add_ellipse(
                    ctp(self.offset.0+r, self.offset.1+r),
                    lr(r),
                    Angle::radians(0.0),
                    Winding::Positive,
                    &[self.color.r as f32, self.color.g as f32, self.color.b as f32]
                )
            }
        }
        builder.build().unwrap();
        vec![]
    }
}

pub struct Canvas2D {
    wgpu: Wgpu,
    scale_factor: f32,
    width: u32,
    height: u32,
    logical_width: f32,
    logical_height: f32,
}

impl Canvas2D {
    pub fn new(window: impl Into<SurfaceTarget<'static>>) -> Self {

        let runtime = Builder::new_current_thread()
            .build()
            .unwrap();

        let instance = Instance::new(InstanceDescriptor::default());

        let surface = instance.create_surface(window).unwrap();

        let adapter = runtime.block_on(instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::None,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        )).unwrap();

        let (device, queue) = runtime.block_on(adapter.request_device(
            &DeviceDescriptor {
                required_features: Features::empty(),
                required_limits: Limits::downlevel_webgl2_defaults(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        )).unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: 0,
            height: 0,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![surface_caps.formats[0]],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor::default());

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[
                    Vertex::desc()
                ]
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(surface_caps.formats[0].into())],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None
        });

        Canvas2D{
            runtime,
            surface,
            device,
            queue,
            config,
            render_pipeline,
            scale_factor: 1.0
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.wgpu.resize(width, height);
        self.width = width;
        self.height = height;
        self.recalc_logical_size();
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor * 2.0;
        self.recalc_logical_size();
    }

    pub fn recalc_logical_size(&mut self) {
        self.logical_width = self.width as f32 / self.scale_factor;
        self.logical_height = self.height as f32 / self.scale_factor;
    }

    pub fn radius_to_ls(&self, r: u32) -> Vector {
        let rw = r as f32 / self.logical_width;
        let rh = r as f32 / self.logical_height;
        Vector::new(rw, rh)
    }

    pub fn coords_to_point(&self, x: u32, y: u32) -> Point {
        let xd = x as f32 / self.logical_width;
        let yd = y as f32 / self.logical_height;
        Point::new(-1.0+xd, 1.0-yd)
    }

    pub fn draw(&mut self, background: Color, meshes: Vec<Mesh>) {
        let mut buffers: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let text = meshes.into_iter().flat_map(|m| m.draw(
            &mut buffers,
            |x: u32, y: u32| self.coords_to_point(x, y),
            |r: u32| self.radius_to_ls(r)
        )).collect();
        self.wgpu.draw(background, buffers.vertices, buffers.indices, text);
    }
}
