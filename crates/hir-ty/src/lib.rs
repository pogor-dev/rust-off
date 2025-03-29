//! The type system. We currently use this to infer types for completion, hover
//! information and various assists.

mod infer;

#[cfg(test)]
mod test_db;
#[cfg(test)]
mod tests;

pub use infer::InferenceResult;
