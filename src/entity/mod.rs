pub mod pathfinder;
pub mod scanner;
pub mod storage;

use std::hash::BuildHasherDefault;
use hashers::fnv::FNV1aHasher32;
pub type FastHash = BuildHasherDefault<FNV1aHasher32>;

pub fn gen_hash() -> FastHash {
    BuildHasherDefault::<FNV1aHasher32>::default()
}
