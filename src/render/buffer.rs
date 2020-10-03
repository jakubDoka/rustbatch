


/// Vertex is vertex representation for opengl that is bit more convenient to work with.
/// Using custom Vertexes also means you have to use custom vertex shader.
/// - size can be from 1 to 4
/// For better of how should layout look like, checkout vertex constants in this module
pub struct Vertex {
    size: usize,
}

impl Vertex {
    pub fn new(size: usize) -> Vertex {
        if size > 4 {
            panic!("illegal vertex size, size can be from 1 to 4");
        }

        Vertex{size}
    }

    fn apply(&self, offset: usize, location: u32, total_size: usize) {
        unsafe {
            gl::EnableVertexAttribArray(location);
            gl::VertexAttribPointer(
                location,
                self.size as i32,
                gl::FLOAT,
                gl::FALSE,
                (total_size * std::mem::size_of::<f32>()) as gl::types::GLint,
                (offset * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }
}


pub const POSITION: Vertex = Vertex{size: 2};
pub const TEXTURE_REGION: Vertex = Vertex{size: 2};
pub const COLOR: Vertex = Vertex{size: 4};

pub const DEFAULT_VERTEX_SIZE: u32 = 8;

/// Buffer is used for customizing how is the vertex data processed
#[derive(Clone)]
pub struct Buffer {
    pub(crate) data_size: usize,
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

impl Buffer {
    /// default returns default buffer
    pub fn default() -> Self {
        Self::new(&[POSITION, TEXTURE_REGION, COLOR])
    }

    pub fn no_texture() -> Self {
        Self::new(&[POSITION, COLOR])
    }

    /// new returns new buffer from Vertexes
    pub fn new(vertexes: &[Vertex]) -> Self {
        if vertexes.len() > 16 {
            panic!("opengl permits only 16 vertexes")
        }

        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
        }

        let mut data_size = 0;
        for v in vertexes {
            data_size += v.size;
        }

        let buff = Buffer{vbo, vao, ebo, data_size};

        buff.bind();

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, buff.vbo);
        }



        let mut offset = 0;
        for (i, v) in vertexes.iter().enumerate() {
            v.apply(offset, i as u32, data_size);
            offset += v.size;
        }

        buff
    }

    /// bind uses the buffer. you still have to call draw afterwards
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    /// set_vertices sets vertices and adds default indices so you can draw
    pub fn set_vertices(&self, vertices: &Vec<f32>) {
        let len = vertices.len();
        self.set_vertices_and_indices(vertices, &(0u32..len as u32).collect());
    }

    /// set_vertices_and_indices sets vertices and custom indices
    pub fn set_vertices_and_indices(&self, vertices: &Vec<f32>, indices: &Vec<u32>) {
        self.bind();
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

    /// draw draws the buffer
    pub fn draw(&self, amount: usize) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::DrawElements(gl::TRIANGLES, amount as i32, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.ebo);
            gl::DeleteBuffers(1, &mut self.vbo);
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}