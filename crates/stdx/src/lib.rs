pub mod assert;
pub mod panic_context;
pub use itertools;

pub fn hash_once<Hasher: std::hash::Hasher + Default>(thing: impl std::hash::Hash) -> u64 {
    std::hash::BuildHasher::hash_one(&std::hash::BuildHasherDefault::<Hasher>::default(), thing)
}
