


/// Vertex is vertex representation for opengl that is bit more convenient to work with.
/// Using custom Vertexes also means you have to use custom vertex shader.
/// - size can be from 1 to 4
/// - offset depends on how much size vertexes before this vertex takes so if first vertex is of
/// size 4 second vertex will have offset of 4
/// - location is number between 0 to 16 and it affects how you access data from vertex shader
/// For better of how should layout look checkout vertex constants in this module
pub struct Vertex {
    pub size: i32,
    pub offset: usize,
    pub location: u32,
}

impl Vertex {
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


pub const POSITION: Vertex = Vertex{size: 2, location: 0, offset: 0};
pub const TEXTURE_REGION: Vertex = Vertex{size: 2, location: 1, offset: 2};
pub const COLOR: Vertex = Vertex{size: 4, location: 2, offset: 4};
pub const DATA_SIZE: usize = (POSITION.size + TEXTURE_REGION.size + COLOR.size) as usize;

/// Buffer is used for customizing how is the vertex data processed
#[derive(Clone)]
pub struct Buffer {
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

impl Buffer {
    /// default returns default buffer
    pub fn default() -> Self {
        Self::from_vertexes(&[POSITION, TEXTURE_REGION, COLOR])
    }

    /// new returns new buffer from Vertexes
    pub fn new(vertexes: &[Vertex]) -> Self {
        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
        }

        let buff = Buffer{vbo, vao, ebo};
        buff.set_used();

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, buff.vbo);
        }

        for v in vertexes {
            v.apply()
        }

        buff
    }

    /// set_used uses the buffer. you still have to call draw afterwards
    pub fn set_used(&self) {
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
            gl::DeleteBuffers(1, self.ebo as *const u32);
            gl::DeleteBuffers(1, self.vbo as *const u32);
            gl::DeleteVertexArrays(1, self.vao as *const u32);
        }
    }
}