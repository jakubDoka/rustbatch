pub extern crate anymap;

use std::hash::BuildHasherDefault;
use hashers::fnv::FNV1aHasher32;

pub mod component_map;
pub mod id_generator;
pub mod scanner;

type FastHash = BuildHasherDefault<FNV1aHasher32>;

