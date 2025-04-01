//! Missing batteries for standard libraries.

mod macros;

pub mod assert;
pub mod panic_context;
pub use itertools;

pub fn hash_once<Hasher: std::hash::Hasher + Default>(thing: impl std::hash::Hash) -> u64 {
    std::hash::BuildHasher::hash_one(&std::hash::BuildHasherDefault::<Hasher>::default(), thing)
}

pub fn trim_indent(mut text: &str) -> String {
    if text.starts_with('\n') {
        text = &text[1..];
    }
    let indent = text
        .lines()
        .filter(|it| !it.trim().is_empty())
        .map(|it| it.len() - it.trim_start().len())
        .min()
        .unwrap_or(0);
    text.split_inclusive('\n')
        .map(|line| {
            if line.len() <= indent {
                line.trim_start_matches(' ')
            } else {
                &line[indent..]
            }
        })
        .collect()
}
