use crate::gl;
#[cfg(feature = "gl46")]
use crate::handles::RawName;
use crate::version::Api;
use crate::version::Version;

use crate::backend::Facade;
use crate::context::Context;
use crate::CapabilitiesSource;
use crate::ContextExt;

use std::rc::Rc;
#[cfg(feature = "gl46")]
use std::{ffi, ptr};

use glow::HasContext;

use crate::GlObject;

#[cfg(feature = "gl46")]
use crate::program::SpirvEntryPoint;
use crate::program::{ProgramCreationError, ShaderType};

/// A single, compiled but unlinked, shader.
pub struct Shader {
    context: Rc<Context>,
    id: glow::Shader,
}

impl GlObject for Shader {
    type Id = glow::Shader;

    #[inline]
    fn get_id(&self) -> glow::Shader {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        let ctxt = self.context.make_current();

        unsafe {
            ctxt.glow.delete_shader(self.id);
        }
    }
}

/// Builds an individual shader.
pub fn build_shader<F: ?Sized>(
    facade: &F,
    shader_type: gl::types::GLenum,
    source_code: &str,
) -> Result<Shader, ProgramCreationError>
where
    F: Facade,
{
    unsafe {
        let ctxt = facade.get_context().make_current();

        if ctxt.capabilities.supported_glsl_versions.is_empty() {
            return Err(ProgramCreationError::CompilationNotSupported);
        }

        if !check_shader_type_compatibility(&ctxt, shader_type) {
            return Err(ProgramCreationError::ShaderTypeNotSupported);
        }

        let shader = match ctxt.glow.create_shader(shader_type) {
            Ok(shader) => shader,
            Err(_) => return Err(ProgramCreationError::ShaderTypeNotSupported),
        };

        ctxt.glow.shader_source(shader, source_code);

        // compiling
        {
            ctxt.report_debug_output_errors.set(false);
            ctxt.glow.compile_shader(shader);
            ctxt.report_debug_output_errors.set(true);
        }

        // checking compilation success by reading a flag on the shader
        if ctxt.glow.get_shader_compile_status(shader) {
            Ok(Shader {
                context: facade.get_context().clone(),
                id: shader,
            })
        } else {
            // compilation error
            let msg = ctxt.glow.get_shader_info_log(shader);

            Err(ProgramCreationError::CompilationError(
                msg,
                ShaderType::from_opengl_type(shader_type),
            ))
        }
    }
}

/// Builds an individual shader from a SPIR-V binary.
#[cfg(feature = "gl46")]
pub fn build_spirv_shader<F: ?Sized>(
    facade: &F,
    shader_type: gl::types::GLenum,
    spirv: &SpirvEntryPoint,
) -> Result<Shader, ProgramCreationError>
where
    F: Facade,
{
    unsafe {
        let ctxt = facade.get_context().make_current();

        if ctxt.capabilities.supported_glsl_versions.is_empty() {
            return Err(ProgramCreationError::CompilationNotSupported);
        }

        if !check_shader_type_compatibility(&ctxt, shader_type) {
            return Err(ProgramCreationError::ShaderTypeNotSupported);
        }

        let shader = if ctxt.version >= &Version(Api::Gl, 4, 6)
            || (ctxt.version.0 == Api::Gl && ctxt.extensions.gl_arb_gl_spirv)
        {
            ctxt.glow.create_shader(shader_type)
        } else {
            return Err(ProgramCreationError::CompilationNotSupported);
        };

        let shader = match shader {
            Ok(shader) => shader,
            Err(_) => return Err(ProgramCreationError::ShaderTypeNotSupported),
        };
        let id = shader.raw_name();

        let binary = &spirv.binary;
        let entry_point = ffi::CString::new(spirv.entry_point.as_bytes()).unwrap();

        const GL_SHADER_BINARY_FORMAT_SPIR_V: gl::types::GLenum = 0x9551;

        ctxt.extra.ShaderBinary(
            1,
            &id,
            GL_SHADER_BINARY_FORMAT_SPIR_V,
            binary.as_ptr() as _,
            binary.len() as _,
        );

        // compiling
        {
            ctxt.report_debug_output_errors.set(false);

            ctxt.extra
                .SpecializeShader(id, entry_point.as_ptr() as _, 0, ptr::null(), ptr::null());

            ctxt.report_debug_output_errors.set(true);
        }

        // checking compilation success by reading a flag on the shader
        if ctxt.glow.get_shader_compile_status(shader) {
            Ok(Shader {
                context: facade.get_context().clone(),
                id: shader,
            })
        } else {
            // compilation error
            let msg = ctxt.glow.get_shader_info_log(shader);

            Err(ProgramCreationError::CompilationError(
                msg,
                ShaderType::from_opengl_type(shader_type),
            ))
        }
    }
}

pub fn check_shader_type_compatibility<C: ?Sized>(ctxt: &C, shader_type: gl::types::GLenum) -> bool
where
    C: CapabilitiesSource,
{
    match shader_type {
        gl::VERTEX_SHADER | gl::FRAGMENT_SHADER => (),
        gl::GEOMETRY_SHADER => {
            if ctxt.get_version().0 != Api::Gl
                && !(ctxt.get_version() >= &Version(Api::GlEs, 3, 2))
                && !ctxt.get_extensions().gl_arb_geometry_shader4
                && !ctxt.get_extensions().gl_ext_geometry_shader4
                && !ctxt.get_extensions().gl_ext_geometry_shader
                && !ctxt.get_extensions().gl_oes_geometry_shader
            {
                return false;
            }
        }
        gl::TESS_CONTROL_SHADER | gl::TESS_EVALUATION_SHADER => {
            if ctxt.get_version().0 != Api::Gl
                && !(ctxt.get_version() >= &Version(Api::GlEs, 3, 2))
                && !ctxt.get_extensions().gl_arb_tessellation_shader
                && !ctxt.get_extensions().gl_oes_tessellation_shader
            {
                return false;
            }
        }
        gl::COMPUTE_SHADER => {
            if !(ctxt.get_version() >= &Version(Api::Gl, 4, 3))
                && !(ctxt.get_version() >= &Version(Api::GlEs, 3, 1))
                && !ctxt.get_extensions().gl_arb_compute_shader
            {
                return false;
            }
        }
        _ => unreachable!(),
    };

    true
}
