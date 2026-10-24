use std::cmp::Ordering;

/// Describes a version.
///
/// A version can only be compared to another version if they belong to the same API.
/// For example, both `Version(Gl, 3, 0) >= Version(GlEs, 3, 0)` and `Version(GlEs, 3, 0) >=
/// Version(Gl, 3, 0)` return `false`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Version(pub Api, pub u8, pub u8);

/// Describes an OpenGL-related API.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Api {
    /// Regular OpenGL.
    Gl,
    /// OpenGL embedded system.
    GlEs,
}

impl PartialOrd for Version {
    #[inline]
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        if self.0 != other.0 {
            return None;
        }

        match self.1.cmp(&other.1) {
            Ordering::Equal => Some(self.2.cmp(&other.2)),
            a => Some(a),
        }
    }
}

/// Returns true if the crate is compiled for WebGL (`wasm32` without Emscripten).
///
/// WebGL 2 is reported as OpenGL ES 3.0, but it lacks or rejects several things that native
/// OpenGL ES 3.0 has (buffer mapping, blocking fence waits, `PRIMITIVE_RESTART_FIXED_INDEX`,
/// ...), and the table of extra entry points (`gl::Extra`) is never loaded.
#[inline]
pub(crate) const fn is_webgl() -> bool {
    cfg!(all(target_family = "wasm", not(target_os = "emscripten")))
}

/// Obtains the OpenGL version of a `glow` context.
///
/// WebGL 2 is reported by glow as OpenGL ES 3.0.
pub fn get_gl_version(gl: &glow::Context) -> Version {
    use glow::HasContext;

    let version = gl.version();
    let api = if version.is_embedded {
        Api::GlEs
    } else {
        Api::Gl
    };
    let clamp = |value: u32| u8::try_from(value).unwrap_or(u8::MAX);

    Version(api, clamp(version.major), clamp(version.minor))
}

/// Given an API version, this function returns the GLSL version that the implementation is
/// required to support.
///
/// # Panic
///
/// Panics if the version is invalid or is not supposed to support a GLSL version.
pub fn get_supported_glsl_version(gl_version: &Version) -> Version {
    match gl_version.0 {
        Api::Gl => {
            // since OpenGL 3.3: glsl versions match gl version, just return a copy
            if *gl_version >= Version(gl_version.0, 3, 3) {
                return *gl_version;
            }

            // match to detect invalid versions
            match *gl_version {
                Version(a, 2, 0) => Version(a, 1, 1),
                Version(a, 2, 1) => Version(a, 1, 2),
                Version(a, 3, 0) => Version(a, 1, 3),
                Version(a, 3, 1) => Version(a, 1, 4),
                Version(a, 3, 2) => Version(a, 1, 5),
                _ => panic!("no corresponding glsl version exists"),
            }
        }
        Api::GlEs => {
            // since OpenGL ES 3.0: glsl versions match gl version, just return a copy
            if *gl_version >= Version(gl_version.0, 3, 0) {
                return *gl_version;
            }

            // only other valid GLES version is 2.0
            if *gl_version == Version(gl_version.0, 2, 0) {
                Version(Api::GlEs, 1, 0)
            } else {
                panic!("no corresponding glsl version exists")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_supported_glsl_version, Api, Version};

    macro_rules! assert_versions {
        ( $api:path, $gl_major:expr, $gl_minor:expr => $glsl_major:expr, $glsl_minor:expr) => {
            assert_eq!(
                get_supported_glsl_version(&Version($api, $gl_major, $gl_minor)),
                Version($api, $glsl_major, $glsl_minor)
            );
        };
    }

    #[test]
    fn valid_gl_versions() {
        // irregular versions
        assert_versions!(Api::Gl, 2, 0 => 1, 1);
        assert_versions!(Api::Gl, 2, 1 => 1, 2);
        assert_versions!(Api::Gl, 3, 0 => 1, 3);
        assert_versions!(Api::Gl, 3, 1 => 1, 4);
        assert_versions!(Api::Gl, 3, 2 => 1, 5);

        // test a few regular versions
        assert_versions!(Api::Gl, 3, 3 => 3, 3);
        assert_versions!(Api::Gl, 4, 0 => 4, 0);
        assert_versions!(Api::Gl, 4, 5 => 4, 5);
    }

    #[test]
    fn valid_gles_versions() {
        // only irregular version
        assert_versions!(Api::GlEs, 2, 0 => 1, 0);

        // some regular versions
        assert_versions!(Api::GlEs, 3, 0 => 3, 0);
        assert_versions!(Api::GlEs, 3, 1 => 3, 1);
    }

    #[test]
    #[should_panic]
    fn invalid_gl_version() {
        get_supported_glsl_version(&Version(Api::Gl, 1, 0));
    }

    #[test]
    #[should_panic]
    fn invalid_gles_version() {
        get_supported_glsl_version(&Version(Api::GlEs, 1, 0));
    }
}
