//! Safe Rust API for Syphon (OpenGL and Metal server/client, server directory).
//!
//! OpenGL: CGL context and GL usage must follow Syphon's and macOS's rules.
//! Metal: pass `MTLDevice`/`MTLTexture`/`MTLCommandBuffer` pointers (e.g. from the `metal` crate).

use std::ptr::NonNull;
#[cfg(target_os = "macos")]
use std::ffi::CStr;
#[cfg(target_os = "macos")]
use std::os::raw::c_char;

#[cfg(target_os = "macos")]
use crate::ffi;
#[cfg(target_os = "windows")]
use crate::ffi as spout_ffi;

/// CGL context (from OpenGL/OpenGL.h). On macOS this is the real type from the FFI; elsewhere a placeholder.
#[cfg(target_os = "macos")]
pub type CGLContextObj = crate::ffi::CGLContextObj;
#[cfg(not(target_os = "macos"))]
pub type CGLContextObj = *mut std::ffi::c_void;

/// OpenGL texture target for rectangle textures (Syphon uses this).
pub const GL_TEXTURE_RECTANGLE: u32 = 0x84F5;

/// Create a headless CGL context for offscreen OpenGL (e.g. tests). Caller must destroy with `cgl_destroy_context`.
#[cfg(target_os = "macos")]
pub fn cgl_create_headless_context() -> Option<CGLContextObj> {
    let ctx = unsafe { ffi::syphon_cgl_create_headless_context() };
    if ctx.is_null() {
        None
    } else {
        Some(ctx)
    }
}
#[cfg(not(target_os = "macos"))]
pub fn cgl_create_headless_context() -> Option<CGLContextObj> {
    None
}

/// Destroy a CGL context created with `cgl_create_headless_context`.
#[cfg(target_os = "macos")]
pub fn cgl_destroy_context(ctx: CGLContextObj) {
    if !ctx.is_null() {
        unsafe { ffi::syphon_cgl_destroy_context(ctx) };
    }
}
#[cfg(not(target_os = "macos"))]
pub fn cgl_destroy_context(_ctx: CGLContextObj) {}

/// Make the given CGL context current on this thread.
#[cfg(target_os = "macos")]
pub fn cgl_make_current(ctx: CGLContextObj) {
    unsafe { ffi::syphon_cgl_make_current(ctx) };
}
#[cfg(not(target_os = "macos"))]
pub fn cgl_make_current(_ctx: CGLContextObj) {}

/// Create a GL_TEXTURE_RECTANGLE RGBA8 texture and upload `rgba` (width*height*4 bytes). CGL context must be current. Returns 0 on failure.
#[cfg(target_os = "macos")]
pub fn gl_create_texture_rectangle_rgba8(width: usize, height: usize, rgba: &[u8]) -> u32 {
    let expected = width * height * 4;
    if rgba.len() < expected {
        return 0;
    }
    unsafe {
        ffi::syphon_gl_create_texture_rectangle_rgba8(width, height, rgba.as_ptr())
    }
}
#[cfg(not(target_os = "macos"))]
pub fn gl_create_texture_rectangle_rgba8(_width: usize, _height: usize, _rgba: &[u8]) -> u32 {
    0
}

/// Read back a GL_TEXTURE_RECTANGLE texture into `out_rgba` (width*height*4 bytes). CGL context must be current.
#[cfg(target_os = "macos")]
pub fn gl_read_texture_rectangle_rgba8(tex_id: u32, width: usize, height: usize, out_rgba: &mut [u8]) {
    let expected = width * height * 4;
    if out_rgba.len() < expected {
        return;
    }
    unsafe {
        ffi::syphon_gl_read_texture_rectangle_rgba8(tex_id, width, height, out_rgba.as_mut_ptr());
    }
}
#[cfg(not(target_os = "macos"))]
pub fn gl_read_texture_rectangle_rgba8(_tex_id: u32, _width: usize, _height: usize, _out_rgba: &mut [u8]) {
}

/// Delete a GL texture created with `gl_create_texture_rectangle_rgba8` or returned by Syphon.
#[cfg(target_os = "macos")]
pub fn gl_delete_texture(tex_id: u32) {
    if tex_id != 0 {
        unsafe { ffi::syphon_gl_delete_texture(tex_id) };
    }
}
#[cfg(not(target_os = "macos"))]
pub fn gl_delete_texture(_tex_id: u32) {}

/// Server directory: shared singleton listing available Syphon servers.
pub struct ServerDirectory {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
}

/// Result of a server directory query by name/app name. Release when done (implements Drop).
pub struct ServerDirectoryMatch {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
}

/// Builder for server creation options (private, antialias, depth/stencil). Pass to `OpenGLServer::new` or `MetalServer::new`.
pub struct SyphonOptions {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
}

/// Notification name: a new Syphon server is available (for Cocoa notification center).
pub fn notification_name_server_announce() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        let s = unsafe { ffi::syphon_notification_name_server_announce() };
        opt_cstr_to_string(s)
    }
    #[cfg(not(target_os = "macos"))]
    None
}

/// Notification name: an existing server updated its description.
pub fn notification_name_server_update() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        let s = unsafe { ffi::syphon_notification_name_server_update() };
        opt_cstr_to_string(s)
    }
    #[cfg(not(target_os = "macos"))]
    None
}

/// Notification name: a server will no longer be available.
pub fn notification_name_server_retire() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        let s = unsafe { ffi::syphon_notification_name_server_retire() };
        opt_cstr_to_string(s)
    }
    #[cfg(not(target_os = "macos"))]
    None
}

/// A description of a Syphon server (from the directory or from a server's `server_description`).
/// If you retain it for longer than a directory snapshot, use `retain`/`release` or clone the strings.
pub struct ServerDescription {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
    /// If true, we own a retain and must release on drop.
    #[cfg(target_os = "macos")]
    owned: bool,
}

/// OpenGL Syphon server: publishes frames to clients.
pub struct OpenGLServer {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
}

/// OpenGL Syphon client: receives frames from a server.
pub struct OpenGLClient {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
    /// Keeps the callback alive and gives a stable pointer to the C side.
    #[cfg(target_os = "macos")]
    _callback_storage: Option<Box<CallbackHolder>>,
}

/// A single frame image from a client. Release promptly after drawing.
pub struct OpenGLImage {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
}

/// Opaque pointer to MTLDevice (e.g. from `metal::Device`). Use when creating Metal server/client.
pub type MTLDevicePtr = *mut std::ffi::c_void;

/// Opaque pointer to MTLTexture. Use when publishing a frame or after receiving one from the client.
pub type MTLTexturePtr = *mut std::ffi::c_void;

/// Opaque pointer to MTLCommandBuffer. Use when publishing a frame on the Metal server.
pub type MTLCommandBufferPtr = *mut std::ffi::c_void;

/// Metal Syphon server: publishes frames from Metal textures.
pub struct MetalServer {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
}

/// Metal Syphon client: receives frames as MTLTextures.
pub struct MetalClient {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
    #[cfg(target_os = "macos")]
    _callback_storage: Option<Box<CallbackHolder>>,
}

/// A Metal texture from Syphon (server or client). Release when done drawing.
pub struct MetalTexture {
    #[cfg(target_os = "macos")]
    ptr: NonNull<std::ffi::c_void>,
}

#[cfg(target_os = "macos")]
fn opt_cstr_to_string(s: *mut c_char) -> Option<String> {
    if s.is_null() {
        return None;
    }
    let cstr = unsafe { CStr::from_ptr(s) };
    let out = cstr.to_string_lossy().into_owned();
    unsafe { libc::free(s as *mut _) };
    Some(out)
}

#[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
impl ServerDirectory {
    /// Returns the shared server directory, or `None` on failure.
    pub fn shared() -> Option<Self> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_server_directory_shared() };
            NonNull::new(ptr).map(|ptr| Self { ptr })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Number of servers currently in the directory.
    pub fn servers_count(&self) -> usize {
        #[cfg(target_os = "macos")]
        {
            unsafe { ffi::syphon_server_directory_servers_count(self.ptr.as_ptr()) }
        }
        #[cfg(not(target_os = "macos"))]
        0
    }

    /// Server description at index (not retained; valid only while directory is not refreshed).
    pub fn server_at_index(&self, index: usize) -> Option<ServerDescription> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_server_directory_server_at_index(self.ptr.as_ptr(), index) };
            NonNull::new(ptr).map(|ptr| ServerDescription { ptr, owned: false })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// All current server descriptions. Descriptions are not retained; valid only until the next directory update.
    pub fn servers(&self) -> Vec<ServerDescription> {
        let n = self.servers_count();
        (0..n)
            .filter_map(|i| self.server_at_index(i))
            .collect()
    }

    /// Query servers by optional name and/or app name. Returns a match result you must iterate and drop.
    pub fn servers_matching(
        &self,
        name: Option<&str>,
        app_name: Option<&str>,
    ) -> Option<ServerDirectoryMatch> {
        #[cfg(target_os = "macos")]
        {
            let name_ptr = name
                .and_then(|s| std::ffi::CString::new(s).ok())
                .as_ref()
                .map(|c| c.as_ptr())
                .unwrap_or(std::ptr::null());
            let app_ptr = app_name
                .and_then(|s| std::ffi::CString::new(s).ok())
                .as_ref()
                .map(|c| c.as_ptr())
                .unwrap_or(std::ptr::null());
            let ptr = unsafe {
                ffi::syphon_server_directory_servers_matching(self.ptr.as_ptr(), name_ptr, app_ptr)
            };
            NonNull::new(ptr).map(|ptr| ServerDirectoryMatch { ptr })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }
}

#[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
impl ServerDirectoryMatch {
    /// Number of server descriptions in this match.
    pub fn count(&self) -> usize {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_server_directory_match_count(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        0
    }

    /// Server description at index (retained; caller owns).
    pub fn at(&self, index: usize) -> Option<ServerDescription> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_server_directory_match_at_index(self.ptr.as_ptr(), index) };
            NonNull::new(ptr).map(|ptr| ServerDescription { ptr, owned: true })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Iterate over matched server descriptions (each retained).
    pub fn iter(&self) -> impl Iterator<Item = ServerDescription> + '_ {
        (0..self.count()).filter_map(|i| self.at(i))
    }
}

impl Drop for ServerDirectoryMatch {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_server_directory_match_release(self.ptr.as_ptr());
        }
    }
}

#[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
impl SyphonOptions {
    /// Create an empty options builder. Use `set_is_private`, `set_antialias_sample_count`, etc., then pass to server create.
    pub fn new() -> Option<Self> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_options_create() };
            NonNull::new(ptr).map(|ptr| Self { ptr })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Set the "is private" option (server invisible to others; you pass server description manually). OpenGL and Metal.
    pub fn set_is_private(&self, value: bool) {
        #[cfg(target_os = "macos")]
        {
            let k = unsafe { ffi::syphon_server_option_key_is_private() };
            if !k.is_null() {
                unsafe { ffi::syphon_options_set_bool(self.ptr.as_ptr(), k, value) };
                unsafe { libc::free(k as *mut _) };
            }
        }
    }

    /// Set antialias sample count for OpenGL server (when using bind_to_draw_frame). Ignored by Metal.
    pub fn set_antialias_sample_count(&self, count: u32) {
        #[cfg(target_os = "macos")]
        {
            let k = unsafe { ffi::syphon_server_option_key_antialias_sample_count() };
            if !k.is_null() {
                unsafe {
                    ffi::syphon_options_set_unsigned_long(self.ptr.as_ptr(), k, count as u64);
                }
                unsafe { libc::free(k as *mut _) };
            }
        }
    }

    /// Set depth buffer resolution (16, 24, or 32) for OpenGL server. Ignored by Metal.
    pub fn set_depth_buffer_resolution(&self, bits: u32) {
        #[cfg(target_os = "macos")]
        {
            let k = unsafe { ffi::syphon_server_option_key_depth_buffer_resolution() };
            if !k.is_null() {
                unsafe {
                    ffi::syphon_options_set_unsigned_long(self.ptr.as_ptr(), k, bits as u64);
                }
                unsafe { libc::free(k as *mut _) };
            }
        }
    }

    /// Set stencil buffer resolution (1, 4, 8, or 16) for OpenGL server. Ignored by Metal.
    pub fn set_stencil_buffer_resolution(&self, bits: u32) {
        #[cfg(target_os = "macos")]
        {
            let k = unsafe { ffi::syphon_server_option_key_stencil_buffer_resolution() };
            if !k.is_null() {
                unsafe {
                    ffi::syphon_options_set_unsigned_long(self.ptr.as_ptr(), k, bits as u64);
                }
                unsafe { libc::free(k as *mut _) };
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn as_ptr(&self) -> *mut std::ffi::c_void {
        self.ptr.as_ptr()
    }
}

impl Drop for SyphonOptions {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_options_release(self.ptr.as_ptr());
        }
    }
}

impl ServerDescription {
    /// Copy the server UUID (unique id), if present.
    pub fn uuid(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            let s = unsafe { ffi::syphon_server_description_copy_uuid(self.ptr.as_ptr()) };
            opt_cstr_to_string(s)
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Copy the server name (human-readable), if present.
    pub fn name(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            let s = unsafe { ffi::syphon_server_description_copy_name(self.ptr.as_ptr()) };
            opt_cstr_to_string(s)
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Copy the application name hosting the server, if present.
    pub fn app_name(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            let s = unsafe { ffi::syphon_server_description_copy_app_name(self.ptr.as_ptr()) };
            opt_cstr_to_string(s)
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Retain the description so it remains valid after the directory updates. Call `release` or drop a retained clone when done.
    pub fn retain(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_server_description_retain(self.ptr.as_ptr());
        }
    }

    /// Release a description that was retained.
    pub fn release(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_server_description_release(self.ptr.as_ptr());
        }
    }
}

impl Clone for ServerDescription {
    fn clone(&self) -> Self {
        self.retain();
        Self {
            #[cfg(target_os = "macos")]
            ptr: self.ptr,
            #[cfg(target_os = "macos")]
            owned: true,
        }
    }
}

impl Drop for ServerDescription {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        if self.owned {
            unsafe { ffi::syphon_server_description_release(self.ptr.as_ptr()) };
        }
    }
}

#[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
impl OpenGLServer {
    /// Create a new OpenGL server. `name` can be None (empty). `options` can be None or a `SyphonOptions` (e.g. private server, antialias, depth/stencil).
    /// Returns None if creation failed.
    pub fn new(
        name: Option<&str>,
        context: CGLContextObj,
        options: Option<&SyphonOptions>,
    ) -> Option<Self> {
        #[cfg(target_os = "macos")]
        {
            let name_ptr = name
                .map(|s| std::ffi::CString::new(s).ok())
                .flatten()
                .as_ref()
                .map(|c| c.as_ptr())
                .unwrap_or(std::ptr::null());
            let opts_ptr = options.map(|o| o.as_ptr()).unwrap_or(std::ptr::null_mut());
            let ptr = unsafe { ffi::syphon_opengl_server_create(name_ptr, context, opts_ptr) };
            NonNull::new(ptr).map(|ptr| Self { ptr })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// The CGL context the server uses for drawing.
    pub fn context(&self) -> CGLContextObj {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_opengl_server_context(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        std::ptr::null_mut()
    }

    /// Human-readable server name, if set.
    pub fn name(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            let s = unsafe { ffi::syphon_opengl_server_copy_name(self.ptr.as_ptr()) };
            opt_cstr_to_string(s)
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Set the server's human-readable name.
    pub fn set_name(&self, name: Option<&str>) {
        #[cfg(target_os = "macos")]
        {
            let name_ptr = name
                .map(|s| std::ffi::CString::new(s).ok())
                .flatten()
                .as_ref()
                .map(|c| c.as_ptr())
                .unwrap_or(std::ptr::null());
            unsafe { ffi::syphon_opengl_server_set_name(self.ptr.as_ptr(), name_ptr) };
        }
    }

    /// True if any clients are attached.
    pub fn has_clients(&self) -> bool {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_opengl_server_has_clients(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        false
    }

    /// Server description (retained; caller owns and should release via ServerDescription).
    pub fn server_description(&self) -> Option<ServerDescription> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_opengl_server_server_description(self.ptr.as_ptr()) };
            NonNull::new(ptr).map(|ptr| ServerDescription { ptr, owned: true })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Publish a frame from a texture. Region (x,y,w,h) and texture size (tex_w, tex_h), flipped.
    pub fn publish_frame(
        &self,
        tex_id: u32,
        target: u32,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        tex_w: f64,
        tex_h: f64,
        flipped: bool,
    ) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_opengl_server_publish_frame(
                self.ptr.as_ptr(),
                tex_id,
                target,
                x,
                y,
                w,
                h,
                tex_w,
                tex_h,
                flipped,
            );
        }
    }

    /// Bind the server's FBO to draw a frame of the given size. Pair with `unbind_and_publish`.
    pub fn bind_to_draw_frame(&self, w: f64, h: f64) -> bool {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_opengl_server_bind_to_draw_frame(self.ptr.as_ptr(), w, h) }
        #[cfg(not(target_os = "macos"))]
        false
    }

    /// Unbind and publish the just-drawn frame.
    pub fn unbind_and_publish(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_opengl_server_unbind_and_publish(self.ptr.as_ptr());
        }
    }

    /// Returns the current output frame as an OpenGL image (e.g. for loopback). Caller must release the returned image.
    pub fn new_frame_image(&self) -> Option<OpenGLImage> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_opengl_server_new_frame_image(self.ptr.as_ptr()) };
            NonNull::new(ptr).map(|ptr| OpenGLImage { ptr })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Stop the server.
    pub fn stop(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_opengl_server_stop(self.ptr.as_ptr());
        }
    }
}

impl Drop for OpenGLServer {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        {
            self.stop();
            unsafe {
                ffi::syphon_opengl_server_release(self.ptr.as_ptr());
            }
        }
    }
}

/// Callback for new frames: invoked when a new frame is available (may be on another thread).
pub type NewFrameCallback = Box<dyn Fn() + Send>;

/// Holds the closure so we can pass a single pointer to C and invoke it from the callback.
#[cfg(target_os = "macos")]
struct CallbackHolder(Box<dyn Fn() + Send>);

#[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
impl OpenGLClient {
    /// Create a client for the given server description and context. `callback` can be None (no handler).
    /// The callback may be invoked on a different thread. When provided, it is kept for the client's lifetime.
    pub fn new(
        description: &ServerDescription,
        context: CGLContextObj,
        _options: Option<&std::collections::HashMap<String, String>>,
        callback: Option<NewFrameCallback>,
    ) -> Option<Self> {
        #[cfg(target_os = "macos")]
        {
            unsafe extern "C" fn raw_callback(userdata: *mut std::ffi::c_void) {
                if userdata.is_null() {
                    return;
                }
                let h = &*(userdata as *const CallbackHolder);
                (h.0)();
            }
            let callback_storage: Option<Box<CallbackHolder>> =
                callback.map(|c| Box::new(CallbackHolder(c)));
            let userdata = callback_storage
                .as_ref()
                .map(|b| (&**b) as *const CallbackHolder as *mut std::ffi::c_void)
                .unwrap_or(std::ptr::null_mut());
            type Cb = unsafe extern "C" fn(*mut std::ffi::c_void);
            let cb_opt: Option<Cb> = callback_storage.as_ref().map(|_| raw_callback as Cb);
            let ptr = unsafe {
                ffi::syphon_opengl_client_create(
                    description.ptr.as_ptr(),
                    context,
                    std::ptr::null_mut(),
                    cb_opt,
                    userdata,
                )
            };
            NonNull::new(ptr).map(|ptr| Self {
                ptr,
                _callback_storage: callback_storage,
            })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// The CGL context associated with the client.
    pub fn context(&self) -> CGLContextObj {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_opengl_client_context(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        std::ptr::null_mut()
    }

    /// Server description for the server this client is attached to (retained; caller owns).
    pub fn server_description(&self) -> Option<ServerDescription> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_opengl_client_server_description(self.ptr.as_ptr()) };
            NonNull::new(ptr).map(|ptr| ServerDescription { ptr, owned: true })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    pub fn is_valid(&self) -> bool {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_opengl_client_is_valid(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        false
    }

    pub fn has_new_frame(&self) -> bool {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_opengl_client_has_new_frame(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        false
    }

    /// Get the current frame image. Caller must drop the image when done drawing.
    pub fn new_frame_image(&self) -> Option<OpenGLImage> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_opengl_client_new_frame_image(self.ptr.as_ptr()) };
            NonNull::new(ptr).map(|ptr| OpenGLImage { ptr })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    pub fn stop(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_opengl_client_stop(self.ptr.as_ptr());
        }
    }
}

impl Drop for OpenGLClient {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        {
            self.stop();
            unsafe {
                ffi::syphon_opengl_client_release(self.ptr.as_ptr());
            }
        }
    }
}

impl OpenGLImage {
    pub fn texture_name(&self) -> u32 {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_opengl_image_texture_name(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        0
    }

    pub fn texture_size(&self) -> (f64, f64) {
        #[cfg(target_os = "macos")]
        {
            let mut w = 0.0;
            let mut h = 0.0;
            unsafe {
                ffi::syphon_opengl_image_texture_size(self.ptr.as_ptr(), &mut w, &mut h);
            }
            (w, h)
        }
        #[cfg(not(target_os = "macos"))]
        (0.0, 0.0)
    }
}

impl Drop for OpenGLImage {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_opengl_image_release(self.ptr.as_ptr());
        }
    }
}

// ---------------------------------------------------------------------------
// Metal server
// ---------------------------------------------------------------------------

#[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
impl MetalServer {
    /// Create a Metal server. `name` can be None. `options` can be None or a `SyphonOptions` (e.g. private server).
    /// `device` must be a valid MTLDevice pointer (e.g. from the `metal` crate).
    pub fn new(
        name: Option<&str>,
        device: MTLDevicePtr,
        options: Option<&SyphonOptions>,
    ) -> Option<Self> {
        #[cfg(target_os = "macos")]
        {
            if device.is_null() {
                return None;
            }
            let name_ptr = name
                .map(|s| std::ffi::CString::new(s).ok())
                .flatten()
                .as_ref()
                .map(|c| c.as_ptr())
                .unwrap_or(std::ptr::null());
            let opts_ptr = options.map(|o| o.as_ptr()).unwrap_or(std::ptr::null_mut());
            let ptr =
                unsafe { ffi::syphon_metal_server_create(name_ptr, device as *mut _, opts_ptr) };
            NonNull::new(ptr).map(|ptr| Self { ptr })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// The MTLDevice the server uses.
    pub fn device(&self) -> MTLDevicePtr {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_metal_server_device(self.ptr.as_ptr()) as MTLDevicePtr }
        #[cfg(not(target_os = "macos"))]
        std::ptr::null_mut()
    }

    /// Human-readable server name, if set.
    pub fn name(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            let s = unsafe { ffi::syphon_metal_server_copy_name(self.ptr.as_ptr()) };
            opt_cstr_to_string(s)
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Set the server's human-readable name.
    pub fn set_name(&self, name: Option<&str>) {
        #[cfg(target_os = "macos")]
        {
            let name_ptr = name
                .map(|s| std::ffi::CString::new(s).ok())
                .flatten()
                .as_ref()
                .map(|c| c.as_ptr())
                .unwrap_or(std::ptr::null());
            unsafe { ffi::syphon_metal_server_set_name(self.ptr.as_ptr(), name_ptr) };
        }
    }

    pub fn has_clients(&self) -> bool {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_metal_server_has_clients(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        false
    }

    /// Server description (retained; caller owns).
    pub fn server_description(&self) -> Option<ServerDescription> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_metal_server_server_description(self.ptr.as_ptr()) };
            NonNull::new(ptr).map(|ptr| ServerDescription { ptr, owned: true })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Publish a frame from a Metal texture. Region (x, y, w, h). You must commit `command_buffer`.
    pub fn publish_frame(
        &self,
        texture: MTLTexturePtr,
        command_buffer: MTLCommandBufferPtr,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        flipped: bool,
    ) {
        #[cfg(target_os = "macos")]
        if !texture.is_null() && !command_buffer.is_null() {
            unsafe {
                ffi::syphon_metal_server_publish_frame(
                    self.ptr.as_ptr(),
                    texture as *mut _,
                    command_buffer as *mut _,
                    x,
                    y,
                    w,
                    h,
                    flipped,
                );
            }
        }
    }

    /// Current frame as MTLTexture (caller must release via MetalTexture or syphon_metal_texture_release).
    pub fn new_frame_image(&self) -> Option<MetalTexture> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_metal_server_new_frame_image(self.ptr.as_ptr()) };
            NonNull::new(ptr).map(|ptr| MetalTexture { ptr })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    pub fn stop(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_metal_server_stop(self.ptr.as_ptr());
        }
    }
}

impl Drop for MetalServer {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        {
            self.stop();
            unsafe {
                ffi::syphon_metal_server_release(self.ptr.as_ptr());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Metal client
// ---------------------------------------------------------------------------

#[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
impl MetalClient {
    /// Create a Metal client. `device` must be a valid MTLDevice pointer. `callback` can be None.
    pub fn new(
        description: &ServerDescription,
        device: MTLDevicePtr,
        _options: Option<&std::collections::HashMap<String, String>>,
        callback: Option<NewFrameCallback>,
    ) -> Option<Self> {
        #[cfg(target_os = "macos")]
        {
            if device.is_null() {
                return None;
            }
            unsafe extern "C" fn raw_callback(userdata: *mut std::ffi::c_void) {
                if userdata.is_null() {
                    return;
                }
                let h = &*(userdata as *const CallbackHolder);
                (h.0)();
            }
            let callback_storage: Option<Box<CallbackHolder>> =
                callback.map(|c| Box::new(CallbackHolder(c)));
            let userdata = callback_storage
                .as_ref()
                .map(|b| (&**b) as *const CallbackHolder as *mut std::ffi::c_void)
                .unwrap_or(std::ptr::null_mut());
            type Cb = unsafe extern "C" fn(*mut std::ffi::c_void);
            let cb_opt: Option<Cb> = callback_storage.as_ref().map(|_| raw_callback as Cb);
            let ptr = unsafe {
                ffi::syphon_metal_client_create(
                    description.ptr.as_ptr(),
                    device as *mut _,
                    std::ptr::null_mut(),
                    cb_opt,
                    userdata,
                )
            };
            NonNull::new(ptr).map(|ptr| Self {
                ptr,
                _callback_storage: callback_storage,
            })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Server description for the server this client is attached to (retained; caller owns).
    pub fn server_description(&self) -> Option<ServerDescription> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_metal_client_server_description(self.ptr.as_ptr()) };
            NonNull::new(ptr).map(|ptr| ServerDescription { ptr, owned: true })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    pub fn is_valid(&self) -> bool {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_metal_client_is_valid(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        false
    }

    pub fn has_new_frame(&self) -> bool {
        #[cfg(target_os = "macos")]
        unsafe { ffi::syphon_metal_client_has_new_frame(self.ptr.as_ptr()) }
        #[cfg(not(target_os = "macos"))]
        false
    }

    /// Get the current frame as MTLTexture. Caller must drop the returned value when done.
    pub fn new_frame_image(&self) -> Option<MetalTexture> {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ffi::syphon_metal_client_new_frame_image(self.ptr.as_ptr()) };
            NonNull::new(ptr).map(|ptr| MetalTexture { ptr })
        }
        #[cfg(not(target_os = "macos"))]
        None
    }

    pub fn stop(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_metal_client_stop(self.ptr.as_ptr());
        }
    }
}

impl Drop for MetalClient {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        {
            self.stop();
            unsafe {
                ffi::syphon_metal_client_release(self.ptr.as_ptr());
            }
        }
    }
}

impl MetalTexture {
    /// Raw MTLTexture pointer for use with the `metal` crate or other Metal code.
    pub fn as_ptr(&self) -> MTLTexturePtr {
        #[cfg(target_os = "macos")]
        {
            self.ptr.as_ptr() as MTLTexturePtr
        }
        #[cfg(not(target_os = "macos"))]
        std::ptr::null_mut()
    }
}

impl Drop for MetalTexture {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            ffi::syphon_metal_texture_release(self.ptr.as_ptr());
        }
    }
}

// ---------------------------------------------------------------------------
// Spout (Windows)
// ---------------------------------------------------------------------------

/// Spout instance for sending or receiving frames (Windows only).
/// One handle can be used as sender or receiver (not both concurrently).
#[cfg(target_os = "windows")]
pub struct Spout {
    handle: NonNull<std::ffi::c_void>,
}

#[cfg(target_os = "windows")]
impl Spout {
    /// Create a new Spout instance. Returns `None` if Spout is unavailable.
    pub fn new() -> Option<Self> {
        let handle = unsafe { spout_ffi::spout_create() };
        NonNull::new(handle).map(|handle| Self { handle })
    }

    /// ---- Sender ----
    /// Set the sender name (publisher name).
    pub fn sender_set_name(&self, name: Option<&str>) {
        let name_ptr = name
            .and_then(|s| std::ffi::CString::new(s).ok())
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe { spout_ffi::spout_sender_set_name(self.handle.as_ptr(), name_ptr) };
    }

    /// Set sender DXGI format (e.g. value from DXGI_FORMAT).
    pub fn sender_set_format(&self, dxgi_format: u32) {
        unsafe { spout_ffi::spout_sender_set_format(self.handle.as_ptr(), dxgi_format) };
    }

    /// Send an OpenGL texture. Returns true on success.
    pub fn sender_send_texture(
        &self,
        tex_id: u32,
        target: u32,
        width: u32,
        height: u32,
        invert: bool,
    ) -> bool {
        unsafe {
            spout_ffi::spout_sender_send_texture(
                self.handle.as_ptr(),
                tex_id,
                target,
                width,
                height,
                invert,
            )
        }
    }

    /// Send from a bound FBO. The FBO must be currently bound. Use 0 for default framebuffer with width/height.
    pub fn sender_send_fbo(&self, fbo_id: u32, width: u32, height: u32, invert: bool) -> bool {
        unsafe {
            spout_ffi::spout_sender_send_fbo(
                self.handle.as_ptr(),
                fbo_id,
                width,
                height,
                invert,
            )
        }
    }

    /// Send image pixels (e.g. GL_RGBA). Buffer size must be at least width*height*bytes_per_pixel for the format.
    pub fn sender_send_image(
        &self,
        pixels: &[u8],
        width: u32,
        height: u32,
        gl_format: u32,
        invert: bool,
    ) -> bool {
        unsafe {
            spout_ffi::spout_sender_send_image(
                self.handle.as_ptr(),
                pixels.as_ptr(),
                width,
                height,
                gl_format,
                invert,
            )
        }
    }

    /// Release the sender and free resources.
    pub fn sender_release(&self) {
        unsafe { spout_ffi::spout_sender_release(self.handle.as_ptr()) };
    }

    /// True if the sender is initialized.
    pub fn sender_is_initialized(&self) -> bool {
        unsafe { spout_ffi::spout_sender_is_initialized(self.handle.as_ptr()) }
    }

    pub fn sender_width(&self) -> u32 {
        unsafe { spout_ffi::spout_sender_get_width(self.handle.as_ptr()) }
    }

    pub fn sender_height(&self) -> u32 {
        unsafe { spout_ffi::spout_sender_get_height(self.handle.as_ptr()) }
    }

    /// Current sender name (None if not set or error). Caller does not free.
    pub fn sender_name(&self) -> Option<String> {
        let s = unsafe { spout_ffi::spout_sender_get_name(self.handle.as_ptr()) };
        if s.is_null() {
            return None;
        }
        let cstr = unsafe { std::ffi::CStr::from_ptr(s) };
        let out = cstr.to_string_lossy().into_owned();
        unsafe { spout_ffi::spout_string_free(s) };
        Some(out)
    }

    /// Current sender DXGI format.
    pub fn sender_format(&self) -> u32 {
        unsafe { spout_ffi::spout_sender_get_format(self.handle.as_ptr()) }
    }

    /// Sender frame rate.
    pub fn sender_fps(&self) -> f64 {
        unsafe { spout_ffi::spout_sender_get_fps(self.handle.as_ptr()) }
    }

    /// Sender frame number.
    pub fn sender_frame(&self) -> i64 {
        unsafe { spout_ffi::spout_sender_get_frame(self.handle.as_ptr()) as i64 }
    }

    /// ---- Receiver ----
    /// Set the sender name to receive from (None = active sender).
    pub fn receiver_set_name(&self, sender_name: Option<&str>) {
        let ptr = sender_name
            .and_then(|s| std::ffi::CString::new(s).ok())
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe { spout_ffi::spout_receiver_set_name(self.handle.as_ptr(), ptr) };
    }

    /// Receive into an OpenGL texture. Returns true on success.
    pub fn receiver_receive_texture(&self, tex_id: u32, target: u32, invert: bool) -> bool {
        unsafe { spout_ffi::spout_receiver_receive_texture(self.handle.as_ptr(), tex_id, target, invert) }
    }

    /// Receive into a pixel buffer. Buffer size must be at least sender_width*sender_height*bytes_per_pixel for the format.
    pub fn receiver_receive_image(&self, pixels: &mut [u8], gl_format: u32, invert: bool) -> bool {
        unsafe {
            spout_ffi::spout_receiver_receive_image(
                self.handle.as_ptr(),
                pixels.as_mut_ptr(),
                gl_format,
                invert,
            )
        }
    }

    /// Release the receiver.
    pub fn receiver_release(&self) {
        unsafe { spout_ffi::spout_receiver_release(self.handle.as_ptr()) };
    }

    /// Get the name of the sender we're receiving from.
    pub fn receiver_sender_name(&self) -> Option<String> {
        let mut buf = vec![0u8; 256];
        let ok = unsafe {
            spout_ffi::spout_receiver_get_sender_name(
                self.handle.as_ptr(),
                buf.as_mut_ptr() as *mut i8,
                256,
            )
        };
        if !ok {
            return None;
        }
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        std::str::from_utf8(&buf[..end]).ok().map(|s| s.to_string())
    }

    pub fn receiver_is_frame_new(&self) -> bool {
        unsafe { spout_ffi::spout_receiver_is_frame_new(self.handle.as_ptr()) }
    }

    /// True if the sender has updated (dimensions or connection). Check before receiving.
    pub fn receiver_is_updated(&self) -> bool {
        unsafe { spout_ffi::spout_receiver_is_updated(self.handle.as_ptr()) }
    }

    pub fn receiver_is_connected(&self) -> bool {
        unsafe { spout_ffi::spout_receiver_is_connected(self.handle.as_ptr()) }
    }

    pub fn receiver_sender_width(&self) -> u32 {
        unsafe { spout_ffi::spout_receiver_get_sender_width(self.handle.as_ptr()) }
    }

    pub fn receiver_sender_height(&self) -> u32 {
        unsafe { spout_ffi::spout_receiver_get_sender_height(self.handle.as_ptr()) }
    }

    /// Sender DXGI format (when receiving).
    pub fn receiver_sender_format(&self) -> u32 {
        unsafe { spout_ffi::spout_receiver_get_sender_format(self.handle.as_ptr()) }
    }

    /// Sender frame rate (when receiving).
    pub fn receiver_sender_fps(&self) -> f64 {
        unsafe { spout_ffi::spout_receiver_get_sender_fps(self.handle.as_ptr()) }
    }

    /// Sender frame number (when receiving).
    pub fn receiver_sender_frame(&self) -> i64 {
        unsafe { spout_ffi::spout_receiver_get_sender_frame(self.handle.as_ptr()) as i64 }
    }

    /// ---- Bind shared texture (receiver) ----
    /// Bind the shared texture for reading. Use `shared_texture_id()` after bind to get the GL texture ID.
    pub fn bind_shared_texture(&self) -> bool {
        unsafe { spout_ffi::spout_bind_shared_texture(self.handle.as_ptr()) }
    }

    /// Unbind the shared texture.
    pub fn unbind_shared_texture(&self) -> bool {
        unsafe { spout_ffi::spout_unbind_shared_texture(self.handle.as_ptr()) }
    }

    /// OpenGL shared texture ID (valid after bind_shared_texture).
    pub fn shared_texture_id(&self) -> u32 {
        unsafe { spout_ffi::spout_get_shared_texture_id(self.handle.as_ptr()) }
    }

    /// ---- Sender list (discovery) ----
    pub fn sender_count(&self) -> i32 {
        unsafe { spout_ffi::spout_get_sender_count(self.handle.as_ptr()) }
    }

    /// Get sender name at index. Returns None if index out of range.
    pub fn sender_name_at(&self, index: i32) -> Option<String> {
        let mut buf = vec![0u8; 256];
        let ok = unsafe {
            spout_ffi::spout_get_sender_name(
                self.handle.as_ptr(),
                index,
                buf.as_mut_ptr() as *mut i8,
                256,
            )
        };
        if !ok {
            return None;
        }
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        std::str::from_utf8(&buf[..end]).ok().map(|s| s.to_string())
    }

    /// True if the given sender name exists in the list.
    pub fn find_sender_name(&self, sendername: &str) -> bool {
        let c = std::ffi::CString::new(sendername).ok();
        let ptr = c.as_ref().map(|c| c.as_ptr()).unwrap_or(std::ptr::null());
        unsafe { spout_ffi::spout_find_sender_name(self.handle.as_ptr(), ptr) }
    }

    /// Get the current active sender name.
    pub fn active_sender(&self) -> Option<String> {
        let mut buf = vec![0u8; 256];
        let ok = unsafe {
            spout_ffi::spout_get_active_sender(
                self.handle.as_ptr(),
                buf.as_mut_ptr() as *mut i8,
                256,
            )
        };
        if !ok {
            return None;
        }
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        if end == 0 {
            return None;
        }
        std::str::from_utf8(&buf[..end]).ok().map(|s| s.to_string())
    }

    /// Set the active sender by name.
    pub fn set_active_sender(&self, sendername: Option<&str>) -> bool {
        let ptr = sendername
            .and_then(|s| std::ffi::CString::new(s).ok())
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe { spout_ffi::spout_set_active_sender(self.handle.as_ptr(), ptr) }
    }

    /// Sender info (width, height, share handle, DX format). Returns None if not found.
    pub fn sender_info(&self, sendername: &str) -> Option<SpoutSenderInfo> {
        let name = std::ffi::CString::new(sendername).ok()?;
        let mut width = 0u32;
        let mut height = 0u32;
        let mut handle = std::ptr::null_mut();
        let mut format = 0u32;
        let ok = unsafe {
            spout_ffi::spout_get_sender_info(
                self.handle.as_ptr(),
                name.as_ptr(),
                &mut width,
                &mut height,
                &mut handle,
                &mut format,
            )
        };
        if !ok {
            return None;
        }
        Some(SpoutSenderInfo {
            width,
            height,
            share_handle: handle,
            format,
        })
    }

    /// Signal a frame-sync event for a sender.
    pub fn set_frame_sync(&self, sendername: Option<&str>) {
        let ptr = sendername
            .and_then(|s| std::ffi::CString::new(s).ok())
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe { spout_ffi::spout_set_frame_sync(self.handle.as_ptr(), ptr) };
    }

    /// Wait (or poll with timeout 0) for a frame-sync event.
    pub fn wait_frame_sync(&self, sendername: Option<&str>, timeout_ms: u32) -> bool {
        let ptr = sendername
            .and_then(|s| std::ffi::CString::new(s).ok())
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe { spout_ffi::spout_wait_frame_sync(self.handle.as_ptr(), ptr, timeout_ms) }
    }

    /// Enable or disable frame sync.
    pub fn enable_frame_sync(&self, enabled: bool) {
        unsafe { spout_ffi::spout_enable_frame_sync(self.handle.as_ptr(), enabled) };
    }

    /// Close frame sync resources.
    pub fn close_frame_sync(&self) {
        unsafe { spout_ffi::spout_close_frame_sync(self.handle.as_ptr()) };
    }

    /// True if frame sync is enabled.
    pub fn is_frame_sync_enabled(&self) -> bool {
        unsafe { spout_ffi::spout_is_frame_sync_enabled(self.handle.as_ptr()) }
    }

    /// Write arbitrary bytes to a shared memory buffer for a sender.
    pub fn write_memory_buffer(&self, sendername: &str, data: &[u8]) -> bool {
        let name = match std::ffi::CString::new(sendername) {
            Ok(v) => v,
            Err(_) => return false,
        };
        unsafe {
            spout_ffi::spout_write_memory_buffer(
                self.handle.as_ptr(),
                name.as_ptr(),
                data.as_ptr() as *const i8,
                data.len() as i32,
            )
        }
    }

    /// Read bytes from a sender memory buffer into `out`. Returns number of bytes read.
    pub fn read_memory_buffer(&self, sendername: &str, out: &mut [u8]) -> usize {
        let name = match std::ffi::CString::new(sendername) {
            Ok(v) => v,
            Err(_) => return 0,
        };
        let n = unsafe {
            spout_ffi::spout_read_memory_buffer(
                self.handle.as_ptr(),
                name.as_ptr(),
                out.as_mut_ptr() as *mut i8,
                out.len() as i32,
            )
        };
        if n <= 0 {
            0
        } else {
            n as usize
        }
    }

    /// Maximum number of registered senders allowed by Spout.
    pub fn max_senders(&self) -> i32 {
        unsafe { spout_ffi::spout_get_max_senders(self.handle.as_ptr()) }
    }

    /// True if sender frame buffering is enabled.
    pub fn buffer_mode(&self) -> bool {
        unsafe { spout_ffi::spout_get_buffer_mode(self.handle.as_ptr()) }
    }

    /// Enable or disable sender frame buffering.
    pub fn set_buffer_mode(&self, active: bool) {
        unsafe { spout_ffi::spout_set_buffer_mode(self.handle.as_ptr(), active) };
    }

    /// Number of sender frame buffers.
    pub fn buffers(&self) -> i32 {
        unsafe { spout_ffi::spout_get_buffers(self.handle.as_ptr()) }
    }

    /// Set number of sender frame buffers.
    pub fn set_buffers(&self, buffers: i32) {
        unsafe { spout_ffi::spout_set_buffers(self.handle.as_ptr(), buffers) };
    }

    /// Current CPU sharing mode.
    pub fn cpu_mode(&self) -> bool {
        unsafe { spout_ffi::spout_get_cpu_mode(self.handle.as_ptr()) }
    }

    /// Set CPU sharing mode.
    pub fn set_cpu_mode(&self, cpu_mode: bool) -> bool {
        unsafe { spout_ffi::spout_set_cpu_mode(self.handle.as_ptr(), cpu_mode) }
    }
}

/// Sender info from discovery (width, height, DX share handle, format).
#[cfg(target_os = "windows")]
#[derive(Debug, Clone)]
pub struct SpoutSenderInfo {
    pub width: u32,
    pub height: u32,
    pub share_handle: *mut std::ffi::c_void,
    pub format: u32,
}

#[cfg(target_os = "windows")]
impl Drop for Spout {
    fn drop(&mut self) {
        unsafe {
            spout_ffi::spout_destroy(self.handle.as_ptr());
        }
    }
}
