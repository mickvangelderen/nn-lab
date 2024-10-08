pub mod mnist;
pub mod nn;
pub mod data;
pub mod math;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
