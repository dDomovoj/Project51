use std::collections::HashMap;
use amethyst::assets::Handle;
use amethyst::renderer::types::Texture;
use amethyst::ecs::World;
use amethyst::ecs::{WorldExt};

use crate::render_mesh::Mesh;
use crate::render_material::Material;

macro_rules! impl_cache {
    ($name:ident, $type:ident, $elt:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            hash_map: HashMap<u32, Handle<$type>>
        }

        impl $name {
        pub fn new() -> $name {
            $name { hash_map: HashMap::default() }
        }
        pub fn cached(&self, id: u32) -> Option::<Handle<$type>> {
            if let Some($elt) = self.hash_map.get(&id) {
                Some((*$elt).clone())
            }
            else {
                None
            }
        }
        pub fn cache(&mut self, $elt: Handle<$type>, id: u32) {
            self.hash_map.insert(id, $elt);
        }

        pub fn item<'a, F>(id: u32, world: &mut World, f: F) -> Handle<$type> 
        where F: FnOnce(&mut World) -> Handle<$type>
        {
                let cached = world.read_resource::<$name>().cached(id);
                let $elt = {
                    if let Some(cached) = cached {
                        cached
                    } else {
                        let new = f(world);
                        {
                            let mut writer = world.write_resource::<$name>();
                            writer.cache(new.clone(), id);
                        }
                        new
                    }
                };
                $elt
            }
        }
    };
}

impl_cache!(TextureCache, Texture, texture);
impl_cache!(MaterialCache, Material, material);
impl_cache!(MeshCache, Mesh, mesh_element);
