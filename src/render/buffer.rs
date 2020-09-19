use sdl2::pixels::Color;

use crate::render::program::Program;
use crate::render::texture::Texture;

pub struct Buffer {
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

pub struct VertexProperties {
    pub size: i32,
    pub offset: usize,
    pub location: u32,
}

impl VertexProperties {
    fn apply(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.location);
            gl::VertexAttribPointer(
                self.location,
                self.size,
                gl::FLOAT,
                gl::FALSE,
                (DATA_SIZE * std::mem::size_of::<f32>()) as gl::types::GLint,
                (self.offset * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }
}


pub const POSITION: VertexProperties = VertexProperties{size: 2, location: 0, offset: 0};
pub const TEXTURE_REGION: VertexProperties = VertexProperties{size: 2, location: 1, offset: 2};
pub const COLOR: VertexProperties = VertexProperties{size: 4, location: 2, offset: 4};
pub const DATA_SIZE: usize = (POSITION.size + TEXTURE_REGION.size + COLOR.size) as usize;

impl Buffer {
    pub fn new() -> Self {
        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
        }

        Buffer{vbo, vao, ebo}.init()
    }

    pub fn init(self) -> Self {
        self.set_used();

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
            POSITION.apply();

            TEXTURE_REGION.apply();

            COLOR.apply();

        self
    }

    pub fn set_used(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn set_vertices(&self, vertices: &Vec<f32>) {
        let len = vertices.len();
        self.set_vertices_and_indices(vertices, &(0u32..len as u32).collect());
    }

    pub fn set_vertices_and_indices(&self, vertices: &Vec<f32>, indices: &Vec<u32>) {
        self.set_used();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }
    }

    pub fn draw(&self, amount: usize) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::DrawElements(gl::TRIANGLES, amount as i32, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}