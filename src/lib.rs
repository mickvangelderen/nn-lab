pub mod mnist;
pub mod nn;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
