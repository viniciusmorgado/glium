# SafeGL

[![Build Status](https://github.com/glium/glium/actions/workflows/ci.yml/badge.svg)](https://github.com/glium/glium/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/tomaka/glium/badge.svg?branch=master&service=github)](https://coveralls.io/github/tomaka/glium?branch=master)
[![crates.io page](https://img.shields.io/crates/v/glium.svg)](https://crates.io/crates/glium)

SafeGL - A safe OpenGL wrapper and API-compatible fork of Glium

This project is a fork of [glium](https://github.com/glium/glium), maintained by
[Oxidian Studios](https://github.com/oxidianindustries), a video game company, with the intent of using it to develop
graphical tools for internal use. We are backporting a large number of [PRs](https://github.com/glium/glium/pulls)
and [bug fixes](https://github.com/glium/glium/issues) that have been left unattended in the original GitHub repository,
including a few PRs that we submitted ourselves, while preserving compatibility with existing projects made with glium
for as long as possible.

One of the main reasons why this fork exists is that we don't believe that bugs, PRs, and other project issues are being
addressed quickly enough by the original glium community. Even though the project is not officially discontinued,
maintenance by the surrounding community has slowed down over time.

If you want to understand how we ended up here, check the links below:
- [Glium post-mortem](https://users.rust-lang.org/t/glium-post-mortem/7063)
- [Original glium repo](https://github.com/glium/glium)

### Note on OpenGL versions

Although SafeGL supports a wide range of OpenGL versions thanks to its glium heritage, our focus will be on 
**OpenGL 4.6** for modern desktops, **OpenGL** ES 3.0 as a baseline for embedded devices, and **OpenGL ES 3.1** 
for optimizations and advanced features.

Other targets will be secondary, with support developed around these priorities. Still, PRs targeting other platforms 
are welcome and will be reviewed promptly.

SafeGL is an intermediate layer between OpenGL and your application. You still need to manually handle
the graphics pipeline, but without having to use OpenGL's old and error-prone API.

```toml
[dependencies]
safegl = "*"
```

Its objectives:
 - Be safe to use. Many aspects of OpenGL that can trigger a crash if misused are automatically handled by SafeGL.
 - Provide an API that enforces good practices such as RAII or stateless function calls.
 - Be compatible with OpenGL 4.6 for modern desktops and OpenGL ES 3.0 for embedded devices.
 - Avoid all OpenGL errors beforehand.
 - Produce optimized OpenGL function calls, and allow the user to easily use modern OpenGL techniques.

## [SafeGL Book - Building a renderer from scratch]()

SafeGL book are design to teach you to implement a Forward+ renderer with MSAA anti-aliasing using Rust, OpenGL 4.6 and 
slang shaders that compiles to GLSL.

## [SafeGL Developer Handbook]()

SafeGL developer handbook explain architectural, implementations and other decisions related with the development
of the SafeGL and aim to help new developers to contribute to the project,

## Why should I use SafeGL instead of raw OpenGL calls?

Easy to use:

 - Functions are higher level in glium than in OpenGL. Glium's API tries to be as Rusty as
   possible, and shouldn't be much different than using any other Rust library. Glium should
   allow you to do everything that OpenGL allows you to do, just through high-level
   functions. If something is missing, please open an issue.

 - You can directly pass vectors, matrices and images to glium instead of manipulating low-level
   data.

 - Thanks to glutin, glium is very easy to setup compared to raw OpenGL.

 - Glium provides easier ways to do common tasks. For example the `VertexBuffer` struct
   contains information about the vertex bindings, because you usually don't use several different
   bindings with the same vertex buffer. This reduces the overall complexity of OpenGL.

 - Glium handles framebuffer objects, samplers, and vertex array objects for you. You no longer
   need to create them explicitly as they are automatically created when needed and destroyed
   when their corresponding object is destroyed.

 - Glium is stateless. There are no `set_something()` functions in the entire library, and
   everything is done by parameter passing. The same set of function calls will always produce
   the same results, which greatly reduces the number of potential problems.

Safety:

 - Glium detects what would normally be errors or undefined behaviors in OpenGL, and panics,
   without calling `glGetError` which would be too slow. Examples include requesting a depth test
   when you don't have a depth buffer available, not binding any value to an attribute or uniform,
   or binding multiple textures with different dimensions to the same framebuffer.

 - If the OpenGL context triggers an error, then you have found a bug in glium. Please open
   an issue. Just like Rust does everything it can to avoid crashes, glium does everything
   it can to avoid OpenGL errors.

 - The OpenGL context is automatically handled by glium. You don't need to worry about thread
   safety, as it is forbidden to change the thread in which OpenGL objects operate. Glium also
   allows you to safely replace the current OpenGL context with another one that shares the same
   lists.

 - Glium enforces RAII. Creating a `Texture2d` struct creates a texture, and destroying the struct
   destroys the texture. It also uses Rust's borrow system to ensure that objects are still
   alive and in the right state when you use them. Glium provides the same guarantees with OpenGL
   objects that you have with regular objects in Rust.

 - High-level functions are much easier to use and thus less error-prone. For example there is
   no risk of making a mistake while specifying the names and offsets of your vertex attributes,
   since Glium automatically generates this data for you.

 - Robustness is automatically handled. If the OpenGL context is lost (because of a crash in the
   driver for example) then swapping buffers will return an error.

Compatibility:

 - In its default mode, Glium should be compatible with both OpenGL and OpenGL ES. If something
   doesn't work on OpenGL ES, please open an issue.

 - During initialization, Glium detects whether the context provides all the required
   functionality, and returns an `Err` if the device is too old. Glium tries to be as tolerant
   as possible, and should work with the majority of the OpenGL2-era devices.

 - Glium will attempt to use the latest, optimized versions of OpenGL functions. This includes
   buffer and texture immutable storage and direct state access. It will automatically fall back
   to older functions if they are not available.

 - Glium comes with a set of tests that you can run with `cargo test`. If your project/game
   doesn't work on specific hardware, you can try running Glium's tests on it to see what is wrong.

Performances:

 - State changes are optimized. The OpenGL state is only modified if the state actually differs.
   For example if you call `draw` with the `IfLess` depth test twice in a row, then
   `glDepthFunc(GL_LESS)` and `glEnable(GL_DEPTH_TEST)` will only be called the first time. If
   you then call `draw` with `IfGreater`, then only `glDepthFunc(GL_GREATER)` will be called.

 - Just like Rust is theoretically slower than C because of additional safety checks, glium is
   theoretically slower than well-prepared and optimized raw OpenGL calls. However in practice
   the difference is very low.

 - Fully optimized OpenGL code uses advanced techniques such as persistent mapping or bindless
   textures. These are hard to do and error-prone, but trivially easy to do with glium. You can
   easily get a huge performance boost just by doing the right function calls.

 - Since glium automatically avoids all OpenGL errors, you can safely use the `GL_KHR_no_error`
   extension when it is available. Using this extension should provide a good performance boost
   (but it is also very recent and not available anywhere for the moment).

Limitations:

 - Robustness isn't supported everywhere yet, so you can still get crashes if you do incorrect
   things in your shaders.

 - Glium gives you access to all the tools but doesn't prevent you from doing horribly slow
   things. Some knowledge of modern techniques is required if you want to reach maximum
   performances.

 - Glium pushes the Rust compiler to its limits. Stack overflows (inside the compiler),
   internal compiler errors, one-hour compile time, etc. happen more often than in smaller
   libraries.

 - Macros are yet work-in-progress; see
  [`glium-derive`](https://github.com/glium/glium_derive) for details.
