#![warn(clippy::all, rust_2018_idioms)]

pub mod consts {
    pub const IMAGE_SIZE: u32 = 700;
    pub const MAX_Z: f32 = IMAGE_SIZE as f32;
    pub const MAX_KD: f32 = 1.0;
    pub const MAX_KS: f32 = 1.0;
    pub const MAX_M: f32 = 100.0;
    pub const ORBIT_R: f32 = 500.0;
}

mod app;
pub mod edge;
pub mod polygon;
pub mod utils;
pub mod vector;
pub use app::PolygonFiller;
