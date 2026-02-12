/*
 * spout_glue.h - C API for Spout (Windows only).
 * Opaque handle; implement in spout_glue.cpp using SpoutLibrary.
 */
#ifndef SPOUT_GLUE_H
#define SPOUT_GLUE_H

#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void* spout_handle;

/* Create a Spout instance (GetSpout()). Caller must call spout_destroy when done. */
spout_handle spout_create(void);
void spout_destroy(spout_handle h);
/* Free C string memory returned by this glue. */
void spout_string_free(char* s);

/* ---- Sender ---- */
void spout_sender_set_name(spout_handle h, const char* name);
void spout_sender_set_format(spout_handle h, unsigned int dxgi_format);
bool spout_sender_send_texture(spout_handle h, unsigned int tex_id, unsigned int target,
    unsigned int width, unsigned int height, bool invert);
bool spout_sender_send_fbo(spout_handle h, unsigned int fbo_id, unsigned int width, unsigned int height, bool invert);
bool spout_sender_send_image(spout_handle h, const unsigned char* pixels, unsigned int width, unsigned int height,
    unsigned int gl_format, bool invert);
void spout_sender_release(spout_handle h);
bool spout_sender_is_initialized(spout_handle h);
unsigned int spout_sender_get_width(spout_handle h);
unsigned int spout_sender_get_height(spout_handle h);
char* spout_sender_get_name(spout_handle h);
unsigned int spout_sender_get_format(spout_handle h);
double spout_sender_get_fps(spout_handle h);
long spout_sender_get_frame(spout_handle h);

/* ---- Receiver ---- */
void spout_receiver_set_name(spout_handle h, const char* sender_name);
bool spout_receiver_receive_texture(spout_handle h, unsigned int tex_id, unsigned int target, bool invert);
bool spout_receiver_receive_image(spout_handle h, unsigned char* pixels, unsigned int gl_format, bool invert);
void spout_receiver_release(spout_handle h);
bool spout_receiver_get_sender_name(spout_handle h, char* buf, int max_chars);
bool spout_receiver_is_frame_new(spout_handle h);
bool spout_receiver_is_updated(spout_handle h);
bool spout_receiver_is_connected(spout_handle h);
unsigned int spout_receiver_get_sender_width(spout_handle h);
unsigned int spout_receiver_get_sender_height(spout_handle h);
unsigned int spout_receiver_get_sender_format(spout_handle h);
double spout_receiver_get_sender_fps(spout_handle h);
long spout_receiver_get_sender_frame(spout_handle h);

/* ---- Bind shared texture (receiver; use GetSharedTextureID after bind) ---- */
bool spout_bind_shared_texture(spout_handle h);
bool spout_unbind_shared_texture(spout_handle h);
unsigned int spout_get_shared_texture_id(spout_handle h);

/* ---- Sender list / discovery ---- */
int spout_get_sender_count(spout_handle h);
bool spout_get_sender_name(spout_handle h, int index, char* buf, int max_chars);
bool spout_find_sender_name(spout_handle h, const char* sendername);
bool spout_get_active_sender(spout_handle h, char* buf, int max_chars);
bool spout_set_active_sender(spout_handle h, const char* sendername);
bool spout_get_sender_info(spout_handle h, const char* sendername, unsigned int* out_width, unsigned int* out_height,
    void** out_handle, unsigned int* out_format);

/* ---- Frame sync ---- */
void spout_set_frame_sync(spout_handle h, const char* sendername);
bool spout_wait_frame_sync(spout_handle h, const char* sendername, unsigned int timeout_ms);
void spout_enable_frame_sync(spout_handle h, bool enabled);
void spout_close_frame_sync(spout_handle h);
bool spout_is_frame_sync_enabled(spout_handle h);

/* ---- Memory buffer ---- */
bool spout_write_memory_buffer(spout_handle h, const char* sendername, const char* data, int length);
int spout_read_memory_buffer(spout_handle h, const char* sendername, char* data, int max_length);

/* ---- Sender count / buffer mode / CPU mode ---- */
int spout_get_max_senders(spout_handle h);
bool spout_get_buffer_mode(spout_handle h);
void spout_set_buffer_mode(spout_handle h, bool active);
int spout_get_buffers(spout_handle h);
void spout_set_buffers(spout_handle h, int buffers);
bool spout_get_cpu_mode(spout_handle h);
bool spout_set_cpu_mode(spout_handle h, bool cpu_mode);

#ifdef __cplusplus
}
#endif

#endif /* SPOUT_GLUE_H */
