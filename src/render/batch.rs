use crate::render::program::Program;
use crate::render::texture::Texture;
use crate::render::buffer::Buffer;

pub trait Target {
    fn append(&mut self, data: &[f32], pattern: &[u32], vertex_size: u32, program: Option<&Program>, texture: Option<&Texture>, buffer: Option<&Buffer>);
}

pub struct VertexData {
    pub(crate) vertices: Vec<f32>,
    pub(crate) indices: Vec<u32>,
    pub(crate) vertex_size: u32,
}

impl VertexData {
    pub fn new() -> Self {
        Self{ vertices: vec![], indices: vec![], vertex_size: 8 }
    }

    pub fn no_texture() -> Self {
        Self{ vertices: vec![], indices: vec![], vertex_size: 6 }
    }

    pub fn custom(vertex_size: u32) -> Self {
        Self{ vertices: vec![], indices: vec![], vertex_size }
    }

    fn error_check(&self, vertex_size: u32) {
        if self.vertex_size != vertex_size {
            panic!("incorrect vertex size, this vertex data accepts only vertex size {}, but \
               you inputted vertex data with vertex size {}", self.vertex_size, vertex_size);
        }
    }

    pub fn draw<T: Target>(&self, target: &mut T) {
        target.append(&self.vertices, &self.indices, self.vertex_size, None, None, None);
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.vertices.clear();
    }
}

impl Target for VertexData {
    #[inline]
    fn append(&mut self, data: &[f32], pattern: &[u32], vertex_size: u32, _: Option<&Program>, _: Option<&Texture>, _: Option<&Buffer>) {
        self.error_check(vertex_size);

        let offset = self.vertices.len() as u32/ vertex_size;

        self.vertices.extend(data);

        for i in pattern {
            self.indices.push(*i + offset);
        }
    }
}

/// Batch is a core drawing method. You can append vertex data to it and then draw it to window.
/// I made sure you can customize batch however you like. You can for example use custom fragment and
/// vertex shader along with custom buffer and make your own rendering model so 3D is totally possible
/// if you kow how.
pub struct Batch {
    pub data: VertexData,
    pub(crate) texture: Texture,
    pub(crate) program: Program,
    pub(crate) buffer: Option<Buffer>,
}

impl Batch {
    pub fn no_texture() -> Batch {
        Self::customized(Texture::NONE, Program::no_texture(), Some(Buffer::no_texture()))
    }

    /// new returns batch from texture.
    pub fn new(texture: Texture) -> Batch {
        Self::customized(texture, Program::default(), None)
    }

    /// from_program creates batch with custom rendering program
    /// # Example
    /// ```should_fail
    /// use rustbatch::render::program::Program;
    /// use rustbatch::render::shader::Shader;
    /// use rustbatch::{Batch, Texture};
    ///
    /// let texture = Texture::new("your_texture.png", gl::NEAREST, gl::RGBA).unwrap();
    /// let program = Program::from_shaders(&[Shader::default_vertex(), Shader::new("your_shader.frag")]).unwrap();
    /// let batch = Batch::customized(texture, program, none);
    /// ```
    pub fn customized(texture: Texture, program: Program, buffer: Option<Buffer>) -> Batch {
        program.set_texture_size(texture.size());
        let data = match &buffer {
            Some(b) => {
                VertexData::custom(b.data_size as u32)
            }
            None => VertexData::new()
        };
        Batch{texture, program, data, buffer}
    }

    pub fn draw<T: Target>(&self, target: &mut T) {
        let buffer = match &self.buffer {
            Some(b) => Some(b),
            None => None,
        };
        target.append(&self.data.vertices, &self.data.indices, self.data.vertex_size, Some(&self.program), Some(&self.texture), buffer)
    }

    /// clear clears batch
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Target for Batch {
    /// append appends vertex data to texture.
    /// pattern specifies how data is used. If you for example provide 4 vertexes you can make
    /// rectangle from it with pattern [0, 2, 3, 1, 2, 3]. If you don't know what i mean try to
    /// draw four points on paper, index them from 0 to 3 and connect them with lines in specified
    /// order.
    ///
    /// # Panics
    ///
    /// If batch has custom buffer and buffers size do not corresponds to `vertex_size` you will get
    /// panic. This is mainly to prevent confusion in case of providing incorrect vertex data
    /// structure.
    #[inline]
    fn append(&mut self, data: &[f32], pattern: &[u32], vertex_size: u32, _: Option<&Program>, _: Option<&Texture>, _: Option<&Buffer>) {
        self.data.append(data, pattern, vertex_size, None, None, None);
    }
}
