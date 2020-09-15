use std;
use std::ffi::{CStr, CString};
use std::fs;
use crate::mat::Mat;
use crate::vect::Vect;

extern crate gl;

pub struct Program {
    id: gl::types::GLuint,
}

pub fn detach() {
    unsafe { gl::UseProgram(0); }
}

impl Program {
    pub fn new(paths: &[&str]) -> Result<Program, String> {
        let mut shaders = Vec::with_capacity(paths.len());
        for path in paths {
            shaders.push(match Shader::new(path){
                Ok(sha) => sha,
                Err(err) => return Err(err),
            })
        }
        Self::from_shaders(&*shaders)
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        unsafe {
            for shader in shaders {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        unsafe {
            for shader in shaders {
                gl::DetachShader(program_id, shader.id());
            }
        }


        Ok(Program { id: program_id })
    }

    pub fn default() -> Program {
        let vert: CString = CString::new("
            #version 330 core\n\n

            layout (location = 0) in vec2 pos;\n
            layout (location = 1) in vec2 reg;\n
            layout (location = 2) in vec4 col;\n\n

            out vec2 region;\n
            out vec4 color;\n\n

            uniform mat4 transform;\n
            uniform mat4 camera;\n
            uniform vec2 texture_size;\n
            uniform vec2 view_size;\n\n


            void main(){\n
            gl_Position = transform * camera * vec4(pos/view_size, 0.0, 1.0);\n
            color = col;\n
            region = reg/texture_size;\n
            }").unwrap();

        let frag: CString = CString::new("
            #version 330 core\n

            in vec4 color;\n
            in vec2 region;\n\n

            out vec4 result_color;\n\n

            uniform sampler2D sp;\n\n

            void main(){\n
            result_color = texture(sp, region) * color;\n
            }").unwrap();
        Self::from_shaders(&[
            Shader::from_source(&vert, gl::VERTEX_SHADER).unwrap(),
            Shader::from_source(&frag, gl::FRAGMENT_SHADER).unwrap()]
        ).unwrap()
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_ptr(&self, name: &str) -> i32 {
        let cstr = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.id, cstr.as_ptr()) }
    }

    pub fn set_mat4(&self, address: &str, mat: Mat) {
        self.set_used();
        unsafe { gl::UniformMatrix4fv(self.get_ptr(address), 1, gl::FALSE, &mat.to_glm_mat4().c0.x); }
        detach()
    }

    pub fn set_vec2(&self ,address: &str, vec: Vect) {
        self.set_used();
        unsafe { gl::Uniform2f(self.get_ptr(address), vec.x, vec.y); }
        detach();
    }

    pub fn set_transform_matrix(&self, mat: Mat) {
        self.set_mat4("transform", mat);
    }

    pub fn set_camera(&self, mat: Mat) {
        self.set_mat4("camera", mat);
    }

    pub fn set_view_size(&self, vec: Vect) {
        self.set_vec2("view_size", vec);
    }

    pub fn set_texture_size(&self, vec: Vect) {
        self.set_vec2("texture_size", vec);
    }

}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn new(path: &str) -> Result<Shader, String> {
        let source = match fs::read_to_string(path){
            Ok(str) => str,
            Err(err) => return Err(err.to_string() + path)
        };

        let source = match CString::new(source) {
            Ok(str) => str,
            Err(err) => return Err(err.to_string())
        };

        Shader::from_source(
            &source,
            if path.ends_with(".frag") { gl::FRAGMENT_SHADER } else { gl::VERTEX_SHADER },
        )
    }

    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

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

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}