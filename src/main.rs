use image::{DynamicImage, GenericImageView};
use std::path::Path;
use std::{ptr::null, ffi::c_void};
use std::mem;

use memoffset::offset_of;

use nalgebra_glm::{Vec3, Mat4, Vec2};

mod input;
mod shader;
mod window;
mod shader_program;
mod camera;

use shader_program::ShaderProgram;
use camera::Camera;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
struct Vertex {
    pub position: Vec3,
    pub tex_coords: Vec2
}

struct Cube {
    pub verticies:  [Vertex; 36],
}


static VERTICES: [Vec3; 8] = [
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    
    Vec3::new(0.0, 1.0, 1.0),
    Vec3::new(1.0, 1.0, 1.0),
    Vec3::new(1.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
];

static TEXCOORDS: [Vec2; 6] = [
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 0.0),
];

static INDICES: [u32; 36] = [
    // Front face
    0, 1, 2,
    2, 3, 0,

    // Top face
    1, 4, 5,
    5, 2, 1,

    // Right face
    2, 5, 6,
    6, 3, 2,

    // Bottom face
    7, 6, 5,
    5, 4, 7,

    // Back face
    4, 0, 3,
    3, 7, 4,

    // Left face
    7, 3, 6,
    6, 3, 2,
];

fn main() {
    let mut window = window::Window::new(1920, 1080, "test");
    window.load_gl();
    
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ibo: u32 = 0;

    let mut last_mouse_x: f64 = 0.0;
    let mut last_mouse_y: f64 = 0.0;

    let shader_program = ShaderProgram::new("vertex.glsl", "fragment.glsl");
    shader_program.bind();

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 0.1);
    
    let mut input = input::Input::new();

    let mut cube: Cube = Cube {
        verticies: [Vertex {
            position: Vec3::zeros(),
            tex_coords: Vec2::zeros(),
        }; 36],
    };

    for vertex_index in 0..INDICES.len() {
        let cube_vertex_index = INDICES[vertex_index] as usize;
        cube.verticies[vertex_index].position = VERTICES[cube_vertex_index];
        cube.verticies[vertex_index].tex_coords = TEXCOORDS[vertex_index % 6];
    }
    
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);

        gl::Viewport(0, 0, window.get_window_width() as i32, window.get_window_height() as i32);

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(&cube.verticies) as isize , cube.verticies.as_ptr() as *const c_void, gl::STATIC_DRAW);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Vertex>() as i32,
            offset_of!(Vertex, position) as *const c_void,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Vertex>() as i32,
            offset_of!(Vertex, tex_coords) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, mem::size_of_val(&INDICES) as isize, INDICES.as_ptr() as *const c_void, gl::STATIC_DRAW);

        let model_matrix = Mat4::identity();
        let model_matrix_location = gl::GetUniformLocation(shader_program.program_id, "u_Model\0".as_ptr() as *const i8);
        gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, model_matrix.as_ptr());
    }

    let texture_path = Path::new("assets/grass_side.png");
    let texture_image = image::open(texture_path).expect("Failed to open texture image");
    let texture_rgba = texture_image.to_rgba8();
    let (texture_width, texture_height) = texture_image.dimensions();

    let mut texture_id = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            texture_width as i32,
            texture_height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            texture_rgba.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    while !window.should_close() {
        window.poll_events();
        window.swap_buffers();

        let (x, y) = glfw::Window::get_cursor_pos(&window.native_window);

        // Calculate the mouse movement since the last frame
        let mouse_dx = x - last_mouse_x;
        let mouse_dy = y - last_mouse_y;

        // Update the last known mouse position
        last_mouse_x = x;
        last_mouse_y = y;

        // Perform actions based on the mouse movement
        // For example, you can adjust the camera's orientation based on mouse movement
        camera.process_mouse_movement(mouse_dx, mouse_dy);

        input.handle_window_events(&window.events);
        

        if input.is_key_down(glfw::Key::W) {
            camera.position += camera.front * 0.001;
        }
        if input.is_key_down(glfw::Key::S) {
            camera.position -= camera.front * 0.001;
        }
        if input.is_key_down(glfw::Key::A) {
            camera.position -= nalgebra_glm::normalize(&nalgebra_glm::cross(&camera.front, &camera.up)) * 0.001;
        }
        if input.is_key_down(glfw::Key::D) {
            camera.position += nalgebra_glm::normalize(&nalgebra_glm::cross(&camera.front, &camera.up)) * 0.001;
        }

        if input.is_key_down(glfw::Key::Space) {
            camera.position += camera.up * 0.001
        }

        if input.is_key_down(glfw::Key::LeftControl) {
            camera.position -= camera.up * 0.001
        }

        if input.is_key_down(glfw::Key::Escape) {
            window.native_window.set_should_close(true);
        }
        
        //let view_matrix = nalgebra_glm::look_at_rh(&Vec3::new(0.0, 0.0, 3.0), &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 1.0, 0.0));
        let projection_matrix = nalgebra_glm::perspective_rh(window.get_window_aspect_ratio(), 70.0, 0.1, 100000.0);

        unsafe {
            gl::ClearColor(0.5, 0.8, 0.9, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let view_matrix_location = gl::GetUniformLocation(shader_program.program_id, "u_View\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(view_matrix_location, 1, gl::FALSE, camera.get_view_matrix().as_ptr());

            let projection_matrix_location = gl::GetUniformLocation(shader_program.program_id, "u_Projection\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(projection_matrix_location, 1, gl::FALSE, projection_matrix.as_ptr());

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, INDICES.len() as i32, gl::UNSIGNED_INT, null());
        }
    }
}