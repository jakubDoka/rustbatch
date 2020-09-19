use crate::math::mat;
use crate::render::program::Program;
use crate::render::texture::Texture;

pub struct Batch {
    pub(crate) vertices: Vec<f32>,
    pub(crate) indices: Vec<u32>,
    pub(crate) texture: Texture,
    pub(crate) program: Program,
}

impl Batch {
    pub fn new(texture: Texture) -> Batch {
        let program = Program::default();
        program.set_texture_size(texture.size());
        program.set_camera(mat::IM);
        Self::from_program(texture, program)
    }

    pub fn from_program(texture: Texture, program: Program) -> Batch {
        Batch{texture, program, vertices: Vec::new(), indices: Vec::new()}
    }

    pub(crate) fn append(&mut self, data: &mut Vec<f32>, pattern: &mut Vec<u32>) {
        let l = self.vertices.len();

        self.vertices.append(data);

        for i in 0..pattern.len() {
            pattern[i] += l as u32;
        }

        self.indices.append(pattern);
    }

    pub fn resize(&mut self, vertex_cap: usize, indices_cap: usize) {
        self.vertices = Vec::with_capacity(vertex_cap);
        self.indices = Vec::with_capacity(indices_cap);
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}
