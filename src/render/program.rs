extern crate gl;

use std;
use std::ffi::{CStr, CString};
use std::fs;

use crate::math::mat::Mat;
use crate::math::vect::Vect;
use crate::render::shader::Shader;
use crate::render::create_whitespace_cstring_with_len;
use crate::math::rgba::RGBA;

/// Program is opengl shader program wrapper
/// main feature is setting uniforms. There are some predefined functions
/// for setting them but you can also define new ones because there is lot of variants.
/// Already defined setters should guid you.
/// Note that even though you can clone Program you are just cloning pointer to program.
#[derive(Clone)]
pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    ///new creates shadre program from path of vertex and fragment shader files
    pub fn new(paths: &[&str]) -> Result<Program, String> {
        let mut shaders = Vec::with_capacity(paths.len());
        for path in paths {
            shaders.push(Shader::new(path)?);
        }
        Self::from_shaders(&*shaders)
    }

    ///from_shaders is same as new it just uses already loaded shaders
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

    /// default returns rustbatch default program
    pub fn default() -> Program {
        Self::from_shaders(&[
            Shader::default_vertex(),
            Shader::default_fragment()]
        ).unwrap()
    }

    /// id returns id of program. its simply pointer to shader object
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    /// set_used uses shader program
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// get_ptr returns pointer to uniform var of shader by name
    pub fn get_ptr(&self, name: &str) -> i32 {
        let cstr = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.id, cstr.as_ptr()) }
    }

    pub fn set_mat4(&self, address: &str, mat: Mat) {
        self.set_used();
        unsafe { gl::UniformMatrix4fv(self.get_ptr(address), 1, gl::FALSE, &mat.to_glm_mat4().c0.x); }
    }

    pub fn set_vec2(&self ,address: &str, vec: Vect) {
        self.set_used();
        unsafe { gl::Uniform2f(self.get_ptr(address), vec.x, vec.y); }
    }

    pub fn set_color(&self ,address: &str, col: &RGBA) {
        self.set_used();
        unsafe { gl::Uniform4f(self.get_ptr(address), col[0], col[1], col[2], col[3]); }
    }

    pub fn set_float(&self, address: &str, value: f32) {
        self.set_used();
        unsafe { gl::Uniform1f(self.get_ptr(address), value)}
    }

    pub(crate) fn set_camera(&self, mat: Mat) {
        self.set_mat4("camera", mat);
    }

    pub(crate) fn set_view_size(&self, vec: Vect) {
        self.set_vec2("view_size", vec/2f32);
    }

    pub(crate) fn set_texture_size(&self, vec: Vect) {
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

