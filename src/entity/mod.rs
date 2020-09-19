extern crate anymap;

use std::hash::BuildHasherDefault;
use hashers::fnv::FNV1aHasher32;

pub use anymap;

mod component_map;
mod id_generator;
mod scanner;

type FastHash = BuildHasherDefault<FNV1aHasher32>;

