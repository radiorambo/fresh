// Editor library - exposes all core modules for testing

// Initialize V8 platform once per process
//
// This is required because:
// 1. V8 platform must be initialized before any JsRuntime instances are created
//    See: https://docs.rs/deno_core/latest/deno_core/struct.JsRuntime.html#method.init_platform
// 2. V8 platform initialization is process-wide and cannot be done more than once
//    See: https://v8.github.io/api/head/classv8_1_1V8.html (V8::Dispose is permanent)
// 3. Multiple Editor instances can be created sequentially (e.g., in tests) as long as
//    they share the same V8 platform initialized once at process startup
//    See: https://docs.rs/deno_core/latest/deno_core/struct.JsRuntime.html
//
// Without this, creating multiple Editor instances sequentially causes segfaults
// because V8 cannot be reinitialized after disposal.
use std::sync::Once;
static INIT_V8: Once = Once::new();

/// Initialize V8 platform exactly once per process
/// This is called automatically when the library is loaded
fn init_v8_platform() {
    INIT_V8.call_once(|| {
        deno_core::JsRuntime::init_platform(None);
    });
}

// Call V8 initialization when library loads using .init_array section
// This ensures initialization happens before any Editor instances are created
#[used]
#[link_section = ".init_array"]
static INITIALIZER: extern "C" fn() = {
    extern "C" fn init() {
        init_v8_platform();
    }
    init
};

// Core modules at root level
pub mod config;
pub mod state;

// Organized modules
pub mod app;
pub mod input;
pub mod model;
pub mod primitives;
pub mod services;
pub mod view;
