use nalgebra_glm::{Vec3, Mat4};

pub struct Camera {
    pub position: Vec3,
    pub up: Vec3,
    pub front: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

impl Camera {
    pub fn new(position: Vec3, up: Vec3, front: Vec3, sensitivity: f32) -> Camera {
        Camera {
            position,
            up,
            front: nalgebra_glm::normalize(&front),
            yaw: 90.0,
            pitch: 0.0,
            sensitivity,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        nalgebra_glm::look_at_rh(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn process_mouse_movement(&mut self, mouse_dx: f64, mouse_dy: f64) {
        let dx = mouse_dx as f32 * self.sensitivity;
        let dy = mouse_dy as f32 * self.sensitivity;

        self.yaw += dx;
        self.pitch -= dy;

        const MAX_PITCH: f32 = 89.0;
        const MIN_PITCH: f32 = -89.0;
        self.pitch = self.pitch.max(MIN_PITCH).min(MAX_PITCH);

        let yaw_radians = self.yaw.to_radians();
        let pitch_radians = self.pitch.to_radians();

        self.front = Vec3::new(
            yaw_radians.cos() * pitch_radians.cos(),
            pitch_radians.sin(),
            yaw_radians.sin() * pitch_radians.cos(),
        )
        .normalize();
    }
}