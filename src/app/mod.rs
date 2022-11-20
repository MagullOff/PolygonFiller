use super::consts::*;
use crate::polygon::*;
use crate::utils::*;
use egui::*;

pub mod painter;
pub mod ui;

pub struct PolygonFiller {
    coeff_data: CoeffData,
    light_rgb: [f32; 3],
    object_rgb: [f32; 3],
    object_texture: image::Rgb32FImage,
    rotation: bool,
    interpolation: InterpolationType,
    color_source: ColorSourceType,
    normal_map_enabled: bool,
    normal_map: image::Rgb32FImage,
    sun_position_angle: f32,
    sun_position_radius: f32,
    object: Vec<Polygon>,
}

impl Default for PolygonFiller {
    fn default() -> Self {
        Self {
            coeff_data: CoeffData {
                kd: MAX_KD / 2.0,
                ks: MAX_KS / 2.0,
                m: MAX_M / 2.0,
                z: MAX_Z / 2.0,
            },
            light_rgb: [1.0, 1.0, 1.0],
            object_rgb: [1.0, 1.0, 1.0],
            object_texture: load_image("assets/texture.jpg"),
            rotation: false,
            interpolation: InterpolationType::Vector,
            color_source: ColorSourceType::Color,
            normal_map_enabled: false,
            normal_map: load_image("assets/normal_map.png"),
            sun_position_angle: 0.0,
            sun_position_radius: ORBIT_R,
            object: load_polygons("assets/sphere.obj"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum InterpolationType {
    Color,
    Vector,
}

#[derive(PartialEq, Eq)]
pub enum ColorSourceType {
    Color,
    Texture,
}
