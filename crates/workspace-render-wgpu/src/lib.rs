//! Typed renderer requests and renderer-owned wgpu resources.

use std::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};

use bytemuck::{Pod, Zeroable};
use parking_lot::Mutex;
use tracing::info_span;
use web_time::Instant;
use wgpu::util::DeviceExt;
use workspace_core::{Camera, PaneId, PhysicalPoint, RenderMetrics, SourceId, TileKey};
use workspace_runtime::{DEFAULT_CACHE_BUDGET, DEFAULT_UPLOAD_BUDGET, DecodeEvent, TileCache};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayMap {
    Viridis,
    Greyscale,
    Threshold,
}

#[derive(Clone, Copy, Debug)]
pub struct DisplaySettings {
    pub window_low: f32,
    pub window_high: f32,
    pub map: DisplayMap,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            window_low: 0.05,
            window_high: 0.9,
            map: DisplayMap::Viridis,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PhysicalViewport {
    pub origin: PhysicalPoint,
    pub size: PhysicalPoint,
    pub scale_factor: f32,
}

#[derive(Clone, Debug)]
pub struct ImageRenderRequest {
    pub pane: PaneId,
    pub source: SourceId,
    pub viewport: PhysicalViewport,
    pub camera: Camera,
    pub display: DisplaySettings,
    pub visible_tiles: Vec<TileKey>,
}

#[derive(Default)]
pub struct RenderPlan {
    pub images: Vec<ImageRenderRequest>,
}

impl RenderPlan {
    pub fn submit(&mut self, request: ImageRenderRequest) {
        self.images.push(request);
    }
}

#[derive(Clone, Default)]
pub struct RenderBridge(pub Arc<Mutex<RenderBridgeState>>);

#[derive(Default)]
pub struct RenderBridgeState {
    pub uploads: VecDeque<DecodeEvent>,
    pub became_resident: Vec<TileKey>,
    pub metrics: RenderMetrics,
}

impl RenderBridge {
    pub fn push(&self, event: DecodeEvent) {
        self.0.lock().uploads.push_back(event);
    }
    pub fn take_resident(&self) -> Vec<TileKey> {
        std::mem::take(&mut self.0.lock().became_resident)
    }
    pub fn snapshot(&self) -> RenderMetrics {
        self.0.lock().metrics.clone()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct DisplayUniform {
    low: f32,
    high: f32,
    map: u32,
    _padding: u32,
}

struct ScalarTexture {
    _texture: wgpu::Texture,
    view: wgpu::TextureView,
}
struct PaneDraw {
    uniform: wgpu::Buffer,
    bind_groups: Vec<(TileKey, wgpu::BindGroup)>,
}

pub struct ScalarRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_layout: wgpu::BindGroupLayout,
    textures: BTreeMap<TileKey, ScalarTexture>,
    panes: BTreeMap<PaneId, PaneDraw>,
    cache: TileCache,
    bridge: RenderBridge,
    last_upload_frame: u64,
}

impl ScalarRenderer {
    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        bridge: RenderBridge,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("polyorama scalar shader"),
            source: wgpu::ShaderSource::Wgsl(SCALAR_SHADER.into()),
        });
        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("polyorama scalar bindings"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("polyorama scalar pipeline layout"),
            bind_group_layouts: &[Some(&bind_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("polyorama scalar pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        {
            let mut state = bridge.0.lock();
            state.metrics.capability_profile = format!(
                "WebGPU/WGPU; scalar=R16Uint; target={target_format:?}; timestamps=unavailable"
            );
            state.metrics.command_buffers = 1;
        }
        Self {
            pipeline,
            bind_layout,
            textures: BTreeMap::new(),
            panes: BTreeMap::new(),
            cache: TileCache::new(DEFAULT_CACHE_BUDGET),
            bridge,
            last_upload_frame: u64::MAX,
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        frame_number: u64,
        request: &ImageRenderRequest,
    ) {
        let started = Instant::now();
        let _span = info_span!("render_preparation", pane = request.pane.0).entered();
        if self.last_upload_frame != frame_number {
            self.last_upload_frame = frame_number;
            self.upload_pending(device, queue);
        }
        let map = match request.display.map {
            DisplayMap::Viridis => 0,
            DisplayMap::Greyscale => 1,
            DisplayMap::Threshold => 2,
        };
        let uniform_data = DisplayUniform {
            low: request.display.window_low,
            high: request.display.window_high,
            map,
            _padding: 0,
        };
        let pane = self.panes.entry(request.pane).or_insert_with(|| PaneDraw {
            uniform: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("polyorama display uniform"),
                contents: bytemuck::bytes_of(&uniform_data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
            bind_groups: Vec::new(),
        });
        queue.write_buffer(&pane.uniform, 0, bytemuck::bytes_of(&uniform_data));
        pane.bind_groups.clear();
        for key in &request.visible_tiles {
            if let Some(texture) = self.textures.get(key) {
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("polyorama scalar tile bind group"),
                    layout: &self.bind_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: pane.uniform.as_entire_binding(),
                        },
                    ],
                });
                pane.bind_groups.push((*key, bind_group));
            }
        }
        let mut state = self.bridge.0.lock();
        state.metrics.gpu_viewports += 1;
        state.metrics.render_jobs += 1;
        state.metrics.render_passes = 1;
        state.metrics.draw_calls += pane.bind_groups.len();
        state.metrics.resident_texture_bytes = self.cache.used();
        state.metrics.cache_evictions = self.cache.evictions;
        state.metrics.prepare_ms = started.elapsed().as_secs_f64() * 1000.0;
    }

    fn upload_pending(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let _span = info_span!("tile_upload").entered();
        let mut state = self.bridge.0.lock();
        state.metrics.gpu_viewports = 0;
        state.metrics.render_jobs = 0;
        state.metrics.draw_calls = 0;
        state.metrics.uploaded_bytes = 0;
        let mut used = 0;
        while let Some(event) = state.uploads.front() {
            let bytes = match event {
                DecodeEvent::Completed { scalar_u16_le, .. } => scalar_u16_le.len(),
                DecodeEvent::Failed { .. } => 0,
            };
            if used > 0 && used + bytes > DEFAULT_UPLOAD_BUDGET {
                break;
            }
            let event = state.uploads.pop_front().unwrap();
            if let DecodeEvent::Completed {
                key, scalar_u16_le, ..
            } = event
            {
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("polyorama R16Uint scalar tile"),
                    size: wgpu::Extent3d {
                        width: workspace_core::TILE_SIZE,
                        height: workspace_core::TILE_SIZE,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::R16Uint,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &scalar_u16_le,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(workspace_core::TILE_SIZE * 2),
                        rows_per_image: Some(workspace_core::TILE_SIZE),
                    },
                    wgpu::Extent3d {
                        width: workspace_core::TILE_SIZE,
                        height: workspace_core::TILE_SIZE,
                        depth_or_array_layers: 1,
                    },
                );
                let evicted = self.cache.insert(key, bytes);
                let eviction_span = info_span!("cache_eviction", count = evicted.len());
                let _eviction_guard = (!evicted.is_empty()).then(|| eviction_span.enter());
                for victim in evicted {
                    self.textures.remove(&victim);
                    state.metrics.resident_texture_bytes = self.cache.used();
                }
                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                self.textures.insert(
                    key,
                    ScalarTexture {
                        _texture: texture,
                        view,
                    },
                );
                state.became_resident.push(key);
                used += bytes;
            }
        }
        state.metrics.uploaded_bytes = used;
        state.metrics.pending_upload_bytes = state
            .uploads
            .iter()
            .map(|event| match event {
                DecodeEvent::Completed { scalar_u16_le, .. } => scalar_u16_le.len(),
                DecodeEvent::Failed { .. } => 0,
            })
            .sum();
        state.metrics.resident_texture_bytes = self.cache.used();
    }

    pub fn paint(
        &self,
        pane: PaneId,
        viewport: PixelRect,
        clip: PixelRect,
        render_pass: &mut wgpu::RenderPass<'static>,
    ) {
        let _span = info_span!("viewport_rendering", pane = pane.0).entered();
        let Some(draw) = self.panes.get(&pane) else {
            return;
        };
        if draw.bind_groups.is_empty() {
            return;
        }
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_scissor_rect(clip.x, clip.y, clip.width.max(1), clip.height.max(1));
        let finest = draw
            .bind_groups
            .iter()
            .map(|(key, _)| key.level)
            .min()
            .unwrap_or(0);
        if let Some((_, fallback)) = draw.bind_groups.iter().max_by_key(|(key, _)| key.level) {
            render_pass.set_viewport(
                viewport.x as f32,
                viewport.y as f32,
                viewport.width as f32,
                viewport.height as f32,
                0.0,
                1.0,
            );
            render_pass.set_bind_group(0, fallback, &[]);
            render_pass.draw(0..3, 0..1);
        }
        let fine: Vec<_> = draw
            .bind_groups
            .iter()
            .filter(|(key, _)| key.level == finest)
            .collect();
        let min_x = fine.iter().map(|(key, _)| key.x).min().unwrap_or(0);
        let max_x = fine.iter().map(|(key, _)| key.x).max().unwrap_or(min_x);
        let min_y = fine.iter().map(|(key, _)| key.y).min().unwrap_or(0);
        let max_y = fine.iter().map(|(key, _)| key.y).max().unwrap_or(min_y);
        let columns = max_x - min_x + 1;
        let rows = max_y - min_y + 1;
        for (key, bind_group) in fine {
            let x0 = viewport.x + viewport.width * (key.x - min_x) / columns;
            let y0 = viewport.y + viewport.height * (key.y - min_y) / rows;
            let x1 = viewport.x + viewport.width * (key.x - min_x + 1) / columns;
            let y1 = viewport.y + viewport.height * (key.y - min_y + 1) / rows;
            render_pass.set_viewport(
                x0 as f32,
                y0 as f32,
                (x1 - x0).max(1) as f32,
                (y1 - y0).max(1) as f32,
                0.0,
                1.0,
            );
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }
    }
}

#[derive(Clone, Copy)]
pub struct PixelRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

const SCALAR_SHADER: &str = r#"
struct Display { low: f32, high: f32, map: u32, padding: u32 };
@group(0) @binding(0) var scalar_tile: texture_2d<u32>;
@group(0) @binding(1) var<uniform> display: Display;

struct VertexOut { @builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32> };
@vertex fn vs_main(@builtin(vertex_index) index: u32) -> VertexOut {
    var positions = array<vec2<f32>, 3>(vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0));
    var out: VertexOut;
    out.position = vec4(positions[index], 0.0, 1.0);
    out.uv = positions[index] * vec2(0.5, -0.5) + vec2(0.5);
    return out;
}
fn viridis(t: f32) -> vec3<f32> {
    return vec3(0.267 + 0.633*t, 0.005 + 0.86*t - 0.42*t*t, 0.329 + 0.55*(1.0-abs(2.0*t-1.0)));
}
@fragment fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let size = textureDimensions(scalar_tile);
    let pixel = vec2<i32>(clamp(in.uv * vec2<f32>(size), vec2(0.0), vec2<f32>(size - vec2<u32>(1u))));
    let scalar = f32(textureLoad(scalar_tile, pixel, 0).r) / 65535.0;
    let t = clamp((scalar - display.low) / max(0.0001, display.high - display.low), 0.0, 1.0);
    var colour = viridis(t);
    if display.map == 1u { colour = vec3(t); }
    if display.map == 2u { colour = select(vec3(0.025, 0.035, 0.045), vec3(1.0, 0.45, 0.08), t > 0.56); }
    return vec4(colour, 1.0);
}
"#;
