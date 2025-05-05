//! Global `Arc`-based object interning infrastructure.
//!
//! Eventually this should probably be replaced with salsa-based interning.

mod symbol;
pub use self::symbol::{Symbol, symbols as sym};
