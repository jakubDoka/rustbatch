extern crate gl;

use std::ffi::{CString, CStr};
use std::fs;
use std::path::Path;
use crate::render::create_whitespace_cstring_with_len;

/// Shader is wrapper for opengl shader
/// it holds only pointer of the shader so you can freely clone ti
pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    /// new loads and compiles shader from given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Shader, String> {
        let source = fs::read_to_string(path.as_ref()).map_err(|err| err.to_string())?;

        let source = CString::new(source).map_err(|err| err.to_string())?;

        Shader::from_source(
            &source,
            if match path.as_ref().to_str(){
                Some(str) => str.ends_with(".frag"),
                None => return Err(String::from("path of shader file contains invalid characters"))
            } { gl::FRAGMENT_SHADER } else { gl::VERTEX_SHADER },
        )
    }

    /// default_vertex returns default vertex shader
    pub fn default_vertex() -> Shader {
        let vert: CString = CString::new("
            #version 330 core\n\n

            layout (location = 0) in vec2 pos;\n
            layout (location = 1) in vec2 reg;\n
            layout (location = 2) in vec4 col;\n\n

            out vec2 region;\n
            out vec4 color;\n\n

            uniform mat4 camera;\n\
            uniform mat4 model;
            uniform vec2 texture_size;\n
            uniform vec2 view_size;\n\n

            void main(){\n
            gl_Position = camera * vec4(pos/view_size, 0.0, 1.0);\n
            color = col;\n
            region = reg/texture_size;\n
            }").unwrap();

        Self::from_source(&vert, gl::VERTEX_SHADER).unwrap()
    }

    /// default_fragment returns default fragment shader
    pub fn default_fragment() -> Shader {
        let frag: CString = CString::new("
            #version 330 core\n

            in vec4 color;\n
            in vec2 region;\n\n

            out vec4 result_color;\n\n

            uniform sampler2D sp;\n\n

            void main(){\n\
                result_color = texture(sp, region) * color;\n\
            }").unwrap();
        Self::from_source(&frag, gl::FRAGMENT_SHADER).unwrap()
    }

    pub fn no_texture_fragment() -> Shader {
        let frag: CString = CString::new("
            #version 330 core\n

            in vec4 color;\n

            out vec4 result_color;\n\n

            void main(){\n
            result_color = color;\n
            }").unwrap();
        Self::from_source(&frag, gl::FRAGMENT_SHADER).unwrap()
    }

    pub fn no_texture_vertex() -> Shader {
        let vert: CString = CString::new("
            #version 330 core\n\n

            layout (location = 0) in vec2 pos;\n
            layout (location = 1) in vec4 col;\n\n

            out vec4 color;\n\n

            uniform mat4 camera;\n\
            uniform vec2 view_size;\n\n

            void main(){\n
            gl_Position = camera * vec4(pos/view_size, 0.0, 1.0);\n
            color = col;\n
            }").unwrap();

        Self::from_source(&vert, gl::VERTEX_SHADER).unwrap()
    }

    /// from_source compiles shader from provided cstring
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    /// id returns id of shader, likewise program this is pointer to opengl object
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

