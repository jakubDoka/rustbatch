//! render is module with all opengl related functionality. Create window first then you can use all
//! features freely.
pub mod shader;
pub mod buffer;
pub mod batch;
pub mod program;
pub mod texture;
pub mod sprite;
pub mod window;
pub mod particle;
pub mod canvas;

use std::ffi::CString;
use crate::math::rgba::RGBA;

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn clear(color: &RGBA) {
    unsafe {
        gl::ClearColor(color[0], color[1], color[2], color[3]);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

