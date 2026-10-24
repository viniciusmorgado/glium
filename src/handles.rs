//! Raw OpenGL names of glow's handle types.
//!
//! Every stored object name is a glow handle. The few entry points that glow doesn't provide
//! (`gl::Extra`) still take raw names, so handles are converted at those call sites. On native
//! targets glow's handles wrap the raw name; on WebGL they are opaque keys, but `gl::Extra` is
//! never called there.

use crate::gl;

/// A glow object handle that can be turned back into its raw OpenGL name.
pub trait RawName {
    /// Returns the raw OpenGL name of the object.
    fn raw_name(self) -> gl::types::GLuint;
}

// The impls name glow's concrete handle types: its `glow::Buffer`, `glow::Texture`, ... aliases
// are associated-type projections, which coherence treats as possibly overlapping.
macro_rules! impl_raw_name {
    ($($native:ident / $web:ident),+) => {
        $(
            #[cfg(any(not(target_family = "wasm"), target_os = "emscripten"))]
            impl RawName for glow::$native {
                #[inline]
                fn raw_name(self) -> gl::types::GLuint {
                    self.0.get()
                }
            }

            #[cfg(all(target_family = "wasm", not(target_os = "emscripten")))]
            impl RawName for glow::$web {
                #[inline]
                fn raw_name(self) -> gl::types::GLuint {
                    let _ = self;
                    unreachable!("raw OpenGL names don't exist on WebGL")
                }
            }
        )+
    };
}

impl_raw_name!(
    NativeBuffer / WebBufferKey,
    NativeTexture / WebTextureKey,
    NativeFramebuffer / WebFramebufferKey,
    NativeRenderbuffer / WebRenderbufferKey,
    NativeProgram / WebProgramKey,
    NativeShader / WebShaderKey,
    NativeVertexArray / WebVertexArrayKey,
    NativeQuery / WebQueryKey,
    NativeSampler / WebSamplerKey,
    NativeTransformFeedback / WebTransformFeedbackKey
);

/// Returns the raw sync object of a glow fence.
#[cfg(any(not(target_family = "wasm"), target_os = "emscripten"))]
#[inline]
pub fn raw_sync(fence: glow::Fence) -> gl::types::GLsync {
    fence.0 as _
}

/// Returns the raw sync object of a glow fence.
#[cfg(all(target_family = "wasm", not(target_os = "emscripten")))]
#[inline]
pub fn raw_sync(fence: glow::Fence) -> gl::types::GLsync {
    let _ = fence;
    unreachable!("raw OpenGL sync objects don't exist on WebGL")
}

/// Maps the `i32` uniform locations that SafeGL exposes to glow's uniform locations.
///
/// On native targets the `i32` is the OpenGL uniform location itself. On WebGL locations are
/// opaque objects, so the `i32` is an index into a table filled during program reflection.
#[derive(Default)]
pub struct UniformLocations {
    #[cfg(all(target_family = "wasm", not(target_os = "emscripten")))]
    table: Vec<glow::UniformLocation>,
}

impl UniformLocations {
    /// Registers a location returned by `glGetUniformLocation` and returns its `i32` form, or
    /// `-1` if the uniform has no location.
    pub fn register(&mut self, location: Option<glow::UniformLocation>) -> gl::types::GLint {
        match location {
            None => -1,
            #[cfg(any(not(target_family = "wasm"), target_os = "emscripten"))]
            Some(location) => location.0 as gl::types::GLint,
            #[cfg(all(target_family = "wasm", not(target_os = "emscripten")))]
            Some(location) => {
                self.table.push(location);
                (self.table.len() - 1) as gl::types::GLint
            }
        }
    }

    /// Returns the glow location for an `i32` location obtained from `register`.
    pub fn get(&self, location: gl::types::GLint) -> glow::UniformLocation {
        #[cfg(any(not(target_family = "wasm"), target_os = "emscripten"))]
        {
            glow::NativeUniformLocation(location as u32)
        }
        #[cfg(all(target_family = "wasm", not(target_os = "emscripten")))]
        {
            self.table[location as usize].clone()
        }
    }
}
