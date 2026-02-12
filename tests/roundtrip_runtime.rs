//! Runtime roundtrip tests for Syphon (macOS) and Spout (Windows).
//!
//! These are intentionally `#[ignore]` because they require a working local
//! graphics/runtime environment and are not stable on generic CI runners.

const W: usize = 64;
const H: usize = 64;
const BPP: usize = 4;
const BYTES: usize = W * H * BPP;

fn make_test_pattern() -> Vec<u8> {
    let mut data = vec![0u8; BYTES];
    for y in 0..H {
        for x in 0..W {
            let i = (y * W + x) * BPP;
            data[i] = (x % 256) as u8;
            data[i + 1] = (y % 256) as u8;
            data[i + 2] = ((x + y) % 256) as u8;
            data[i + 3] = 255;
        }
    }
    data
}

#[cfg(target_os = "macos")]
mod macos {
    use super::*;
    use rusty_syphon_spout::{
        cgl_create_headless_context, cgl_destroy_context, cgl_make_current,
        gl_create_texture_rectangle_rgba8, gl_delete_texture, gl_read_texture_rectangle_rgba8,
        OpenGLClient, OpenGLServer, GL_TEXTURE_RECTANGLE,
    };
    use std::time::Duration;

    #[test]
    #[ignore = "requires local Syphon/OpenGL runtime (headless CGL + framework availability)"]
    fn syphon_opengl_roundtrip_runtime() {
        let ctx = cgl_create_headless_context().expect("create headless CGL context");
        cgl_make_current(ctx);

        let pattern = make_test_pattern();
        let tex_id = gl_create_texture_rectangle_rgba8(W, H, &pattern);
        assert!(tex_id != 0, "failed to create sender texture");

        let server = OpenGLServer::new(Some("rusty-syphon-runtime-roundtrip"), ctx, None)
            .expect("failed to create OpenGL server");
        server.publish_frame(
            tex_id,
            GL_TEXTURE_RECTANGLE,
            0.0,
            0.0,
            W as f64,
            H as f64,
            W as f64,
            H as f64,
            false,
        );

        let desc = server.server_description().expect("missing server description");
        let client = OpenGLClient::new(&desc, ctx, None, None).expect("failed to create client");

        for _ in 0..100 {
            if client.has_new_frame() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(client.has_new_frame(), "no new frame observed");

        let image = client.new_frame_image().expect("missing frame image");
        let recv_tex = image.texture_name();
        let mut readback = vec![0u8; BYTES];
        gl_read_texture_rectangle_rgba8(recv_tex, W, H, &mut readback);
        drop(image);

        gl_delete_texture(tex_id);
        cgl_destroy_context(ctx);

        assert_eq!(pattern, readback, "Syphon OpenGL roundtrip mismatch");
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use rusty_syphon_spout::Spout;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    const GL_RGBA: u32 = 0x1908;

    #[test]
    #[ignore = "requires local Spout runtime/driver stack (sender/receiver interop)"]
    fn spout_image_roundtrip_runtime() {
        let sender = Spout::new().expect("failed to create sender");
        let receiver = Spout::new().expect("failed to create receiver");

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_millis();
        let sender_name = format!("rusty-spout-runtime-roundtrip-{nonce}");

        // CPU mode makes image path less dependent on local GL state.
        let _ = sender.set_cpu_mode(true);
        let _ = receiver.set_cpu_mode(true);

        sender.sender_set_name(Some(&sender_name));
        receiver.receiver_set_name(Some(&sender_name));

        let pattern = make_test_pattern();
        assert!(
            sender.sender_send_image(&pattern, W as u32, H as u32, GL_RGBA, false),
            "failed to send image"
        );

        let mut out = vec![0u8; BYTES];
        let mut got = false;
        for _ in 0..100 {
            if receiver.receiver_receive_image(&mut out, GL_RGBA, false) {
                got = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(got, "failed to receive image from sender");
        assert_eq!(pattern, out, "Spout image roundtrip mismatch");
    }
}
