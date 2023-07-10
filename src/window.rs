use std::sync::mpsc::Receiver;

use glfw::{WindowEvent, Glfw, Context};
use image::{
    imageops::{resize, Nearest},
    open as open_image, DynamicImage, ImageBuffer, Rgba,
};

pub struct Window {
    pub glfw: Glfw,
    pub native_window: glfw::Window,
    pub events: Receiver<(f64, WindowEvent)>,

    width: u32,
    height: u32,
}

fn load_icon(path: &str) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let icon: DynamicImage = open_image(path).unwrap();
    let resized_icons: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = vec![
        resize(&icon, 16, 16, Nearest),
        resize(&icon, 32, 32, Nearest),
        resize(&icon, 48, 48, Nearest),
    ];
    resized_icons
}

impl Window {

    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut native_window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        native_window.set_key_polling(true);
        native_window.make_current();
        native_window.set_cursor_mode(glfw::CursorMode::Disabled);
        native_window.set_cursor_pos(0.0, 0.0);

        native_window.set_icon(load_icon("assets/grass_side.png"));

        Window {
            glfw,
            native_window,
            events,
            width,
            height,
        }
    }

    pub fn load_gl(&mut self) {
        gl::load_with(|s| self.native_window.get_proc_address(s) as *const _);
    }

    pub fn get_window_width(&self) -> u32 {
        self.width
    }

    pub fn get_window_height(&self) -> u32 {
        self.height
    }

    pub fn get_window_aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn swap_buffers(&mut self) {
        self.native_window.swap_buffers();
    }

    pub fn should_close(&mut self) -> bool {
        self.native_window.should_close()
    }
}