
use crate::math::mat;
use crate::render::program::Program;
use crate::render::texture::Texture;

/// Batch is a core drawing method. You can append vertex data to it and then draw it to window.
/// More about a data structure and how to modify it can be found in buffer module.
pub struct Batch {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    texture: Texture,
    program: Program,
}

impl Batch {
    /// new returns batch from texture.
    pub fn new(texture: Texture) -> Batch {
        Self::from_program(texture, Program::default())
    }

    /// from_program creates batch with custom rendering program
    /// # Example
    /// ```
    /// use rustbatch::render::program::Program;
    /// use rustbatch::render::shader::Shader;
    /// use rustbatch::{Batch, Texture};
    ///
    /// let texture = Texture::new("your_texture.png", gl::NEAREST, gl::RGBA).unwrap();
    /// let program = Program::from_shaders(&[Shader::default_vertex(), Shader::new("your_shader.frag")]).unwrap();
    /// let batch = Batch::from_program(texture, program);
    /// ```
    pub fn from_program(texture: Texture, program: Program) -> Batch {
        program.set_texture_size(texture.size());
        Batch{texture, program, vertices: Vec::new(), indices: Vec::new()}
    }

    /// append appends vertex data to texture.
    /// pattern specifies how data is used. If you for example provide 4 vertexes you can make
    /// rectangle from it with pattern [0, 2, 3, 1, 2, 3]. If you don't know what i mean try to
    /// draw four points on paper, index them from 0 to 3 and connect them with lines in specified
    /// order.
    pub fn append(&mut self, data: &mut Vec<f32>, pattern: &mut Vec<u32>) {
        let l = self.vertices.len();

        self.vertices.append(data);

        for i in 0..pattern.len() {
            pattern[i] += l as u32;
        }

        self.indices.append(pattern);
    }

    /// clear clears batch
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}
