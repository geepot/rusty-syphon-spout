/*
 * syphon_glue.m - Objective-C implementation of the Syphon C glue API.
 */
#ifdef __APPLE__

#define GL_SILENCE_DEPRECATION 1
/* GL_TEXTURE_RECTANGLE not in macOS gl.h; Syphon uses rectangle textures. */
#ifndef GL_TEXTURE_RECTANGLE
#define GL_TEXTURE_RECTANGLE 0x84F5
#endif

#import <Foundation/Foundation.h>
#import <CoreFoundation/CoreFoundation.h>
#import <OpenGL/OpenGL.h>
#import <OpenGL/gl.h>
#import <Metal/Metal.h>
#import <Syphon/Syphon.h>

/* Support both newer framework (SyphonOpenGLServer etc.) and older (SyphonServer/SyphonClient/SyphonImage). */
#if __has_include(<Syphon/SyphonOpenGLServer.h>)
#import <Syphon/SyphonOpenGLServer.h>
#import <Syphon/SyphonOpenGLClient.h>
#import <Syphon/SyphonOpenGLImage.h>
#define SYPHON_GL_SERVER SyphonOpenGLServer
#define SYPHON_GL_CLIENT SyphonOpenGLClient
#define SYPHON_GL_IMAGE SyphonOpenGLImage
#else
#define SYPHON_GL_SERVER SyphonServer
#define SYPHON_GL_CLIENT SyphonClient
#define SYPHON_GL_IMAGE SyphonImage
#endif

#if __has_include(<Syphon/SyphonMetalServer.h>)
#import <Syphon/SyphonMetalServer.h>
#import <Syphon/SyphonMetalClient.h>
#define SYPHON_HAS_METAL 1
#else
#define SYPHON_HAS_METAL 0
#endif

static NSString *nullable_cstring_to_nsstring(const char *cstr) {
    if (!cstr) return nil;
    return [NSString stringWithUTF8String:cstr];
}

/* Server directory */
void *syphon_server_directory_shared(void) {
    return (__bridge void *)[SyphonServerDirectory sharedDirectory];
}

size_t syphon_server_directory_servers_count(void *dir) {
    SyphonServerDirectory *d = (__bridge SyphonServerDirectory *)dir;
    return (size_t)[d.servers count];
}

void *syphon_server_directory_server_at_index(void *dir, size_t index) {
    SyphonServerDirectory *d = (__bridge SyphonServerDirectory *)dir;
    NSArray *servers = d.servers;
    if (index >= [servers count]) return NULL;
    return (__bridge void *)[servers objectAtIndex:index];
}

void *syphon_server_directory_servers_matching(void *dir, const char *name, const char *app_name) {
    SyphonServerDirectory *d = (__bridge SyphonServerDirectory *)dir;
    NSString *nsName = nullable_cstring_to_nsstring(name);
    NSString *nsApp = nullable_cstring_to_nsstring(app_name);
    NSArray *arr = [d serversMatchingName:nsName appName:nsApp];
    return (__bridge_retained void *)arr;
}

size_t syphon_server_directory_match_count(void *match_result) {
    NSArray *arr = (__bridge NSArray *)match_result;
    return (size_t)[arr count];
}

void *syphon_server_directory_match_at_index(void *match_result, size_t index) {
    NSArray *arr = (__bridge NSArray *)match_result;
    if (index >= [arr count]) return NULL;
    NSDictionary *desc = [arr objectAtIndex:index];
    return (__bridge_retained void *)desc;
}

void syphon_server_directory_match_release(void *match_result) {
    (void)(__bridge_transfer NSArray *)match_result;
}

static char *copy_nsstring_to_cstring(NSString *s) {
    if (!s) return NULL;
    const char *utf8 = [s UTF8String];
    if (!utf8) return NULL;
    return strdup(utf8);
}

char *syphon_server_description_copy_uuid(void *desc) {
    NSDictionary *d = (__bridge NSDictionary *)desc;
    NSString *v = d[SyphonServerDescriptionUUIDKey];
    return copy_nsstring_to_cstring(v);
}

char *syphon_server_description_copy_name(void *desc) {
    NSDictionary *d = (__bridge NSDictionary *)desc;
    NSString *v = d[SyphonServerDescriptionNameKey];
    return copy_nsstring_to_cstring(v);
}

char *syphon_server_description_copy_app_name(void *desc) {
    NSDictionary *d = (__bridge NSDictionary *)desc;
    NSString *v = d[SyphonServerDescriptionAppNameKey];
    return copy_nsstring_to_cstring(v);
}

char *syphon_notification_name_server_announce(void) {
    return copy_nsstring_to_cstring(SyphonServerAnnounceNotification);
}

char *syphon_notification_name_server_update(void) {
    return copy_nsstring_to_cstring(SyphonServerUpdateNotification);
}

char *syphon_notification_name_server_retire(void) {
    return copy_nsstring_to_cstring(SyphonServerRetireNotification);
}

void syphon_server_description_retain(void *desc) {
    (void)CFBridgingRetain((__bridge id)desc);
}

void syphon_server_description_release(void *desc) {
    if (desc) CFRelease((CFTypeRef)desc);
}

/* Server options */
void *syphon_options_create(void) {
    return (__bridge_retained void *)[NSMutableDictionary dictionary];
}

void syphon_options_set_bool(void *opts, const char *key, bool value) {
    NSMutableDictionary *d = (__bridge NSMutableDictionary *)opts;
    NSString *k = nullable_cstring_to_nsstring(key);
    if (k) d[k] = @(value ? YES : NO);
}

void syphon_options_set_unsigned_long(void *opts, const char *key, unsigned long value) {
    NSMutableDictionary *d = (__bridge NSMutableDictionary *)opts;
    NSString *k = nullable_cstring_to_nsstring(key);
    if (k) d[k] = @(value);
}

void syphon_options_release(void *opts) {
    (void)(__bridge_transfer NSMutableDictionary *)opts;
}

static char *copy_key(NSString *s) {
    if (!s) return NULL;
    const char *utf8 = [s UTF8String];
    return utf8 ? strdup(utf8) : NULL;
}

char *syphon_server_option_key_is_private(void) {
    return copy_key(SyphonServerOptionIsPrivate);
}

char *syphon_server_option_key_antialias_sample_count(void) {
    return copy_key(SyphonServerOptionAntialiasSampleCount);
}

char *syphon_server_option_key_depth_buffer_resolution(void) {
    return copy_key(SyphonServerOptionDepthBufferResolution);
}

char *syphon_server_option_key_stencil_buffer_resolution(void) {
    return copy_key(SyphonServerOptionStencilBufferResolution);
}

/* OpenGL server */
void *syphon_opengl_server_create(const char *name, CGLContextObj context, void *options) {
    NSString *nsName = nullable_cstring_to_nsstring(name);
    SYPHON_GL_SERVER *server = [[SYPHON_GL_SERVER alloc] initWithName:nsName
                                                                  context:context
                                                                  options:(__bridge NSDictionary *)options];
    return (__bridge_retained void *)server;
}

void syphon_opengl_server_release(void *server) {
    (void)(__bridge_transfer SYPHON_GL_SERVER *)server;
}

bool syphon_opengl_server_has_clients(void *server) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    return s.hasClients ? true : false;
}

void *syphon_opengl_server_server_description(void *server) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    NSDictionary *desc = s.serverDescription;
    return (__bridge_retained void *)desc;
}

void syphon_opengl_server_publish_frame(void *server, GLuint tex_id, GLenum target,
    double x, double y, double w, double h, double tex_w, double tex_h, bool flipped) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    NSRect region = NSMakeRect(x, y, w, h);
    NSSize size = NSMakeSize(tex_w, tex_h);
    [s publishFrameTexture:tex_id textureTarget:target imageRegion:region
        textureDimensions:size flipped:flipped ? YES : NO];
}

bool syphon_opengl_server_bind_to_draw_frame(void *server, double w, double h) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    NSSize size = NSMakeSize(w, h);
    return [s bindToDrawFrameOfSize:size] ? true : false;
}

void syphon_opengl_server_unbind_and_publish(void *server) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    [s unbindAndPublish];
}

void syphon_opengl_server_stop(void *server) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    [s stop];
}

CGLContextObj syphon_opengl_server_context(void *server) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    return s.context;
}

char *syphon_opengl_server_copy_name(void *server) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    return copy_nsstring_to_cstring(s.name);
}

void syphon_opengl_server_set_name(void *server, const char *name) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    s.name = nullable_cstring_to_nsstring(name);
}

void *syphon_opengl_server_new_frame_image(void *server) {
    SYPHON_GL_SERVER *s = (__bridge SYPHON_GL_SERVER *)server;
    SYPHON_GL_IMAGE *img = [s newFrameImage];
    return (__bridge_retained void *)img;
}

/* OpenGL client */
typedef void (*new_frame_callback_t)(void *userdata);

void *syphon_opengl_client_create(void *server_description, CGLContextObj context,
    void *options, new_frame_callback_t new_frame_callback, void *userdata) {
    NSDictionary *desc = (__bridge NSDictionary *)server_description;
    void (^handler)(SYPHON_GL_CLIENT *);
    if (new_frame_callback) {
        new_frame_callback_t cb = new_frame_callback;
        void *ud = userdata;
        handler = ^(SYPHON_GL_CLIENT *client) {
            (void)client;
            cb(ud);
        };
    } else {
        handler = nil;
    }
    SYPHON_GL_CLIENT *client = [[SYPHON_GL_CLIENT alloc] initWithServerDescription:desc
                                                                             context:context
                                                                             options:(__bridge NSDictionary *)options
                                                                       newFrameHandler:handler];
    return (__bridge_retained void *)client;
}

void syphon_opengl_client_release(void *client) {
    (void)(__bridge_transfer SYPHON_GL_CLIENT *)client;
}

bool syphon_opengl_client_is_valid(void *client) {
    SYPHON_GL_CLIENT *c = (__bridge SYPHON_GL_CLIENT *)client;
    return c.isValid ? true : false;
}

bool syphon_opengl_client_has_new_frame(void *client) {
    SYPHON_GL_CLIENT *c = (__bridge SYPHON_GL_CLIENT *)client;
    return c.hasNewFrame ? true : false;
}

void *syphon_opengl_client_new_frame_image(void *client) {
    SYPHON_GL_CLIENT *c = (__bridge SYPHON_GL_CLIENT *)client;
    SYPHON_GL_IMAGE *img = [c newFrameImage];
    return (__bridge_retained void *)img;
}

void syphon_opengl_client_stop(void *client) {
    SYPHON_GL_CLIENT *c = (__bridge SYPHON_GL_CLIENT *)client;
    [c stop];
}

CGLContextObj syphon_opengl_client_context(void *client) {
    SYPHON_GL_CLIENT *c = (__bridge SYPHON_GL_CLIENT *)client;
    return c.context;
}

void *syphon_opengl_client_server_description(void *client) {
    SYPHON_GL_CLIENT *c = (__bridge SYPHON_GL_CLIENT *)client;
    NSDictionary *desc = c.serverDescription;
    return (__bridge_retained void *)desc;
}

/* OpenGL image */
void syphon_opengl_image_release(void *image) {
    (void)(__bridge_transfer SYPHON_GL_IMAGE *)image;
}

GLuint syphon_opengl_image_texture_name(void *image) {
    SYPHON_GL_IMAGE *img = (__bridge SYPHON_GL_IMAGE *)image;
    return img.textureName;
}

void syphon_opengl_image_texture_size(void *image, double *out_w, double *out_h) {
    SYPHON_GL_IMAGE *img = (__bridge SYPHON_GL_IMAGE *)image;
    NSSize size = img.textureSize;
    if (out_w) *out_w = size.width;
    if (out_h) *out_h = size.height;
}

#if SYPHON_HAS_METAL
/* Metal server */
void *syphon_metal_server_create(const char *name, void *device, void *options) {
    NSString *nsName = nullable_cstring_to_nsstring(name);
    id<MTLDevice> mtlDevice = (__bridge id<MTLDevice>)device;
    SyphonMetalServer *server = [[SyphonMetalServer alloc] initWithName:nsName
                                                                  device:mtlDevice
                                                                 options:(__bridge NSDictionary *)options];
    return (__bridge_retained void *)server;
}

void syphon_metal_server_release(void *server) {
    (void)(__bridge_transfer SyphonMetalServer *)server;
}

bool syphon_metal_server_has_clients(void *server) {
    SyphonMetalServer *s = (__bridge SyphonMetalServer *)server;
    return s.hasClients ? true : false;
}

void *syphon_metal_server_server_description(void *server) {
    SyphonMetalServer *s = (__bridge SyphonMetalServer *)server;
    NSDictionary *desc = s.serverDescription;
    return (__bridge_retained void *)desc;
}

void syphon_metal_server_publish_frame(void *server, void *texture, void *command_buffer,
    double x, double y, double w, double h, bool flipped) {
    SyphonMetalServer *s = (__bridge SyphonMetalServer *)server;
    id<MTLTexture> mtlTexture = (__bridge id<MTLTexture>)texture;
    id<MTLCommandBuffer> mtlCmdBuf = (__bridge id<MTLCommandBuffer>)command_buffer;
    NSRect region = NSMakeRect(x, y, w, h);
    [s publishFrameTexture:mtlTexture onCommandBuffer:mtlCmdBuf imageRegion:region flipped:flipped ? YES : NO];
}

void *syphon_metal_server_new_frame_image(void *server) {
    SyphonMetalServer *s = (__bridge SyphonMetalServer *)server;
    id<MTLTexture> tex = [s newFrameImage];
    return (__bridge_retained void *)tex;
}

void syphon_metal_server_stop(void *server) {
    SyphonMetalServer *s = (__bridge SyphonMetalServer *)server;
    [s stop];
}

void *syphon_metal_server_device(void *server) {
    SyphonMetalServer *s = (__bridge SyphonMetalServer *)server;
    return (__bridge void *)s.device;
}

char *syphon_metal_server_copy_name(void *server) {
    SyphonMetalServer *s = (__bridge SyphonMetalServer *)server;
    return copy_nsstring_to_cstring(s.name);
}

void syphon_metal_server_set_name(void *server, const char *name) {
    SyphonMetalServer *s = (__bridge SyphonMetalServer *)server;
    s.name = nullable_cstring_to_nsstring(name);
}

/* Metal client */
void *syphon_metal_client_create(void *server_description, void *device,
    void *options, new_frame_callback_t new_frame_callback, void *userdata) {
    NSDictionary *desc = (__bridge NSDictionary *)server_description;
    id<MTLDevice> mtlDevice = (__bridge id<MTLDevice>)device;
    void (^handler)(SyphonMetalClient *);
    if (new_frame_callback) {
        new_frame_callback_t cb = new_frame_callback;
        void *ud = userdata;
        handler = ^(SyphonMetalClient *client) {
            (void)client;
            cb(ud);
        };
    } else {
        handler = nil;
    }
    SyphonMetalClient *client = [[SyphonMetalClient alloc] initWithServerDescription:desc
                                                                             device:mtlDevice
                                                                            options:(__bridge NSDictionary *)options
                                                                    newFrameHandler:handler];
    return (__bridge_retained void *)client;
}

void syphon_metal_client_release(void *client) {
    (void)(__bridge_transfer SyphonMetalClient *)client;
}

bool syphon_metal_client_is_valid(void *client) {
    SyphonMetalClient *c = (__bridge SyphonMetalClient *)client;
    return c.isValid ? true : false;
}

bool syphon_metal_client_has_new_frame(void *client) {
    SyphonMetalClient *c = (__bridge SyphonMetalClient *)client;
    return c.hasNewFrame ? true : false;
}

void *syphon_metal_client_new_frame_image(void *client) {
    SyphonMetalClient *c = (__bridge SyphonMetalClient *)client;
    id<MTLTexture> tex = [c newFrameImage];
    return (__bridge_retained void *)tex;
}

void syphon_metal_client_stop(void *client) {
    SyphonMetalClient *c = (__bridge SyphonMetalClient *)client;
    [c stop];
}

void *syphon_metal_client_server_description(void *client) {
    SyphonMetalClient *c = (__bridge SyphonMetalClient *)client;
    NSDictionary *desc = c.serverDescription;
    return (__bridge_retained void *)desc;
}

void syphon_metal_texture_release(void *texture) {
    (void)(__bridge_transfer id)texture;
}
#else
/* Stubs when framework has no Metal support (older Syphon) */
void *syphon_metal_server_create(const char *name, void *device, void *options) { (void)name;(void)device;(void)options; return NULL; }
void syphon_metal_server_release(void *server) { (void)server; }
bool syphon_metal_server_has_clients(void *server) { (void)server; return false; }
void *syphon_metal_server_server_description(void *server) { (void)server; return NULL; }
void syphon_metal_server_publish_frame(void *server, void *texture, void *command_buffer, double x, double y, double w, double h, bool flipped) { (void)server;(void)texture;(void)command_buffer;(void)x;(void)y;(void)w;(void)h;(void)flipped; }
void *syphon_metal_server_new_frame_image(void *server) { (void)server; return NULL; }
void syphon_metal_server_stop(void *server) { (void)server; }
void *syphon_metal_server_device(void *server) { (void)server; return NULL; }
char *syphon_metal_server_copy_name(void *server) { (void)server; return NULL; }
void syphon_metal_server_set_name(void *server, const char *name) { (void)server;(void)name; }
void *syphon_metal_client_create(void *server_description, void *device, void *options, void (*new_frame_callback)(void *), void *userdata) { (void)server_description;(void)device;(void)options;(void)new_frame_callback;(void)userdata; return NULL; }
void syphon_metal_client_release(void *client) { (void)client; }
bool syphon_metal_client_is_valid(void *client) { (void)client; return false; }
bool syphon_metal_client_has_new_frame(void *client) { (void)client; return false; }
void *syphon_metal_client_new_frame_image(void *client) { (void)client; return NULL; }
void syphon_metal_client_stop(void *client) { (void)client; }
void *syphon_metal_client_server_description(void *client) { (void)client; return NULL; }
void syphon_metal_texture_release(void *texture) { (void)texture; }
#endif /* SYPHON_HAS_METAL */

/* CGL headless context (for tests) */
CGLContextObj syphon_cgl_create_headless_context(void) {
    CGLPixelFormatAttribute attrs[] = {
        kCGLPFAOpenGLProfile, (CGLPixelFormatAttribute)kCGLOGLPVersion_3_2_Core,
        kCGLPFAAccelerated,
        (CGLPixelFormatAttribute)0
    };
    CGLPixelFormatObj pix = NULL;
    GLint npix = 0;
    if (CGLChoosePixelFormat(attrs, &pix, &npix) != kCGLNoError || !pix || npix == 0) {
        return NULL;
    }
    CGLContextObj ctx = NULL;
    if (CGLCreateContext(pix, NULL, &ctx) != kCGLNoError) {
        CGLDestroyPixelFormat(pix);
        return NULL;
    }
    CGLDestroyPixelFormat(pix);
    return ctx;
}

void syphon_cgl_destroy_context(CGLContextObj ctx) {
    if (ctx) {
        CGLSetCurrentContext(NULL);
        CGLDestroyContext(ctx);
    }
}

void syphon_cgl_make_current(CGLContextObj ctx) {
    CGLSetCurrentContext(ctx);
}

/* OpenGL texture helpers; CGL context must be current. GL_TEXTURE_RECTANGLE, RGBA8. */
GLuint syphon_gl_create_texture_rectangle_rgba8(size_t width, size_t height, const unsigned char *rgba) {
    GLuint tex = 0;
    glGenTextures(1, &tex);
    if (tex == 0) return 0;
    glBindTexture(GL_TEXTURE_RECTANGLE, tex);
    glTexImage2D(GL_TEXTURE_RECTANGLE, 0, GL_RGBA8, (GLsizei)width, (GLsizei)height, 0,
                 GL_RGBA, GL_UNSIGNED_BYTE, rgba ? rgba : NULL);
    glBindTexture(GL_TEXTURE_RECTANGLE, 0);
    return tex;
}

void syphon_gl_read_texture_rectangle_rgba8(GLuint tex_id, size_t width, size_t height, unsigned char *out_rgba) {
    if (!out_rgba || tex_id == 0) return;
    GLuint fbo = 0;
    glGenFramebuffers(1, &fbo);
    glBindFramebuffer(GL_FRAMEBUFFER, fbo);
    glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_RECTANGLE, tex_id, 0);
    /* Syphon client textures are top-down in GL (top = y=0); read rows in order so out_rgba is top row first. */
    const size_t row_bytes = width * 4;
    for (size_t row = 0; row < height; row++) {
        glReadPixels(0, (GLint)row, (GLsizei)width, 1, GL_RGBA, GL_UNSIGNED_BYTE,
                     out_rgba + row * row_bytes);
    }
    glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_RECTANGLE, 0, 0);
    glBindFramebuffer(GL_FRAMEBUFFER, 0);
    glDeleteFramebuffers(1, &fbo);
}

void syphon_gl_delete_texture(GLuint tex_id) {
    if (tex_id != 0) {
        glDeleteTextures(1, &tex_id);
    }
}

#endif /* __APPLE__ */
