//! Extended Renderer system
use amethyst::renderer::{
    camera::{ActiveCamera, Camera},
    debug_drawing::DebugLinesComponent,
    light::Light,
    // resources::Tint,
    skinning::JointTransforms,
    // sprite::SpriteRender,
    transparent::Transparent,
    types::{Texture},
    GraphCreator,
};
use amethyst::assets::{AssetStorage, Handle, HotReloadStrategy, ProcessingState, ThreadPool};
use amethyst::core::{
    components::Transform,
    ecs::{Read, ReadExpect, ReadStorage, RunNow, System, SystemData, World, Write, WriteExpect},
    timing::Time,
    Hidden, HiddenPropagate,
};
use amethyst::renderer::palette::{Srgba};
use amethyst::renderer::rendy::{
    command::{Families, QueueId},
    factory::{Factory, ImageState},
    graph::{Graph},
    texture::palette::{load_from_srgba},
};
use std::{marker::PhantomData, sync::Arc};

// #[cfg(feature = "profiler")]
// use thread_profiler::profile_scope;

use crate::render_cache::{MaterialCache, MeshCache, TextureCache};
use crate::render_material::{Material, CompositeMaterial, MaterialDefaults};
use crate::render_mesh::{Mesh, CompositeMesh};
use crate::render_visibility::Visibility;
use crate::render_backend::IExtendedBackend;

/// Extended Amethyst rendering system
#[allow(missing_debug_implementations)]
pub struct ExtendedRenderingSystem<B, G>
where
    B: IExtendedBackend,
    G: GraphCreator<B>,
{
    graph: Option<Graph<B, World>>,
    families: Option<Families<B>>,
    graph_creator: G,
}

impl<B, G> ExtendedRenderingSystem<B, G>
where
    B: IExtendedBackend,
    G: GraphCreator<B>,
{
    /// Create a new `ExtendedRenderingSystem` with the supplied graph via `GraphCreator`
    pub fn new(graph_creator: G) -> Self {
        Self {
            graph: None,
            families: None,
            graph_creator,
        }
    }
}

type SetupData<'a> = (
    Read<'a, AssetStorage<Material>>,
    Read<'a, AssetStorage<Mesh>>,
    ReadStorage<'a, Handle<Mesh>>,
    ReadStorage<'a, Handle<Texture>>,
    ReadStorage<'a, Handle<Material>>,
    ReadStorage<'a, CompositeMaterial>,
    ReadStorage<'a, CompositeMesh>,
    // ReadStorage<'a, Tint>,
    ReadStorage<'a, Light>,
    ReadStorage<'a, Camera>,
    ReadStorage<'a, Hidden>,
    ReadStorage<'a, HiddenPropagate>,
    ReadStorage<'a, DebugLinesComponent>,
    ReadStorage<'a, Transparent>,
    ReadStorage<'a, Transform>,
    // ReadStorage<'a, SpriteRender>,
    Option<Read<'a, Visibility>>,
    Read<'a, ActiveCamera>,
    ReadStorage<'a, JointTransforms>,
);

impl<B, G> ExtendedRenderingSystem<B, G>
where
    B: IExtendedBackend,
    G: GraphCreator<B>,
{
    fn rebuild_graph(&mut self, world: &World) {
        // #[cfg(feature = "profiler")]
        // profile_scope!("rebuild_graph");

        let mut factory = world.fetch_mut::<Factory<B>>();

        if let Some(graph) = self.graph.take() {
            // #[cfg(feature = "profiler")]
            // profile_scope!("dispose_graph");
            graph.dispose(&mut *factory, world);
        }

        let builder = {
            // #[cfg(feature = "profiler")]
            // profile_scope!("run_graph_creator");
            self.graph_creator.builder(&mut factory, world)
        };

        let graph = {
            // #[cfg(feature = "profiler")]
            // profile_scope!("build_graph");
            builder
                .build(&mut factory, self.families.as_mut().unwrap(), world)
                .unwrap()
        };

        self.graph = Some(graph);
    }

    fn run_graph(&mut self, world: &World) {
        let mut factory = world.fetch_mut::<Factory<B>>();
        factory.maintain(self.families.as_mut().unwrap());
        self.graph
            .as_mut()
            .unwrap()
            .run(&mut factory, self.families.as_mut().unwrap(), world)
    }
}

impl<'a, B, G> RunNow<'a> for ExtendedRenderingSystem<B, G>
where
    B: IExtendedBackend,
    G: GraphCreator<B>,
{
    fn run_now(&mut self, world: &'a World) {
        let rebuild = self.graph_creator.rebuild(world);
        if self.graph.is_none() || rebuild {
            self.rebuild_graph(world);
        }
        self.run_graph(world);
    }

    fn setup(&mut self, world: &mut World) {
        let config: amethyst::renderer::rendy::factory::Config = Default::default();
        let (factory, families): (Factory<B>, _) = amethyst::renderer::rendy::factory::init(config).unwrap();

        let queue_id = QueueId {
            family: families.family_by_index(0).id(),
            index: 0,
        };

        self.families = Some(families);
        world.insert(factory);
        world.insert(queue_id);

        SetupData::setup(world);
        let mat = create_default_mat::<B>(world);
        let textures = TextureCache::new();
        let meshes = MeshCache::new();
        let materials = MaterialCache::new();

        world.insert(MaterialDefaults(mat));
        world.insert(textures);
        world.insert(meshes);
        world.insert(materials);
    }

    fn dispose(mut self: Box<Self>, world: &mut World) {
        if let Some(graph) = self.graph.take() {
            let mut factory = world.fetch_mut::<Factory<B>>();
            log::debug!("Dispose graph");
            graph.dispose(&mut *factory, world);
        }

        log::debug!("Unload resources");
        if let Some(mut storage) = world.try_fetch_mut::<AssetStorage<Mesh>>() {
            storage.unload_all();
        }
        if let Some(mut storage) = world.try_fetch_mut::<AssetStorage<Texture>>() {
            storage.unload_all();
        }

        log::debug!("Drop families");
        drop(self.families);
    }
}

/// Asset processing system for `Mesh` asset type.
#[derive(Debug, derivative::Derivative)]
#[derivative(Default(bound = ""))]
pub struct MeshProcessorSystem<B: IExtendedBackend>(PhantomData<B>);
impl<'a, B: IExtendedBackend> System<'a> for MeshProcessorSystem<B> {
    type SystemData = (
        Write<'a, AssetStorage<Mesh>>,
        ReadExpect<'a, QueueId>,
        Read<'a, Time>,
        ReadExpect<'a, Arc<ThreadPool>>,
        Option<Read<'a, HotReloadStrategy>>,
        ReadExpect<'a, Factory<B>>,
    );

    fn run(
        &mut self,
        (mut mesh_storage, queue_id, time, pool, strategy, factory): Self::SystemData,
    ) {
        // #[cfg(feature = "profiler")]
        // profile_scope!("mesh_processor");

        mesh_storage.process(
            |b| {
                // #[cfg(feature = "profiler")]
                // profile_scope!("process_mesh");

                b.0.build(*queue_id, &factory)
                    .map(B::wrap_mesh_element)
                    .map(ProcessingState::Loaded)
                    .map_err(|e| e.compat().into())
            },
            time.frame_number(),
            &**pool,
            strategy.as_deref(),
        );
    }
}

/// Asset processing system for `Texture` asset type.
#[derive(Debug, derivative::Derivative)]
#[derivative(Default(bound = ""))]
pub struct TextureProcessorSystem<B: IExtendedBackend>(PhantomData<B>);
impl<'a, B: IExtendedBackend> System<'a> for TextureProcessorSystem<B> {
    type SystemData = (
        Write<'a, AssetStorage<Texture>>,
        ReadExpect<'a, QueueId>,
        Read<'a, Time>,
        ReadExpect<'a, Arc<ThreadPool>>,
        Option<Read<'a, HotReloadStrategy>>,
        WriteExpect<'a, Factory<B>>,
    );

    fn run(
        &mut self,
        (mut texture_storage, queue_id, time, pool, strategy, mut factory): Self::SystemData,
    ) {
        use amethyst::renderer::rendy::hal;
        // #[cfg(feature = "profiler")]
        // profile_scope!("texture_processor");

        texture_storage.process(
            |b| {
                // #[cfg(feature = "profiler")]
                // profile_scope!("process_texture");

                b.0.build(
                    ImageState {
                        queue: *queue_id,
                        stage: hal::pso::PipelineStage::VERTEX_SHADER
                            | hal::pso::PipelineStage::FRAGMENT_SHADER,
                        access: hal::image::Access::SHADER_READ,
                        layout: hal::image::Layout::ShaderReadOnlyOptimal,
                    },
                    &mut factory,
                )
                .map(B::wrap_texture)
                .map(ProcessingState::Loaded)
                .map_err(|e| e.compat().into())
            },
            time.frame_number(),
            &**pool,
            strategy.as_deref(),
        );
    }
}

fn create_default_mat<B: IExtendedBackend>(world: &mut World) -> Material {
    use amethyst::assets::Loader;
    use amethyst::renderer::mtl::TextureOffset;

    let loader = world.fetch::<Loader>();

    let diffuse = load_from_srgba(Srgba::new(0.5, 0.5, 0.5, 1.0));
    // let emission = load_from_srgba(Srgba::new(0.0, 0.0, 0.0, 0.0));
    // let normal = load_from_linear_rgba(LinSrgba::new(0.5, 0.5, 1.0, 1.0));
    // let metallic_roughness = load_from_linear_rgba(LinSrgba::new(0.0, 0.5, 0.0, 0.0));
    // let ambient_occlusion = load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));
    // let cavity = load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));

    let tex_storage = world.fetch();

    let diffuse = loader.load_from_data(diffuse.into(), (), &tex_storage);
    // let emission = loader.load_from_data(emission.into(), (), &tex_storage);
    // let normal = loader.load_from_data(normal.into(), (), &tex_storage);
    // let metallic_roughness = loader.load_from_data(metallic_roughness.into(), (), &tex_storage);
    // let ambient_occlusion = loader.load_from_data(ambient_occlusion.into(), (), &tex_storage);
    // let cavity = loader.load_from_data(cavity.into(), (), &tex_storage);

    Material {
        // alpha_cutoff: 0.01,
        diffuse,
        // emission,
        // normal,
        // metallic_roughness,
        // ambient_occlusion,
        // cavity,
        uv_offset: TextureOffset::default(),
    }
}
