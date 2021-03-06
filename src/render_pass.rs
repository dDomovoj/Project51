/// Custom version of
/// use amethyst::renderer::pass::DrawBase3D;
use amethyst::{
    assets::{AssetStorage},// Handle},
    core::{
        ecs::{Join, Read, ReadExpect, ReadStorage, SystemData},
        transform::Transform,
        Hidden, HiddenPropagate,
    },
    renderer::{
        batch::{GroupIterator, OrderedTwoLevelBatch}, //, TwoLevelBatch},
        bundle::Target,
        // mtl::{FullTextureSet, Material, StaticTextureSet}, //, TexAlbedo, TexEmission},
        pipeline::{PipelineDescBuilder, PipelinesBuilder},
        // pod::VertexArgs,
        rendy::{
            command::{QueueId, RenderPassEncoder},
            factory::Factory,
            graph::{
                render::{PrepareResult, RenderGroup, RenderGroupDesc},
                GraphContext, NodeBuffer, NodeImage,
            },
            hal::{
                self,
                device::Device,
                pso::{self, ShaderStageFlags},
            },
            mesh::{AsVertex, VertexFormat},
            shader::{Shader, SpirvShader},
        },
        // resources::Tint,
        submodules::{DynamicVertexBuffer, EnvironmentSub}, //, MaterialId, MaterialSub},
        transparent::Transparent,
        types::Backend, //, Mesh},
        util,
    },
};
use derivative::Derivative;
// use smallvec::SmallVec;
use std::marker::PhantomData;

use crate::render_material::{FullTextureSet, ITextureSet, CompositeMaterial};
use crate::render_material_sub::{MaterialId, MaterialSub};
use crate::render_mesh::{CompositeMesh, Mesh};
use crate::render_vertex::Vertex;
use crate::render_visibility::{Visibility, VisibilitySortingSystem};
use crate::render_backend::IExtendedBackend;

// macro_rules! profile_scope_impl {
//     ($string:expr) => {
//         #[cfg(feature = "profiler")]
//         let _profile_scope = thread_profiler::ProfileScope::new(format!(
//             "{} {}: {}",
//             module_path!(),
//             <T as IRender3DPassDef>::NAME,
//             $string
//         ));
//     };
// }

// region - Shaders

use crate::render_shader::PathBufShaderInfo;
use crate::render_vertex::VertexArgs;
use amethyst::renderer::rendy::shader::{ShaderKind, SourceLanguage};
use std::path::PathBuf;

use amethyst::{
    core::ecs::{DispatcherBuilder, World},
    error::Error,
    renderer::bundle::{RenderOrder, RenderPlan, RenderPlugin},
};

lazy_static::lazy_static! {

    // static ref VERTEX: SpirvShader = SpirvShader::from_bytes(
    //     include_bytes!("../assets/shaders/compiled/vertex/pos_norm_tang_tex.vert.spv"),
    //     ShaderStageFlags::VERTEX,
    //     "main",
    // ).unwrap();

    // static ref FRAGMENT: SpirvShader = SpirvShader::from_bytes(
    //     include_bytes!("../assets/shaders/compiled/fragment/pbr.frag.spv"),
    //     ShaderStageFlags::FRAGMENT,
    //     "main",
    // ).unwrap();

    static ref VERTEX: SpirvShader = PathBufShaderInfo::new(
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/src/vertex/custom.vert")),
        ShaderKind::Vertex,
        SourceLanguage::GLSL,
       "main",
    ).precompile().unwrap();

    // static ref MATH: SpirvShader = PathBufShaderInfo::new(
    //     PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/src/fragment/header/math.frag")),
    //     ShaderKind::Fragment,
    //     SourceLanguage::GLSL,
    //    "main",
    // ).precompile().unwrap();

    // static ref ENV: SpirvShader = PathBufShaderInfo::new(
    //     PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/src/fragment/header/env.frag")),
    //     ShaderKind::Fragment,
    //     SourceLanguage::GLSL,
    //    "main",
    // ).precompile().unwrap();

    static ref FRAGMENT: SpirvShader = PathBufShaderInfo::new(
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/src/fragment/custom.frag")),
        ShaderKind::Fragment,
        SourceLanguage::GLSL,
       "main",
    ).precompile().unwrap();

}

/// Example code of using a custom shader
///
/// Requires "shader-compiler" flag
///
/// ''' rust
/// use std::path::PathBuf;
/// use amethyst::renderer::rendy::shader::{PathBufShaderInfo, ShaderKind, SourceLanguage};
///
///  lazy_static::lazy_static! {
///     static ref VERTEX: SpirvShader = PathBufShaderInfo::new(
///         PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/assets/shaders/src/vertex/custom.vert")),
///         ShaderKind::Vertex,
///         SourceLanguage::GLSL,
///        "main",
///     ).precompile().unwrap();
///
///     static ref FRAGMENT: SpirvShader = PathBufShaderInfo::new(
///         PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/assets/shaders/src/fragment/custom.frag")),
///         ShaderKind::Fragment,
///         SourceLanguage::GLSL,
///         "main",
///     ).precompile().unwrap();
/// }
/// '''

// endregion

// region - Plugin

/// A `RenderPlugin` for forward rendering of 3d objects shading.
// pub type Render3D = BaseRender<CustomPassDef>;

/// Describes a Base 3d Pass with lighting without transparency
pub type Draw3DDesc<B> = BaseDrawDesc<B, CustomPassDef>;

// endregion

// region - 3DPassDef

/// Implementation of `CustomPassDef` describing a simple shaded 3D pass.
#[derive(Debug)]
pub struct CustomPassDef;

impl IRenderPassDef for CustomPassDef {
    const NAME: &'static str = "Render 3d";

    type TextureSet = FullTextureSet;

    fn vertex_shader() -> &'static SpirvShader {
        &VERTEX
    }
    fn fragment_shader() -> &'static SpirvShader {
        &FRAGMENT
    }
    fn base_format() -> Vec<VertexFormat> {
        vec![Vertex::vertex()]
    }
}

// region - RenderPass

/// A `RenderPlugin` for forward rendering of 3d objects.
/// Generic over 3d pass rendering method.
#[derive(derivative::Derivative)]
#[derivative(Default(bound = ""), Debug(bound = ""))]
pub struct BaseRender<D: IRenderPassDef> {
    target: Target,
    marker: std::marker::PhantomData<D>,
}

// impl<D: IRenderPassDef> BaseRender<D> {
//     /// Set target to which 3d meshes will be rendered.
//     pub fn with_target(mut self, target: Target) -> Self {
//         self.target = target;
//         self
//     }
// }

impl<B: IExtendedBackend, D: IRenderPassDef> RenderPlugin<B> for BaseRender<D> {
    fn on_build<'a, 'b>(&mut self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(VisibilitySortingSystem::new(), "visibility_system", &[]);
        Ok(())
    }

    fn on_plan(&mut self, plan: &mut RenderPlan<B>, _factory: &mut Factory<B>, _world: &World) -> Result<(), Error> {
        plan.extend_target(self.target, move |ctx| {
            ctx.add(RenderOrder::Opaque, BaseDrawDesc::<B, D>::new().builder())?;
            // ctx.add(
            //     RenderOrder::Transparent,
            //     BaseDrawTransparentDesc::<B, D>::new().builder(),
            // )?;
            Ok(())
        });
        Ok(())
    }
}

// endregion

// region - IRenderPassDef

pub trait IRenderPassDef: 'static + std::fmt::Debug + Send + Sync {
    const NAME: &'static str;

    /// The [ITextureSet] type implementation for this pass
    type TextureSet: for<'a> ITextureSet<'a>;

    fn vertex_shader() -> &'static SpirvShader;

    fn fragment_shader() -> &'static SpirvShader;

    fn base_format() -> Vec<VertexFormat>;
}

// region - BaseDrawDesc

/// Draw opaque 3d meshes with specified shaders and texture set
#[derive(Clone, Derivative)]
#[derivative(Debug(bound = ""), Default(bound = ""))]
pub struct BaseDrawDesc<B: IExtendedBackend, T: IRenderPassDef> {
    marker: PhantomData<(B, T)>,
}

impl<B: IExtendedBackend, T: IRenderPassDef> BaseDrawDesc<B, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<B: IExtendedBackend, T: IRenderPassDef> RenderGroupDesc<B, World> for BaseDrawDesc<B, T> {
    fn build(
        self, _ctx: &GraphContext<B>, factory: &mut Factory<B>, _queue: QueueId, _aux: &World, framebuffer_width: u32,
        framebuffer_height: u32, subpass: hal::pass::Subpass<'_, B>, _buffers: Vec<NodeBuffer>,
        _images: Vec<NodeImage>,
    ) -> Result<Box<dyn RenderGroup<B, World>>, failure::Error> {
        // profile_scope_impl!("build");

        let env = EnvironmentSub::new(factory, [ShaderStageFlags::VERTEX, ShaderStageFlags::FRAGMENT])?;
        let materials = MaterialSub::new(factory)?;
        let mut vertex_format_base = T::base_format();

        let (mut pipelines, pipeline_layout) = build_pipelines::<B, T>(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
            &vertex_format_base,
            false,
            vec![env.raw_layout(), materials.raw_layout()],
            // vec![env.raw_layout()],
        )?;

        vertex_format_base.sort();

        Ok(Box::new(BaseDraw::<B, T> {
            pipeline_basic: pipelines.remove(0),
            pipeline_layout,
            static_batches: Default::default(),
            vertex_format_base,
            env,
            materials,
            models: DynamicVertexBuffer::new(),
            change: Default::default(),
            marker: PhantomData,
        }))
    }
}

// endregion

// region - BaseDraw

/// Base implementation of a 3D render pass
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct BaseDraw<B: IExtendedBackend, T: IRenderPassDef> {
    pipeline_basic: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    // static_batches: TwoLevelBatch<MaterialId, u32, SmallVec<[VertexArgs; 4]>>,
    static_batches: OrderedTwoLevelBatch<MaterialId, u32, VertexArgs>,
    // static_batches: TwoLevelBatch<u32, u32, SmallVec<[VertexArgs; 4]>>,
    vertex_format_base: Vec<VertexFormat>,
    env: EnvironmentSub<B>,
    materials: MaterialSub<B, T::TextureSet>,
    models: DynamicVertexBuffer<B, VertexArgs>,
    change: util::ChangeDetection,
    marker: PhantomData<T>,
}

impl<B: IExtendedBackend, T: IRenderPassDef> RenderGroup<B, World> for BaseDraw<B, T> {
    fn prepare(
        &mut self, factory: &Factory<B>, _queue: QueueId, index: usize, _subpass: hal::pass::Subpass<'_, B>,
        resources: &World,
    ) -> PrepareResult {
        // return PrepareResult::DrawReuse;

        // profile_scope_impl!("prepare opaque");

        let (
            mesh_elements_assets,
            visibility,
            _transparent,
            _hiddens,
            _hiddens_prop,
            meshes,
            materials,
            transforms,
            // tints,
        ) = <(
            Read<'_, AssetStorage<Mesh>>,
            ReadExpect<'_, Visibility>,
            ReadStorage<'_, Transparent>,
            ReadStorage<'_, Hidden>,
            ReadStorage<'_, HiddenPropagate>,
            ReadStorage<'_, CompositeMesh>,
            ReadStorage<'_, CompositeMaterial>,
            ReadStorage<'_, Transform>,
            // ReadStorage<'_, Tint>,
        )>::fetch(resources);

        // Prepare environment
        self.env.process(factory, index, resources);
        self.materials.maintain();

        // self.static_batches.clear_inner();
        self.static_batches.swap_clear();
        let mut changed = false;

        let materials_ref = &mut self.materials;
        let statics_ref = &mut self.static_batches;

        // let static_input = || ((&materials, &meshes, &transforms, tints.maybe()));
        let static_input = || ((&materials, &meshes, &transforms));
        {
            // profile_scope_impl!("prepare");
            (static_input(), &visibility.visible_unordered)
                .join()
                // .map(|((mat, mesh, tform, tint), _)| {
                .flat_map(|((mat, mesh, tform), _)| {
                    let args = VertexArgs::from_object_data(tform);
                    // ((mat, mesh.id()), VertexArgs::from_object_data(tform, tint, 0))
                    mat.components
                        .iter()
                        .zip(mesh.elements.iter())
                        .map(move |(m, e)| ((m, e.id()), args))
                })
                // .for_each_group(|(mat, mesh_id), data| {
                .for_each_group(|(mat, mesh_element_id), data| {
                    if mesh_elements_assets.contains_id(mesh_element_id) {
                        // if let Some((mat, _)) = materials_ref.insert(factory, resources, mat) {
                        //     statics_ref.insert(mat, mesh_element_id, data.drain(..));
                        // }
                        if let Some((mat, this_changed)) = materials_ref.insert(factory, resources, mat) {
                            changed = changed || this_changed;
                            statics_ref.insert(mat, mesh_element_id, data.drain(..));
                        }
                    }
                });
        }
        //{
        //    profile_scope_impl!("write");
        //    self.static_batches.prune();
        //
        //     self.models.write(
        //         factory,
        //         index,
        //         self.static_batches.count() as u64,
        //         self.static_batches.data(),
        //     );
        // }
        // PrepareResult::DrawRecord
        self.models.write(
            factory,
            index,
            self.static_batches.count() as u64,
            Some(self.static_batches.data()),
        );

        changed = changed || self.static_batches.changed();

        self.change.prepare_result(index, changed)
    }

    fn draw_inline(
        &mut self, mut encoder: RenderPassEncoder<'_, B>, index: usize, _subpass: hal::pass::Subpass<'_, B>,
        resources: &World,
    ) {
        // return;

        // profile_scope_impl!("draw opaque");
        let mesh_elements_assets = <Read<'_, AssetStorage<Mesh>>>::fetch(resources);
        let models_loc = self.vertex_format_base.len() as u32;

        encoder.bind_graphics_pipeline(&self.pipeline_basic);
        self.env.bind(index, &self.pipeline_layout, 0, &mut encoder);

        if self.models.bind(index, models_loc, 0, &mut encoder) {
            let mut instances_drawn = 0;
            for (&mat_id, batches) in self.static_batches.iter() {
                if self.materials.loaded(mat_id) {
                    self.materials.bind(&self.pipeline_layout, 1, mat_id, &mut encoder);
                    for (mesh_element_id, batch_data) in batches {
                        debug_assert!(mesh_elements_assets.contains_id(*mesh_element_id));
                        if let Some(mesh_element) = B::unwrap_mesh_element(unsafe {
                            mesh_elements_assets.get_by_id_unchecked(*mesh_element_id)
                        }) {
                            println!("{}", batch_data.len() as u32);
                            mesh_element
                                .bind_and_draw(
                                    0,
                                    &self.vertex_format_base,
                                    instances_drawn..instances_drawn + batch_data.len() as u32,
                                    &mut encoder,
                                )
                                .unwrap();
                        }
                        instances_drawn += batch_data.len() as u32;
                    }
                }
            }
        }
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _aux: &World) {
        // profile_scope_impl!("dispose");
        unsafe {
            factory.device().destroy_graphics_pipeline(self.pipeline_basic);
            factory.device().destroy_pipeline_layout(self.pipeline_layout);
        }
    }
}

// endregion

// region - BaseDrawTransparentDesc

/// Draw transparent mesh with physically based lighting
#[derive(Clone, Derivative)]
#[derivative(Debug(bound = ""), Default(bound = ""))]
pub struct BaseDrawTransparentDesc<B: IExtendedBackend, T: IRenderPassDef> {
    marker: PhantomData<(B, T)>,
}

impl<B: IExtendedBackend, T: IRenderPassDef> BaseDrawTransparentDesc<B, T> {
    /// Create pass in default configuration
    pub fn new() -> Self {
        Self { marker: PhantomData }
    }
}

impl<B: IExtendedBackend, T: IRenderPassDef> RenderGroupDesc<B, World> for BaseDrawTransparentDesc<B, T> {
    fn build(
        self, _ctx: &GraphContext<B>, factory: &mut Factory<B>, _queue: QueueId, _aux: &World, framebuffer_width: u32,
        framebuffer_height: u32, subpass: hal::pass::Subpass<'_, B>, _buffers: Vec<NodeBuffer>,
        _images: Vec<NodeImage>,
    ) -> Result<Box<dyn RenderGroup<B, World>>, failure::Error> {
        let env = EnvironmentSub::new(
            factory,
            [hal::pso::ShaderStageFlags::VERTEX, hal::pso::ShaderStageFlags::FRAGMENT],
        )?;

        // let materials = MaterialSub::new(factory)?;

        let mut vertex_format_base = T::base_format();

        let (mut pipelines, pipeline_layout) = build_pipelines::<B, T>(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
            &vertex_format_base,
            true,
            // vec![env.raw_layout(), materials.raw_layout()],
            vec![env.raw_layout()],
        )?;

        vertex_format_base.sort();

        Ok(Box::new(BaseDrawTransparent::<B, T> {
            pipeline_basic: pipelines.remove(0),
            pipeline_layout,
            static_batches: Default::default(),
            vertex_format_base,
            env,
            // materials,
            models: DynamicVertexBuffer::new(),
            change: Default::default(),
            marker: PhantomData,
        }))
    }
}

// endregion

// region - BaseDrawTransparent

/// Draw transparent mesh with physically based lighting
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct BaseDrawTransparent<B: IExtendedBackend, T: IRenderPassDef> {
    pipeline_basic: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    // static_batches: OrderedTwoLevelBatch<MaterialId, u32, VertexArgs>,
    static_batches: OrderedTwoLevelBatch<u32, u32, VertexArgs>,
    vertex_format_base: Vec<VertexFormat>,
    env: EnvironmentSub<B>,
    // materials: MaterialSub<B, FullTextureSet>,
    models: DynamicVertexBuffer<B, VertexArgs>,
    change: util::ChangeDetection,
    marker: PhantomData<T>,
}

impl<B: IExtendedBackend, T: IRenderPassDef> RenderGroup<B, World> for BaseDrawTransparent<B, T> {
    fn prepare(
        &mut self, factory: &Factory<B>, _queue: QueueId, index: usize, _subpass: hal::pass::Subpass<'_, B>,
        resources: &World,
    ) -> PrepareResult {
        // profile_scope_impl!("prepare transparent");

        // let (mesh_storage, visibility, meshes, materials, transforms, tints) =
        let (mesh_elements_assets, visibility, meshes, transforms) = <(
            Read<'_, AssetStorage<Mesh>>,
            ReadExpect<'_, Visibility>,
            ReadStorage<'_, CompositeMesh>,
            // ReadStorage<'_, Handle<Material>>,
            ReadStorage<'_, Transform>,
            // ReadStorage<'_, Tint>,
        )>::fetch(resources);

        // Prepare environment
        self.env.process(factory, index, resources);
        // self.materials.maintain();

        self.static_batches.swap_clear();

        // let materials_ref = &mut self.materials;
        let statics_ref = &mut self.static_batches;
        let mut changed = false;

        // let mut joined = (&materials, &meshes, &transforms, tints.maybe()).join();
        let mut joined = (&meshes, &transforms).join();
        visibility
            .visible_ordered
            .iter()
            .filter_map(|e| joined.get_unchecked(e.id()))
            // .map(|(mat, mesh, tform, tint)| {
            .flat_map(|(mesh, tform)| {
                let args = VertexArgs::from_object_data(tform);
                mesh.elements.iter().map(move |e| (e.id(), args))
            })
            // .for_each_group(|(mat, mesh_id), data| {
            .for_each_group(|mesh_element_id, data| {
                if mesh_elements_assets.contains_id(mesh_element_id) {
                    // if let Some((mat, this_changed)) = materials_ref.insert(factory, resources, mat)
                    // {
                    //     changed = changed || this_changed;
                    //     statics_ref.insert(mat, mesh_id, data.drain(..));
                    // }
                    statics_ref.insert(0, mesh_element_id, data.drain(..));
                }
            });

        self.models.write(
            factory,
            index,
            self.static_batches.count() as u64,
            Some(self.static_batches.data()),
        );

        changed = changed || self.static_batches.changed();

        self.change.prepare_result(index, changed)
    }

    fn draw_inline(
        &mut self, mut encoder: RenderPassEncoder<'_, B>, index: usize, _subpass: hal::pass::Subpass<'_, B>,
        resources: &World,
    ) {
        // profile_scope_impl!("draw transparent");

        let mesh_elements_assets = <Read<'_, AssetStorage<Mesh>>>::fetch(resources);
        let layout = &self.pipeline_layout;
        let encoder = &mut encoder;

        let models_loc = self.vertex_format_base.len() as u32;

        encoder.bind_graphics_pipeline(&self.pipeline_basic);
        self.env.bind(index, layout, 0, encoder);

        if self.models.bind(index, models_loc, 0, encoder) {
            for (&_mat, batches) in self.static_batches.iter() {
                // if self.materials.loaded(mat) {
                // self.materials.bind(layout, 1, mat, encoder);
                for (mesh, range) in batches {
                    // debug_assert!(mesh_storage.contains_id(*mesh));
                    if let Some(mesh) =
                        B::unwrap_mesh_element(unsafe { mesh_elements_assets.get_by_id_unchecked(*mesh) })
                    {
                        if let Err(error) = mesh.bind_and_draw(0, &self.vertex_format_base, range.clone(), encoder) {
                            log::warn!(
                                    "Trying to draw a mesh that lacks {:?} vertex attributes. Pass {} requires attributes {:?}.",
                                    error.not_found.attributes,
                                    T::NAME,
                                    T::base_format(),
                                );
                        }
                    }
                }
                // }
            }
        }
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _aux: &World) {
        unsafe {
            factory.device().destroy_graphics_pipeline(self.pipeline_basic);
            factory.device().destroy_pipeline_layout(self.pipeline_layout);
        }
    }
}

// endregion

// region - Common

fn build_pipelines<B: Backend, T: IRenderPassDef>(
    factory: &Factory<B>, subpass: hal::pass::Subpass<'_, B>, framebuffer_width: u32, framebuffer_height: u32,
    vertex_format_base: &[VertexFormat], transparent: bool, layouts: Vec<&B::DescriptorSetLayout>,
) -> Result<(Vec<B::GraphicsPipeline>, B::PipelineLayout), failure::Error> {
    let pipeline_layout = unsafe { factory.device().create_pipeline_layout(layouts, None as Option<(_, _)>) }?;

    let vertex_desc = vertex_format_base
        .iter()
        .map(|f| (f.clone(), pso::VertexInputRate::Vertex))
        .chain(Some((VertexArgs::vertex(), pso::VertexInputRate::Instance(1))))
        .collect::<Vec<_>>();

    let shader_vertex_basic = unsafe { T::vertex_shader().module(factory).unwrap() };
    let shader_fragment = unsafe { T::fragment_shader().module(factory).unwrap() };
    let pipe_desc = PipelineDescBuilder::new()
        .with_vertex_desc(&vertex_desc)
        .with_shaders(util::simple_shader_set(&shader_vertex_basic, Some(&shader_fragment)))
        .with_layout(&pipeline_layout)
        .with_subpass(subpass)
        .with_framebuffer_size(framebuffer_width, framebuffer_height)
        .with_face_culling(if transparent { pso::Face::BACK } else { pso::Face::BACK })
        .with_depth_test(pso::DepthTest {
            fun: pso::Comparison::Less,
            write: !transparent,
        })
        .with_blend_targets(vec![pso::ColorBlendDesc {
            mask: pso::ColorMask::ALL,
            blend: if transparent {
                Some(pso::BlendState::PREMULTIPLIED_ALPHA)
            } else {
                None
            },
        }]);

    let pipelines = { PipelinesBuilder::new().with_pipeline(pipe_desc).build(factory, None) };

    unsafe {
        factory.destroy_shader_module(shader_vertex_basic);
        factory.destroy_shader_module(shader_fragment);
    }

    match pipelines {
        Err(e) => {
            unsafe {
                factory.device().destroy_pipeline_layout(pipeline_layout);
            }
            Err(e)
        }
        Ok(pipelines) => Ok((pipelines, pipeline_layout)),
    }
}
