/*
 * spout_glue.cpp - C wrapper for SpoutLibrary (Windows only).
 */
#ifdef _WIN32

#include "SpoutLibrary.h"
#include "spout_glue.h"
#include <cstdlib>
#include <cstring>

static SPOUTLIBRARY* ptr(spout_handle h) {
    return reinterpret_cast<SPOUTLIBRARY*>(h);
}

spout_handle spout_create(void) {
    return reinterpret_cast<spout_handle>(GetSpout());
}

void spout_destroy(spout_handle h) {
    if (h) ptr(h)->Release();
}

void spout_string_free(char* s) {
    if (s) free(s);
}

void spout_sender_set_name(spout_handle h, const char* name) {
    if (h) ptr(h)->SetSenderName(name);
}

void spout_sender_set_format(spout_handle h, unsigned int dxgi_format) {
    if (h) ptr(h)->SetSenderFormat((DWORD)dxgi_format);
}

bool spout_sender_send_texture(spout_handle h, unsigned int tex_id, unsigned int target,
    unsigned int width, unsigned int height, bool invert) {
    return h && ptr(h)->SendTexture(tex_id, target, width, height, invert, 0);
}

void spout_sender_release(spout_handle h) {
    if (h) ptr(h)->ReleaseSender(0);
}

bool spout_sender_is_initialized(spout_handle h) {
    return h && ptr(h)->IsInitialized();
}

unsigned int spout_sender_get_width(spout_handle h) {
    return h ? ptr(h)->GetWidth() : 0;
}

unsigned int spout_sender_get_height(spout_handle h) {
    return h ? ptr(h)->GetHeight() : 0;
}

char* spout_sender_get_name(spout_handle h) {
    if (!h) return nullptr;
    const char* s = ptr(h)->GetName();
    return s ? _strdup(s) : nullptr;
}

unsigned int spout_sender_get_format(spout_handle h) {
    return h ? (unsigned int)ptr(h)->GetSenderFormat() : 0;
}

double spout_sender_get_fps(spout_handle h) {
    return h ? ptr(h)->GetFps() : 0.0;
}

long spout_sender_get_frame(spout_handle h) {
    return h ? ptr(h)->GetFrame() : 0;
}

bool spout_sender_send_fbo(spout_handle h, unsigned int fbo_id, unsigned int width, unsigned int height, bool invert) {
    return h && ptr(h)->SendFbo(fbo_id, width, height, invert);
}

bool spout_sender_send_image(spout_handle h, const unsigned char* pixels, unsigned int width, unsigned int height,
    unsigned int gl_format, bool invert) {
    return h && ptr(h)->SendImage(pixels, width, height, static_cast<GLenum>(gl_format), invert);
}

void spout_receiver_set_name(spout_handle h, const char* sender_name) {
    if (h) ptr(h)->SetReceiverName(sender_name);
}

bool spout_receiver_receive_texture(spout_handle h, unsigned int tex_id, unsigned int target, bool invert) {
    return h && ptr(h)->ReceiveTexture(tex_id, target, invert, 0);
}

bool spout_receiver_receive_image(spout_handle h, unsigned char* pixels, unsigned int gl_format, bool invert) {
    return h && ptr(h)->ReceiveImage(pixels, static_cast<GLenum>(gl_format), invert, 0);
}

void spout_receiver_release(spout_handle h) {
    if (h) ptr(h)->ReleaseReceiver();
}

bool spout_receiver_get_sender_name(spout_handle h, char* buf, int max_chars) {
    if (!h || !buf || max_chars <= 0) return false;
    return ptr(h)->GetReceiverName(buf, max_chars);
}

bool spout_receiver_is_frame_new(spout_handle h) {
    return h && ptr(h)->IsFrameNew();
}

bool spout_receiver_is_connected(spout_handle h) {
    return h && ptr(h)->IsConnected();
}

unsigned int spout_receiver_get_sender_width(spout_handle h) {
    return h ? ptr(h)->GetSenderWidth() : 0;
}

unsigned int spout_receiver_get_sender_height(spout_handle h) {
    return h ? ptr(h)->GetSenderHeight() : 0;
}

unsigned int spout_receiver_get_sender_format(spout_handle h) {
    return h ? (unsigned int)ptr(h)->GetSenderFormat() : 0;
}

bool spout_receiver_is_updated(spout_handle h) {
    return h && ptr(h)->IsUpdated();
}

double spout_receiver_get_sender_fps(spout_handle h) {
    return h ? ptr(h)->GetSenderFps() : 0.0;
}

long spout_receiver_get_sender_frame(spout_handle h) {
    return h ? ptr(h)->GetSenderFrame() : 0;
}

bool spout_bind_shared_texture(spout_handle h) {
    return h && ptr(h)->BindSharedTexture();
}

bool spout_unbind_shared_texture(spout_handle h) {
    return h && ptr(h)->UnBindSharedTexture();
}

unsigned int spout_get_shared_texture_id(spout_handle h) {
    return h ? ptr(h)->GetSharedTextureID() : 0;
}

int spout_get_sender_count(spout_handle h) {
    return h ? ptr(h)->GetSenderCount() : 0;
}

bool spout_get_sender_name(spout_handle h, int index, char* buf, int max_chars) {
    if (!h || !buf || max_chars <= 0) return false;
    return ptr(h)->GetSender(index, buf, max_chars);
}

bool spout_find_sender_name(spout_handle h, const char* sendername) {
    return h && sendername && ptr(h)->FindSenderName(sendername);
}

bool spout_get_active_sender(spout_handle h, char* buf, int max_chars) {
    if (!h || !buf || max_chars <= 0) return false;
    char active[256] = {};
    bool ok = ptr(h)->GetActiveSender(active);
    if (!ok) {
        buf[0] = '\0';
        return false;
    }
    std::strncpy(buf, active, (size_t)(max_chars - 1));
    buf[max_chars - 1] = '\0';
    return true;
}

bool spout_set_active_sender(spout_handle h, const char* sendername) {
    return h && ptr(h)->SetActiveSender(sendername);
}

bool spout_get_sender_info(spout_handle h, const char* sendername, unsigned int* out_width, unsigned int* out_height,
    void** out_handle, unsigned int* out_format) {
    if (!h || !sendername) return false;
    unsigned int w = 0, hh = 0;
    DWORD fmt = 0;
    HANDLE handle = nullptr;
    bool ok = ptr(h)->GetSenderInfo(sendername, w, hh, handle, fmt);
    if (ok) {
        if (out_width) *out_width = w;
        if (out_height) *out_height = hh;
        if (out_handle) *out_handle = handle;
        if (out_format) *out_format = fmt;
    }
    return ok;
}

void spout_set_frame_sync(spout_handle h, const char* sendername) {
    if (h) ptr(h)->SetFrameSync(sendername);
}

bool spout_wait_frame_sync(spout_handle h, const char* sendername, unsigned int timeout_ms) {
    return h && ptr(h)->WaitFrameSync(sendername, (DWORD)timeout_ms);
}

void spout_enable_frame_sync(spout_handle h, bool enabled) {
    if (h) ptr(h)->EnableFrameSync(enabled);
}

void spout_close_frame_sync(spout_handle h) {
    if (h) ptr(h)->CloseFrameSync();
}

bool spout_is_frame_sync_enabled(spout_handle h) {
    return h && ptr(h)->IsFrameSyncEnabled();
}

bool spout_write_memory_buffer(spout_handle h, const char* sendername, const char* data, int length) {
    return h && sendername && data && length >= 0 && ptr(h)->WriteMemoryBuffer(sendername, data, length);
}

int spout_read_memory_buffer(spout_handle h, const char* sendername, char* data, int max_length) {
    if (!h || !sendername || !data || max_length <= 0) return 0;
    return ptr(h)->ReadMemoryBuffer(sendername, data, max_length);
}

int spout_get_max_senders(spout_handle h) {
    return h ? ptr(h)->GetMaxSenders() : 0;
}

bool spout_get_buffer_mode(spout_handle h) {
    return h && ptr(h)->GetBufferMode();
}

void spout_set_buffer_mode(spout_handle h, bool active) {
    if (h) ptr(h)->SetBufferMode(active);
}

int spout_get_buffers(spout_handle h) {
    return h ? ptr(h)->GetBuffers() : 0;
}

void spout_set_buffers(spout_handle h, int buffers) {
    if (h) ptr(h)->SetBuffers(buffers);
}

bool spout_get_cpu_mode(spout_handle h) {
    return h && ptr(h)->GetCPUmode();
}

bool spout_set_cpu_mode(spout_handle h, bool cpu_mode) {
    return h && ptr(h)->SetCPUmode(cpu_mode);
}

#endif /* _WIN32 */
