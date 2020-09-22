//! render is module with all opengl related functionality. Create window first then you can use all
//! functionality
pub mod shader;
pub mod buffer;
pub mod batch;
pub mod program;
pub mod texture;
pub mod sprite;
pub mod window;

use std::ffi::CString;
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

use sdl2::{video, VideoSubsystem};
use crate::Window;
use crate::math::{mat, rgba};

