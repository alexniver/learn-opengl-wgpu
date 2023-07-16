use glam::{Mat4, Vec3};
use winit::event::VirtualKeyCode;

use crate::input::Input;

pub struct Camera {
    pub pos: Vec3,
    pub front: Vec3,
    pub up: Vec3,

    fov: f32,
    ratio: f32, // width / height
    z_near: f32,
    z_far: f32,

    speed: f32,
    yaw: f32,
    pitch: f32,

    is_first_mouse_move: bool,
    mouse_sensitivity: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 3.0),
            front: Vec3::NEG_Z,
            up: Vec3::Y,

            fov: 45.0,
            ratio: width / height,
            z_near: 0.1,
            z_far: 100.0,

            speed: 2.0,
            yaw: -90.0,
            pitch: 0.0,

            is_first_mouse_move: true,
            mouse_sensitivity: 0.1,
        }
    }

    pub fn fov(&mut self, scroll: f32) {
        self.fov += scroll;
        self.fov = self.fov.min(60.0);
        self.fov = self.fov.max(10.0);
    }

    pub fn yaw_pitch(&mut self, x: f32, y: f32) {
        if self.is_first_mouse_move {
            self.is_first_mouse_move = false;
        } else {
            self.yaw += x * self.mouse_sensitivity;
            self.pitch -= y * self.mouse_sensitivity;
            self.pitch = self.pitch.min(89.0);
            self.pitch = self.pitch.max(-89.0);

            self.front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
            self.front.y = self.pitch.to_radians().sin();
            self.front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
            self.front = self.front.normalize();
        }
    }

    pub fn moving(&mut self, input: &Input, delta_time: f32) {
        if input.is_pressed(VirtualKeyCode::W) {
            self.move_front(delta_time);
        }
        if input.is_pressed(VirtualKeyCode::S) {
            self.move_back(delta_time);
        }
        if input.is_pressed(VirtualKeyCode::A) {
            self.move_left(delta_time);
        }
        if input.is_pressed(VirtualKeyCode::D) {
            self.move_right(delta_time);
        }

        if input.is_pressed(VirtualKeyCode::Space) {
            self.move_up(delta_time);
        }
        if input.is_pressed(VirtualKeyCode::LShift) {
            self.move_down(delta_time);
        }
    }

    pub fn move_left(&mut self, delta_time: f32) {
        let right = self.front.cross(Vec3::Y);
        self.pos -= right * self.speed * delta_time;
    }

    pub fn move_right(&mut self, delta_time: f32) {
        let right = self.front.cross(Vec3::Y);
        self.pos += right * self.speed * delta_time;
    }

    pub fn move_front(&mut self, delta_time: f32) {
        self.pos += self.front * self.speed * delta_time;
    }

    pub fn move_back(&mut self, delta_time: f32) {
        self.pos -= self.front * self.speed * delta_time;
    }

    pub fn move_up(&mut self, delta_time: f32) {
        self.pos += Vec3::Y * self.speed * delta_time;
    }

    pub fn move_down(&mut self, delta_time: f32) {
        self.pos += Vec3::NEG_Y * self.speed * delta_time;
    }

    pub fn update_size(&mut self, width: f32, height: f32) {
        self.ratio = width / height;
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_to_rh(self.pos, self.front, self.up)
    }

    pub fn proj(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov.to_radians(), self.ratio, self.z_near, self.z_far)
    }
}
