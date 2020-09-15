

pub trait Target {
    fn append(data: Vec<f32>, tex_id: gl::types::GLuint);
}

pub struct Batch {
    data: Vec<f32>,
    texture: texture::Texture,
    program: program::Program
}