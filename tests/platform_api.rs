#[cfg(target_os = "macos")]
mod macos {
    use rusty_syphon_spout::*;

    #[test]
    fn syphon_api_surface_compiles() {
        let _: fn() -> Option<CGLContextObj> = cgl_create_headless_context;
        let _: fn(CGLContextObj) = cgl_destroy_context;
        let _: fn(CGLContextObj) = cgl_make_current;
        let _: fn(usize, usize, &[u8]) -> u32 = gl_create_texture_rectangle_rgba8;
        let _: fn(u32, usize, usize, &mut [u8]) = gl_read_texture_rectangle_rgba8;
        let _: fn(u32) = gl_delete_texture;
        let _: fn() -> Option<String> = notification_name_server_announce;
        let _: fn() -> Option<String> = notification_name_server_update;
        let _: fn() -> Option<String> = notification_name_server_retire;
        let _: fn() -> Option<ServerDirectory> = ServerDirectory::shared;
        let _: fn() -> Option<SyphonOptions> = SyphonOptions::new;
        let _: fn(Option<&str>, CGLContextObj, Option<&SyphonOptions>) -> Option<OpenGLServer> =
            OpenGLServer::new;
        let _: fn(&ServerDescription, CGLContextObj, Option<&std::collections::HashMap<String, String>>, Option<NewFrameCallback>) -> Option<OpenGLClient> =
            OpenGLClient::new;
        let _: fn(Option<&str>, MTLDevicePtr, Option<&SyphonOptions>) -> Option<MetalServer> =
            MetalServer::new;
        let _: fn(&ServerDescription, MTLDevicePtr, Option<&std::collections::HashMap<String, String>>, Option<NewFrameCallback>) -> Option<MetalClient> =
            MetalClient::new;
    }

    #[test]
    fn cgl_context_smoke() {
        if let Some(ctx) = cgl_create_headless_context() {
            cgl_make_current(ctx);
            cgl_destroy_context(ctx);
        }
    }

    #[test]
    fn notification_name_queries_do_not_panic() {
        let _ = notification_name_server_announce();
        let _ = notification_name_server_update();
        let _ = notification_name_server_retire();
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use rusty_syphon_spout::*;

    #[test]
    fn spout_api_surface_compiles() {
        let _: fn() -> Option<Spout> = Spout::new;
        let _: fn(&Spout, Option<&str>) = Spout::sender_set_name;
        let _: fn(&Spout, u32) = Spout::sender_set_format;
        let _: fn(&Spout, u32, u32, u32, u32, bool) -> bool = Spout::sender_send_texture;
        let _: fn(&Spout, u32, u32, u32, bool) -> bool = Spout::sender_send_fbo;
        let _: fn(&Spout, &[u8], u32, u32, u32, bool) -> bool = Spout::sender_send_image;
        let _: fn(&Spout) = Spout::sender_release;
        let _: fn(&Spout) -> bool = Spout::sender_is_initialized;
        let _: fn(&Spout) -> u32 = Spout::sender_width;
        let _: fn(&Spout) -> u32 = Spout::sender_height;
        let _: fn(&Spout) -> Option<String> = Spout::sender_name;
        let _: fn(&Spout) -> u32 = Spout::sender_format;
        let _: fn(&Spout) -> f64 = Spout::sender_fps;
        let _: fn(&Spout) -> i64 = Spout::sender_frame;
        let _: fn(&Spout, Option<&str>) = Spout::receiver_set_name;
        let _: fn(&Spout, u32, u32, bool) -> bool = Spout::receiver_receive_texture;
        let _: fn(&Spout, &mut [u8], u32, bool) -> bool = Spout::receiver_receive_image;
        let _: fn(&Spout) = Spout::receiver_release;
        let _: fn(&Spout) -> Option<String> = Spout::receiver_sender_name;
        let _: fn(&Spout) -> bool = Spout::receiver_is_frame_new;
        let _: fn(&Spout) -> bool = Spout::receiver_is_updated;
        let _: fn(&Spout) -> bool = Spout::receiver_is_connected;
        let _: fn(&Spout) -> u32 = Spout::receiver_sender_width;
        let _: fn(&Spout) -> u32 = Spout::receiver_sender_height;
        let _: fn(&Spout) -> u32 = Spout::receiver_sender_format;
        let _: fn(&Spout) -> f64 = Spout::receiver_sender_fps;
        let _: fn(&Spout) -> i64 = Spout::receiver_sender_frame;
        let _: fn(&Spout) -> bool = Spout::bind_shared_texture;
        let _: fn(&Spout) -> bool = Spout::unbind_shared_texture;
        let _: fn(&Spout) -> u32 = Spout::shared_texture_id;
        let _: fn(&Spout) -> i32 = Spout::sender_count;
        let _: fn(&Spout, i32) -> Option<String> = Spout::sender_name_at;
        let _: fn(&Spout, &str) -> bool = Spout::find_sender_name;
        let _: fn(&Spout) -> Option<String> = Spout::active_sender;
        let _: fn(&Spout, Option<&str>) -> bool = Spout::set_active_sender;
        let _: fn(&Spout, &str) -> Option<SpoutSenderInfo> = Spout::sender_info;
        let _: fn(&Spout, Option<&str>) = Spout::set_frame_sync;
        let _: fn(&Spout, Option<&str>, u32) -> bool = Spout::wait_frame_sync;
        let _: fn(&Spout, bool) = Spout::enable_frame_sync;
        let _: fn(&Spout) = Spout::close_frame_sync;
        let _: fn(&Spout) -> bool = Spout::is_frame_sync_enabled;
        let _: fn(&Spout, &str, &[u8]) -> bool = Spout::write_memory_buffer;
        let _: fn(&Spout, &str, &mut [u8]) -> usize = Spout::read_memory_buffer;
        let _: fn(&Spout) -> i32 = Spout::max_senders;
        let _: fn(&Spout) -> bool = Spout::buffer_mode;
        let _: fn(&Spout, bool) = Spout::set_buffer_mode;
        let _: fn(&Spout) -> i32 = Spout::buffers;
        let _: fn(&Spout, i32) = Spout::set_buffers;
        let _: fn(&Spout) -> bool = Spout::cpu_mode;
        let _: fn(&Spout, bool) -> bool = Spout::set_cpu_mode;
    }

    #[test]
    fn spout_sender_info_is_cloneable() {
        let info = SpoutSenderInfo {
            width: 1920,
            height: 1080,
            share_handle: std::ptr::null_mut(),
            format: 0,
        };
        let cloned = info.clone();
        assert_eq!(cloned.width, info.width);
        assert_eq!(cloned.height, info.height);
        assert_eq!(cloned.format, info.format);
    }
}
