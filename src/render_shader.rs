// This module is gated under "shader-compiler" feature
use amethyst::renderer::rendy::{
    hal::pso::ShaderStageFlags,
    shader::{Shader, SpirvShader},
};
use shaderc::{self, ShaderKind, SourceLanguage};
use std::path::{Path};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

macro_rules! vk_make_version {
    ($major: expr, $minor: expr, $patch: expr) => {{
        let (major, minor, patch): (u32, u32, u32) = ($major, $minor, $patch);
        (major << 22) | (minor << 12) | patch
    }};
}

/// Shader loaded from a source in the filesystem.
#[derive(Clone, Copy, Debug)]
pub struct ShaderInfo<P, E> {
    path: P,
    kind: ShaderKind,
    lang: SourceLanguage,
    entry: E,
}

impl<P, E> ShaderInfo<P, E> {
    /// New shader.
    pub fn new(path: P, kind: ShaderKind, lang: SourceLanguage, entry: E) -> Self {
        ShaderInfo {
            path,
            kind,
            lang,
            entry,
        }
    }
}

impl<P, E> ShaderInfo<P, E>
where
    E: AsRef<str>,
{
    /// Precompile shader source code into Spir-V bytecode.
    pub fn precompile(&self) -> Result<SpirvShader, failure::Error>
    where
        Self: Shader,
    {
        Ok(SpirvShader::new(
            self.spirv()?.into_owned(),
            stage_from_kind(&self.kind),
            self.entry.as_ref(),
        ))
    }
}

impl<P, E> Shader for ShaderInfo<P, E>
where
    P: AsRef<std::path::Path> + std::fmt::Debug,
    E: AsRef<str>,
{
    fn spirv(&self) -> Result<std::borrow::Cow<'static, [u32]>, failure::Error> {
        let code = std::fs::read_to_string(&self.path)?;

        let artifact = shaderc::Compiler::new()
            .ok_or_else(|| failure::format_err!("Failed to init Shaderc"))?
            .compile_into_spirv(
                &code,
                self.kind,
                self.path.as_ref().to_str().ok_or_else(|| {
                    failure::format_err!("'{:?}' is not valid UTF-8 string", self.path)
                })?,
                self.entry.as_ref(),
                Some({
                    let mut ops = shaderc::CompileOptions::new()
                        .ok_or_else(|| failure::format_err!("Failed to init Shaderc"))?;
                    ops.set_include_callback(|header, _include_type, path, _depth| {
                        let path = Path::new(path)
                            .parent().expect("Must have a parent dir")
                            .join(Path::new(header));
                        let path_str = path.to_str().expect("Must be a valid path");

                        let mut s = String::new();
                        File::open(path_str).expect("File must exist")
                            .read_to_string(&mut s).expect("File must contain data");

                        let resolved = shaderc::ResolvedInclude {
                            resolved_name: String::from_str(path_str).expect(""),
                            content: s,
                        };
                        Result::Ok(resolved)
                    });
                    ops.set_target_env(shaderc::TargetEnv::Vulkan, vk_make_version!(1, 0, 0));
                    ops.set_source_language(self.lang);
                    ops.set_generate_debug_info();
                    ops.set_optimization_level(shaderc::OptimizationLevel::Performance);
                    ops
                })
                .as_ref(),
            )?;

        Ok(std::borrow::Cow::Owned(artifact.as_binary().into()))
    }

    fn entry(&self) -> &str {
        self.entry.as_ref()
    }

    fn stage(&self) -> ShaderStageFlags {
        stage_from_kind(&self.kind)
    }
}

fn stage_from_kind(kind: &ShaderKind) -> ShaderStageFlags {
    match kind {
        ShaderKind::Vertex => ShaderStageFlags::VERTEX,
        ShaderKind::Fragment => ShaderStageFlags::FRAGMENT,
        ShaderKind::Geometry => ShaderStageFlags::GEOMETRY,
        ShaderKind::TessEvaluation => ShaderStageFlags::HULL,
        ShaderKind::TessControl => ShaderStageFlags::DOMAIN,
        ShaderKind::Compute => ShaderStageFlags::COMPUTE,
        _ => panic!("Invalid shader type specified"),
    }
}

pub type PathBufShaderInfo = ShaderInfo<std::path::PathBuf, &'static str>;
