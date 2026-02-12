/*
 * spout_glue_stub.c - No-op implementation of spout_glue API for macOS.
 * Spout is Windows-only; this stub allows the build to compile and run
 * bindgen for spout_glue.h on macOS so both code paths are exercised.
 */
#include "spout_glue.h"
#include <stdbool.h>
#include <stddef.h>
#include <string.h>

spout_handle spout_create(void) {
    return NULL;
}

void spout_destroy(spout_handle h) {
    (void)h;
}

void spout_string_free(char* s) {
    (void)s;
}

void spout_sender_set_name(spout_handle h, const char* name) {
    (void)h;
    (void)name;
}

void spout_sender_set_format(spout_handle h, unsigned int dxgi_format) {
    (void)h;
    (void)dxgi_format;
}

bool spout_sender_send_fbo(spout_handle h, unsigned int fbo_id, unsigned int width, unsigned int height, bool invert) {
    (void)h;(void)fbo_id;(void)width;(void)height;(void)invert;
    return false;
}

bool spout_sender_send_image(spout_handle h, const unsigned char* pixels, unsigned int width, unsigned int height,
    unsigned int gl_format, bool invert) {
    (void)h;(void)pixels;(void)width;(void)height;(void)gl_format;(void)invert;
    return false;
}

bool spout_sender_send_texture(spout_handle h, unsigned int tex_id, unsigned int target,
    unsigned int width, unsigned int height, bool invert) {
    (void)h;
    (void)tex_id;
    (void)target;
    (void)width;
    (void)height;
    (void)invert;
    return false;
}

void spout_sender_release(spout_handle h) {
    (void)h;
}

bool spout_sender_is_initialized(spout_handle h) {
    (void)h;
    return false;
}

unsigned int spout_sender_get_width(spout_handle h) {
    (void)h;
    return 0;
}

unsigned int spout_sender_get_height(spout_handle h) {
    (void)h;
    return 0;
}

char* spout_sender_get_name(spout_handle h) {
    (void)h;
    return NULL;
}

unsigned int spout_sender_get_format(spout_handle h) {
    (void)h;
    return 0;
}

double spout_sender_get_fps(spout_handle h) {
    (void)h;
    return 0.0;
}

long spout_sender_get_frame(spout_handle h) {
    (void)h;
    return 0;
}

void spout_receiver_set_name(spout_handle h, const char* sender_name) {
    (void)h;
    (void)sender_name;
}

bool spout_receiver_receive_texture(spout_handle h, unsigned int tex_id, unsigned int target, bool invert) {
    (void)h;
    (void)tex_id;
    (void)target;
    (void)invert;
    return false;
}

bool spout_receiver_receive_image(spout_handle h, unsigned char* pixels, unsigned int gl_format, bool invert) {
    (void)h;(void)pixels;(void)gl_format;(void)invert;
    return false;
}

void spout_receiver_release(spout_handle h) {
    (void)h;
}

bool spout_receiver_get_sender_name(spout_handle h, char* buf, int max_chars) {
    (void)h;
    if (buf && max_chars > 0)
        buf[0] = '\0';
    return false;
}

bool spout_receiver_is_frame_new(spout_handle h) {
    (void)h;
    return false;
}

bool spout_receiver_is_updated(spout_handle h) {
    (void)h;
    return false;
}

bool spout_receiver_is_connected(spout_handle h) {
    (void)h;
    return false;
}

unsigned int spout_receiver_get_sender_width(spout_handle h) {
    (void)h;
    return 0;
}

unsigned int spout_receiver_get_sender_height(spout_handle h) {
    (void)h;
    return 0;
}

unsigned int spout_receiver_get_sender_format(spout_handle h) {
    (void)h;
    return 0;
}

double spout_receiver_get_sender_fps(spout_handle h) {
    (void)h;
    return 0.0;
}

long spout_receiver_get_sender_frame(spout_handle h) {
    (void)h;
    return 0;
}

bool spout_bind_shared_texture(spout_handle h) {
    (void)h;
    return false;
}

bool spout_unbind_shared_texture(spout_handle h) {
    (void)h;
    return false;
}

unsigned int spout_get_shared_texture_id(spout_handle h) {
    (void)h;
    return 0;
}

int spout_get_sender_count(spout_handle h) {
    (void)h;
    return 0;
}

bool spout_get_sender_name(spout_handle h, int index, char* buf, int max_chars) {
    (void)h;
    (void)index;
    if (buf && max_chars > 0)
        buf[0] = '\0';
    return false;
}

bool spout_find_sender_name(spout_handle h, const char* sendername) {
    (void)h;
    (void)sendername;
    return false;
}

bool spout_get_active_sender(spout_handle h, char* buf, int max_chars) {
    (void)h;
    if (buf && max_chars > 0)
        buf[0] = '\0';
    return false;
}

bool spout_set_active_sender(spout_handle h, const char* sendername) {
    (void)h;
    (void)sendername;
    return false;
}

bool spout_get_sender_info(spout_handle h, const char* sendername, unsigned int* out_width, unsigned int* out_height,
    void** out_handle, unsigned int* out_format) {
    (void)h;
    (void)sendername;
    if (out_width) *out_width = 0;
    if (out_height) *out_height = 0;
    if (out_handle) *out_handle = NULL;
    if (out_format) *out_format = 0;
    return false;
}

void spout_set_frame_sync(spout_handle h, const char* sendername) {
    (void)h;
    (void)sendername;
}

bool spout_wait_frame_sync(spout_handle h, const char* sendername, unsigned int timeout_ms) {
    (void)h;
    (void)sendername;
    (void)timeout_ms;
    return false;
}

void spout_enable_frame_sync(spout_handle h, bool enabled) {
    (void)h;
    (void)enabled;
}

void spout_close_frame_sync(spout_handle h) {
    (void)h;
}

bool spout_is_frame_sync_enabled(spout_handle h) {
    (void)h;
    return false;
}

bool spout_write_memory_buffer(spout_handle h, const char* sendername, const char* data, int length) {
    (void)h;
    (void)sendername;
    (void)data;
    (void)length;
    return false;
}

int spout_read_memory_buffer(spout_handle h, const char* sendername, char* data, int max_length) {
    (void)h;
    (void)sendername;
    (void)data;
    (void)max_length;
    return 0;
}

int spout_get_max_senders(spout_handle h) {
    (void)h;
    return 0;
}

bool spout_get_buffer_mode(spout_handle h) {
    (void)h;
    return false;
}

void spout_set_buffer_mode(spout_handle h, bool active) {
    (void)h;
    (void)active;
}

int spout_get_buffers(spout_handle h) {
    (void)h;
    return 0;
}

void spout_set_buffers(spout_handle h, int buffers) {
    (void)h;
    (void)buffers;
}

bool spout_get_cpu_mode(spout_handle h) {
    (void)h;
    return false;
}

bool spout_set_cpu_mode(spout_handle h, bool cpu_mode) {
    (void)h;
    (void)cpu_mode;
    return false;
}
