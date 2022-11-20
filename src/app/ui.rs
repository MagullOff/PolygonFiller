use super::{ColorSourceType, InterpolationType, PolygonFiller};
use crate::app::CollapsingHeader;
use crate::consts::*;
use crate::utils::*;
use egui::*;

impl eframe::App for PolygonFiller {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.rotation {
                use chrono::Timelike;
                let time = chrono::Local::now().time();
                let sec_since_midnight =
                    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64);
                self.sun_position_angle =
                    (sec_since_midnight * 2.5).rem_euclid(2.0 * std::f64::consts::PI) as f32;
                let r = (sec_since_midnight * 20.0).rem_euclid(ORBIT_R as f64 * 2f64);
                self.sun_position_radius = if (r as f32) < ORBIT_R {
                    r as f32
                } else {
                    2f32 * ORBIT_R - r as f32
                };
                ui.ctx().request_repaint();
            }

            let bitmap = self.paint();

            let window_size = ui.available_size();
            let mut img_ui = ui.child_ui(
                Rect {
                    min: pos2(
                        window_size.x / 2.0 - IMAGE_SIZE as f32 / 2.0,
                        window_size.y / 2.0 - IMAGE_SIZE as f32 / 2.0,
                    ),
                    max: pos2(
                        window_size.x / 2.0 + IMAGE_SIZE as f32 / 2.0,
                        window_size.y / 2.0 + IMAGE_SIZE as f32 / 2.0,
                    ),
                },
                egui::Layout::left_to_right(egui::Align::RIGHT),
            );

            let texture = &img_ui
                .ctx()
                .load_texture("sphere", bitmap, egui::TextureFilter::Linear);
            img_ui.image(texture, texture.size_vec2());

            Frame::popup(ui.style())
                .stroke(Stroke::none())
                .show(ui, |ui| {
                    ui.set_max_width(270.0);
                    CollapsingHeader::new("Settings").show(ui, |ui| self.options_ui(ui));
                });
        });
    }
}

impl PolygonFiller {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            coeff_data,
            light_rgb,
            object_rgb,
            object_texture,
            rotation,
            interpolation,
            color_source,
            normal_map,
            normal_map_enabled,
            object,
            ..
        } = self;

        let CoeffData { kd, ks, m, z } = coeff_data;

        ui.add(egui::Checkbox::new(&mut *rotation, "Enable animation"));

        if ui.add(egui::Button::new("Load new model")).clicked() {
            let new_obj = load_obj();
            if !new_obj.is_empty() {
                *object = new_obj
            }
        }

        ui.separator();
        ui.label("Coefficients");
        ui.add(egui::Slider::new(m, 1f32..=MAX_M).text("m"));
        ui.add(egui::Slider::new(z, (MAX_Z / 2.0)..=MAX_Z).text("z"));
        ui.add(egui::Slider::new(kd, 0.001..=MAX_KD).text("kd"));
        ui.add(egui::Slider::new(ks, 0.001..=MAX_KS).text("ks"));

        ui.separator();
        ui.label("Interpolation");
        ui.radio_value(&mut *interpolation, InterpolationType::Color, "color");
        ui.radio_value(&mut *interpolation, InterpolationType::Vector, "vector");

        ui.separator();
        ui.label("Colors and textures");

        ui.horizontal(|ui| {
            ui.color_edit_button_rgb(&mut *light_rgb);
            ui.label(": choose light color");
        });

        ui.horizontal(|ui| {
            ui.radio_value(&mut *color_source, ColorSourceType::Texture, "Texture");
            if ui.add(egui::Button::new("Load new texture")).clicked() {
                let new_image = load_texture();
                if let Some(i) = new_image {
                    *object_texture = i;
                }
            }
        });

        ui.horizontal(|ui| {
            ui.radio_value(&mut *color_source, ColorSourceType::Color, "Color");
            ui.color_edit_button_rgb(&mut *object_rgb);
        });

        ui.horizontal(|ui| {
            ui.add(egui::Checkbox::new(
                &mut *normal_map_enabled,
                "Enable custom normal map",
            ));
            if ui.add(egui::Button::new("Load normal map")).clicked() {
                let new_normal_map = load_texture();
                if let Some(i) = new_normal_map {
                    *normal_map = i;
                }
            }
        });
    }
}
