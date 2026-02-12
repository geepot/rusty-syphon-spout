# Bindings verification

This document summarizes the C glue → Rust FFI → safe API mapping and the **exposed feature set** for both Syphon and Spout.

## Syphon (macOS) — exposed features

All public frame-sharing APIs from the Syphon framework are exposed:

- **Server directory:** shared directory, server count, server at index, servers matching name/app, match release; notification names (announce, update, retire).
- **Server description:** copy UUID, name, app name; retain/release.
- **Server options:** create, set bool/unsigned long, release; option keys (is_private, antialias, depth, stencil).
- **OpenGL server:** create, release, has_clients, server_description, publish_frame, bind_to_draw_frame, unbind_and_publish, stop, context, copy_name, set_name, new_frame_image.
- **OpenGL client:** create (with optional new-frame callback), release, is_valid, has_new_frame, new_frame_image, stop, context, server_description.
- **OpenGL image:** release, texture_name, texture_size.
- **Metal server:** create, release, has_clients, server_description, publish_frame, new_frame_image, stop, device, copy_name, set_name.
- **Metal client:** create (with optional new-frame callback), release, is_valid, has_new_frame, new_frame_image, stop, server_description.
- **Metal texture:** release.
- **CGL/GL helpers:** create_headless_context, destroy_context, make_current; create_texture_rectangle_rgba8, read_texture_rectangle_rgba8, delete_texture.

Older frameworks (single `SyphonServer`/`SyphonClient`/`SyphonImage`) are supported via glue compatibility; Metal APIs are stubbed when the framework has no Metal.

## Syphon (macOS) — glue/FFI

- **Glue:** `syphon_glue/syphon_glue.h` + `syphon_glue/syphon_glue.m`
- **FFI:** `include!(concat!(env!("OUT_DIR"), "/bindings.rs"))` — bindgen from the glue header with `allowlist_function("syphon_.*")`.
- **Safe API:** `src/safe.rs` wraps all `syphon_*` calls with null checks, `Option`, and ownership (e.g. `opt_cstr_to_string` frees C strings returned by `copy_*` with `libc::free`).

**Contract checks:**
- All `syphon_*` functions in the header are implemented in the glue and wrapped in `safe.rs`.
- Pointers returned as “caller must release” (e.g. `syphon_server_description_copy_*`, `syphon_opengl_server_copy_name`) are consumed with `opt_cstr_to_string` which calls `libc::free`.
- Opaque pointers (directory, server, client, image, options, description) are wrapped in structs that call the corresponding `*_release` / `*_retain` / `*_destroy` in `Drop` or when appropriate.

## Spout (Windows) — exposed features

Core SpoutLibrary frame-sharing and discovery APIs are exposed:

- **Sender:** set_name, send_texture, send_fbo, send_image, release, is_initialized, width, height, name, fps, frame.
- **Sender format:** set/get DXGI texture format.
- **Receiver:** set_name, receive_texture, receive_image, release, sender_name, is_frame_new, is_updated, is_connected, sender_width, sender_height, sender_format, sender_fps, sender_frame.
- **Bind shared texture (receiver):** bind_shared_texture, unbind_shared_texture, shared_texture_id.
- **Discovery:** sender_count, sender_name_at, find_sender_name, active_sender, set_active_sender, sender_info (width, height, share_handle, format).
- **Frame sync:** set_frame_sync, wait_frame_sync, enable_frame_sync, close_frame_sync, is_frame_sync_enabled.
- **Memory buffer:** write_memory_buffer, read_memory_buffer.
- **Buffer/CPU controls:** get/set buffer mode, get/set buffer count, max_senders, get/set cpu_mode.

Not exposed (utility/advanced): log/console utilities, MessageBox wrappers, registry/settings helpers, full timing/refresh controls, and 2.006 compatibility methods (CreateSender, UpdateSender, etc.). These can be added in the glue if needed.

## Spout (Windows) — glue/FFI

- **Glue:** `spout_glue/spout_glue.h` + `spout_glue/spout_glue.cpp` (real) or `spout_glue_stub.c` (macOS stub).
- **FFI:** bindgen with `allowlist_function("spout_.*")` and `allowlist_type("spout_handle")`.
- **Safe API:** `Spout` in `safe.rs` wraps all glue functions; `spout_sender_get_name` return value is freed via `spout_string_free` (same allocation domain); `SpoutSenderInfo` holds sender_info out params.

## Type consistency

- **Syphon:** `CGLContextObj`, `GLuint`, `GLenum` come from the header (and macOS SDK); safe API uses the same or `u32` where appropriate.
- **Spout:** `spout_handle` = `void*`; `bool` in C matches Rust `bool`; `unsigned int` → `u32`; buffer outputs use `char*` with fixed 256-byte buffers in safe API.

Both bindings are accurate: glue matches the underlying SDK/API, and the safe API correctly wraps the FFI with proper null handling, string ownership, and `Drop` behavior.

## Unit tests

Unit tests are platform-gated in `tests/platform_api.rs`:

- **macOS tests**
  - API-surface compile checks for Syphon-safe wrappers.
  - `cgl_create_headless_context` / `cgl_make_current` / `cgl_destroy_context` smoke test.
  - Notification-name query smoke tests.
- **Windows tests**
  - API-surface compile checks for all exposed `Spout` methods and types.
  - `SpoutSenderInfo` clone/field sanity test.

Suggested commands:

```bash
cargo test --all-targets
cargo test --target x86_64-apple-darwin --all-targets
cargo test --target x86_64-pc-windows-gnu --all-targets --no-run
```
