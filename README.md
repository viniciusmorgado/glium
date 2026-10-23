# SafeGL

[![Build Status](https://github.com/glium/glium/actions/workflows/ci.yml/badge.svg)](https://github.com/glium/glium/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/tomaka/glium/badge.svg?branch=master&service=github)](https://coveralls.io/github/tomaka/glium?branch=master)
[![crates.io page](https://img.shields.io/crates/v/glium.svg)](https://crates.io/crates/glium)

SafeGL - A safe OpenGL wrapper for

SafeGL is an intermediate layer between OpenGL and your application. You still need to manage the
graphics pipeline yourself, but you don't have to deal with OpenGL's old, error-prone API or with
manual memory management.

This project is a fork of [glium](https://github.com/glium/glium), maintained by
[Oxidian Studios](https://github.com/oxidianindustries), a video game company, with the intent of using it to develop
graphical tools for internal use. We are backporting a large number of [PRs](https://github.com/glium/glium/pulls)
and [bug fixes](https://github.com/glium/glium/issues) that have been left unattended in the original GitHub repository,
including a few PRs that we submitted ourselves.

One of the main reasons why this fork exists is that we don't believe that bugs, PRs, and other project issues are being
addressed quickly enough by the original glium community. Even though the project is not officially discontinued,
maintenance by the surrounding community has slowed down over time.

If you want to understand how we ended up here, check the links below:
- [Glium post-mortem](https://users.rust-lang.org/t/glium-post-mortem/7063)
- [Original glium repo](https://github.com/glium/glium)

### Note on OpenGL versions

Glium, SafeGL's predecessor, supports a wide range of OpenGL versions. SafeGL narrows that scope
to three tiers:

| Platform                    | Target             |
|-----------------------------|--------------------|
| Windows and Linux desktops  | **OpenGL 4.6**     |
| macOS                       | **OpenGL 4.1**     |
| Embedded devices            | **OpenGL ES 3.0**  |

> [!NOTE]
> OpenGL 4.2+ features are excluded at compile time on macOS, so using them there is a compile
> error rather than a runtime failure.

```toml
[dependencies]
safegl = "0.36.0"
```

Its objectives:
 - Be safe to use. Many aspects of OpenGL that can trigger a crash if misused are automatically handled by SafeGL.
 - Provide an API that enforces good practices such as RAII or stateless function calls.
 - Be compatible with OpenGL 4.6 for modern desktops and OpenGL ES 3.0 for embedded devices.
 - Avoid all OpenGL errors beforehand.
 - Produce optimized OpenGL function calls, and allow the user to easily use modern OpenGL techniques.

## Why SafeGL matters for Oxidian

At Oxidian, we want to use SafeGL to build practical internal tools such as asset viewers,
texture inspectors, material previews, and thumbnail generators. These tools address everyday
production needs while giving programmers new to computer graphics manageable projects with
useful results.

SafeGL's safe abstractions let newcomers learn the graphics pipeline by working with geometry,
textures, shaders, and draw calls, while handling much of the resource lifetime and state management
that makes raw OpenGL error-prone. They can develop these foundations without first tackling
Vulkan's explicit GPU memory management or many of the manual resource management pitfalls of
raw OpenGL.

For us, this also creates a practical path for onboarding and developing graphics programmers:
new team members contribute useful tools as they learn, build confidence through hands-on work,
and prepare to take on more demanding rendering tasks. Investing in SafeGL supports both our
production workflow and the growth of our team.

## [SafeGL Book - Building a renderer from scratch]()

SafeGL book are design to teach you to implement a Forward+ renderer with MSAA anti-aliasing using Rust, OpenGL 4.6 and 
slang shaders that compiles to GLSL.

## [SafeGL Developer Handbook]()

SafeGL developer handbook explain architectural, implementations and other decisions related with the development
of the SafeGL and aim to help new developers to contribute to the project,

## When should I use SafeGL or raw OpenGL?

SafeGL is a good fit when you want to build a renderer with a Rust API that handles much of
OpenGL's resource management and state bookkeeping. You still design the rendering pipeline,
write shaders, and choose how to organize your data and draw calls.

Lower-level bindings such as [`gl`](https://docs.rs/gl/) are a better fit when you need direct
control over OpenGL calls and are prepared to manage their safety requirements yourself.

### Choose SafeGL when

 - You want typed buffers, textures, and uniforms, with RAII to manage resource lifetimes
   and less manual work configuring vertex attributes and bindings.

 - You want to describe draw parameters explicitly while SafeGL manages framebuffer objects,
   samplers, vertex array objects, and cached OpenGL state to avoid redundant state changes.

 - You want validation that catches many invalid resource combinations and draw configurations
   before they reach OpenGL, reducing the amount of unsafe code in your application.

 - You want the library to handle supported differences between OpenGL and OpenGL ES, including
   capability checks and fallback paths where available. Individual features still depend on
   your context, hardware, and driver.

 - You want to focus on rendering techniques and application behavior while relying on an
   existing abstraction for common OpenGL operations.

### Choose lower-level bindings when

 - You need OpenGL features or extensions that SafeGL does not expose, using bindings that
   include those entry points.

 - You are building your own graphics abstraction and need to define its resource ownership,
   state tracking, synchronization, and error-handling rules.

 - You are integrating with an existing renderer or external library that needs to control
   the same OpenGL context and state directly.

 - Profiling has identified a bottleneck in SafeGL's abstraction, and you need precise control
   over the underlying calls, bindings, or resource updates to address it.

 - You want to learn or experiment with the OpenGL API directly, including its state model
   and resource management requirements.

With raw bindings, you are responsible for context setup and current-context rules, function
loading, resource cleanup, valid state, synchronization, and feature availability. The bindings
provide access to OpenGL; these responsibilities remain with your application.

### Performance and interoperability

SafeGL caches state and uses optimized OpenGL paths where supported. Raw calls give you more
control, but performance depends on how you use that control. Measure your workload on the
hardware you intend to support before choosing lower-level bindings for performance reasons.

Mixing SafeGL with raw calls requires care: SafeGL tracks OpenGL state internally. When using
`exec_in_context`, any OpenGL state changed by your code must be restored before the closure
returns. For integrations that require frequent direct state manipulation, owning the OpenGL
layer yourself may be simpler.

Limitations:

 - Robustness isn't supported everywhere yet, so you can still get crashes if you do incorrect
   things in your shaders.

 - SafeGL gives you access to all the tools but doesn't prevent you from doing horribly slow
   things. Some knowledge of modern techniques is required if you want to reach maximum
   performances.

 - SafeGL pushes the Rust compiler to its limits. Stack overflows (inside the compiler),
   internal compiler errors, one-hour compile time, etc. happen more often than in smaller
   libraries.

 - Macros are yet work-in-progress; see
  [`glium-derive`](https://github.com/glium/glium_derive) for details.
