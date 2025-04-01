pub mod core;
pub mod models;
pub mod operations;
pub mod rendering;
pub mod history;

// Re-export main structures for easy access
pub use core::SimpleNodeGraph;
// Remove other exports to avoid unused warnings 