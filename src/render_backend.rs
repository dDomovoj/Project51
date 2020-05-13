use amethyst::renderer::types::Backend;
use amethyst::renderer::rendy;

use crate::render_mesh::{Mesh, GenericMesh};

// /// Extension of the rendy Backend trait.
// pub trait Backend: rendy::hal::Backend {
//     /// Unwrap a Backend to a rendy `Mesh`
//     fn unwrap_mesh(mesh: &Mesh) -> Option<&BackendMesh<Self>>;
//     /// Unwrap a Backend to a rendy `Texture`
//     fn wrap_mesh(mesh: BackendMesh<Self>) -> Mesh;

//     // /// Wrap a rendy `Texture` to its Backend generic.
//     // fn wrap_texture(texture: rendy::texture::Texture<Self>) -> Texture;
//     // fn unwrap_texture(texture: &Texture) -> Option<&rendy::texture::Texture<Self>>;
//     // /// Wrap a rendy `Mesh` to its Backend generic.
// }

pub trait IExtendedBackend: Backend {
    fn unwrap_mesh_element(mesh: &Mesh) -> Option<&GenericMesh<Self>>;

    fn wrap_mesh_element(mesh: GenericMesh<Self>) -> Mesh;
}

// macro_rules! impl_backends {
//     ($($variant:ident, $feature:literal, $backend:ty;)*) => {

//         // impl_single_default!($([$feature, $backend]),*);
//         // static_assertions::assert_cfg!(
//         //     any($(feature = $feature),*),
//         //     concat!("You must specify at least one graphical backend feature: ", stringify!($($feature),* "See the wiki article https://book.amethyst.rs/stable/appendices/c_feature_gates.html#graphics-features for more details."))
//         // );

//         // /// Backend wrapper.
//         // #[derive(Debug)]
//         // pub enum BackendVariant {
//         //     $(
//         //         #[cfg(feature = $feature)]
//         //         #[doc = "Backend Variant"]
//         //         $variant,
//         //     )*
//         // }

//         /// Mesh wrapper.
//         #[derive(Debug)]
//         pub enum Mesh {
//             $(
//                 #[cfg(feature = $feature)]
//                 #[doc = "Mesh Variant"]
//                 $variant(BackendMesh<$backend>),
//             )*
//         }

//         // /// Texture wrapper.
//         // #[derive(Debug)]
//         // pub enum Texture {
//         //     $(
//         //         #[cfg(feature = $feature)]
//         //         #[doc = "Texture Variant"]
//         //         $variant(rendy::texture::Texture<$backend>),
//         //     )*
//         // }

//         $(
//             #[cfg(feature = $feature)]
//             impl ExtendedBackend for $backend {
//                 #[inline]
//                 #[allow(irrefutable_let_patterns)]
//                 fn unwrap_custom_mesh(mesh: &Mesh) -> Option<&BackendMesh<Self>> {
//                     if let Mesh::$variant(inner) = mesh {
//                         Some(inner)
//                     } else {
//                         None
//                     }
//                 }
//                 #[inline]
//                 fn wrap_custom_mesh(mesh: BackendMesh<Self>) -> Mesh {
//                     Mesh::$variant(mesh)
//                 }
//                 // #[inline]
//                 // #[allow(irrefutable_let_patterns)]
//                 // fn unwrap_texture(texture: &Texture) -> Option<&rendy::texture::Texture<Self>> {
//                 //     if let Texture::$variant(inner) = texture {
//                 //         Some(inner)
//                 //     } else {
//                 //         None
//                 //     }
//                 // }
//                 // #[inline]
//                 // fn wrap_texture(texture: rendy::texture::Texture<Self>) -> Texture {
//                 //     Texture::$variant(texture)
//                 // }
//             }
//         )*
//     };
// }

// Create `DefaultExtendedBackend` type alias only when exactly one backend is selected.
// macro_rules! impl_single_default {
// ( $([$feature:literal, $backend:ty]),* ) => {
//     impl_single_default!(@ (), ($([$feature, $backend])*));
// };
// (@ ($($prev:literal)*), ([$cur:literal, $backend:ty]) ) => {
//     #[cfg(all( feature = $cur, not(any($(feature = $prev),*)) ))]
//     #[doc = "Default backend"]
//     pub type DefaultExtendedBackend = $backend;
// };
// (@ ($($prev:literal)*), ([$cur:literal, $backend:ty] $([$nf:literal, $nb:ty])*) ) => {
//     #[cfg(all( feature = $cur, not(any($(feature = $prev,)* $(feature = $nf),*)) ))]
//     #[doc = "Default backend"]
//     pub type DefaultExtendedBackend = $backend;

//     impl_single_default!(@ ($($prev)* $cur), ($([$nf, $nb])*) );
// };
// }

// impl_backends!(
//     // DirectX 12 is currently disabled because of incomplete gfx-hal support for it.
//     // It will be re-enabled when it actually works.
//     // Dx12, "dx12", rendy::dx12::Backend;
//     Metal, "metal", rendy::metal::Backend;
//     Vulkan, "vulkan", rendy::vulkan::Backend;
//     Empty, "empty", rendy::empty::Backend;
// );

pub type DefaultExtendedBackend = rendy::metal::Backend;

impl IExtendedBackend for rendy::metal::Backend {
    #[inline]
    #[allow(irrefutable_let_patterns)]
    fn unwrap_mesh_element(mesh: &Mesh) -> Option<&GenericMesh<Self>> {
        if let Mesh::Metal(inner) = mesh {
            Some(inner)
        } else {
            None
        }
    }
    #[inline]
    fn wrap_mesh_element(mesh: GenericMesh<Self>) -> Mesh {
        Mesh::Metal(mesh)
    }
}