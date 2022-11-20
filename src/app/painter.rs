use super::{ColorSourceType, InterpolationType, PolygonFiller};
use crate::consts::*;
use crate::edge::Edge;
use crate::polygon::*;
use crate::utils::*;
use crate::vector::Vector3;
use egui::*;
use std::collections::HashMap;

impl PolygonFiller {
    pub fn get_sun_position(&self) -> Pos2 {
        pos2(
            IMAGE_SIZE as f32 / 2.0 + self.sun_position_radius * self.sun_position_angle.sin(),
            IMAGE_SIZE as f32 / 2.0 - self.sun_position_radius * self.sun_position_angle.cos(),
        )
    }

    fn get_light(&self, positions: &[i32; 3]) -> Vector3 {
        let sun = self.get_sun_position();
        Vector3::new(
            sun.x - positions[0] as f32,
            sun.y - positions[1] as f32,
            self.coeff_data.z - positions[2] as f32,
        )
    }

    fn get_color(
        &self,
        n_vec: Vector3,
        l_vec: Vector3,
        v_vec: Vector3,
        r_vec: Vector3,
        cords: (u32, u32),
    ) -> Vector3 {
        let color = match self.color_source {
            ColorSourceType::Color => self.object_rgb,
            ColorSourceType::Texture => self.object_texture.get_pixel(cords.0, cords.1).0,
        };

        let r =
            self.coeff_data.kd * self.light_rgb[0] * color[0] * Vector3::cos(n_vec, l_vec).max(0.0)
                + self.coeff_data.ks
                    * self.light_rgb[0]
                    * color[0]
                    * Vector3::cos(v_vec, r_vec).max(0.0).powf(self.coeff_data.m);
        let g =
            self.coeff_data.kd * self.light_rgb[1] * color[1] * Vector3::cos(n_vec, l_vec).max(0.0)
                + self.coeff_data.ks
                    * self.light_rgb[1]
                    * color[1]
                    * Vector3::cos(v_vec, r_vec).max(0.0).powf(self.coeff_data.m);
        let b =
            self.coeff_data.kd * self.light_rgb[2] * color[2] * Vector3::cos(n_vec, l_vec).max(0.0)
                + self.coeff_data.ks
                    * self.light_rgb[2]
                    * color[2]
                    * Vector3::cos(v_vec, r_vec).max(0.0).powf(self.coeff_data.m);
        Vector3::new(r / 2.0, g / 2.0, b / 2.0)
    }

    fn get_normal_from_texture(&self, normal: Vector3, positions: &[i32; 3]) -> Vector3 {
        let rgb = self
            .normal_map
            .get_pixel(positions[0] as u32, positions[1] as u32);
        let n_tex = Vector3::new((rgb[0] - 0.5) * 2.0, (rgb[1] - 0.5) * 2.0, rgb[2]);
        let b_vec = if normal == Vector3::new(0.0, 0.0, 1.0) {
            normal.cross(Vector3::new(0.0, 0.0, 1.0))
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        };
        let t_vec = b_vec.cross(normal);
        let matrix = (
            Vector3::new(t_vec.x, b_vec.x, normal.x),
            Vector3::new(t_vec.y, b_vec.y, normal.y),
            Vector3::new(t_vec.z, b_vec.z, normal.z),
        );
        Vector3::new(matrix.0 * n_tex, matrix.1 * n_tex, matrix.2 * n_tex)
    }

    fn get_vertice_color(&self, positions: &[i32; 3], normal: Vector3) -> Vector3 {
        let v_vec = Vector3::new(0.0, 0.0, 1.0);
        let l_vec = self.get_light(positions);
        let n_vec = match self.normal_map_enabled {
            true => self
                .get_normal_from_texture(normal.norm(), positions)
                .norm(),
            false => normal.norm(),
        };

        let r_vec = n_vec.multiply(n_vec * l_vec * 2.0) - l_vec;
        self.get_color(
            n_vec,
            l_vec,
            v_vec,
            r_vec,
            (positions[0] as u32, positions[1] as u32),
        )
    }

    fn paint_line(&self, aet: &Vec<Edge>, polygon: &Polygon, y: i32, map: &mut ColorImage) {
        let mut i = 0;
        let (x1, y1) = (
            polygon.vertices[0].position[0],
            polygon.vertices[0].position[1],
        );
        let (x2, y2) = (
            polygon.vertices[1].position[0],
            polygon.vertices[1].position[1],
        );
        let (x3, y3) = (
            polygon.vertices[2].position[0],
            polygon.vertices[2].position[1],
        );
        while i <= (aet.len() as i8) - 2 {
            for x in (aet[i as usize].min as i32)..(aet[(i + 1) as usize].min as i32) {
                //interpolation
                let w1 = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3)) as f32
                    / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)) as f32;
                let w2 = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3)) as f32
                    / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)) as f32;
                let w3 = 1.0 - w1 - w2;

                let (r, g, b) = match self.interpolation {
                    InterpolationType::Color => (
                        ((polygon.vertices[0].color.x * w1
                            + polygon.vertices[1].color.x * w2
                            + polygon.vertices[2].color.x * w3)
                            * 255.0) as u8,
                        ((polygon.vertices[0].color.y * w1
                            + polygon.vertices[1].color.y * w2
                            + polygon.vertices[2].color.y * w3)
                            * 255.0) as u8,
                        ((polygon.vertices[0].color.z * w1
                            + polygon.vertices[1].color.z * w2
                            + polygon.vertices[2].color.z * w3)
                            * 255.0) as u8,
                    ),
                    InterpolationType::Vector => {
                        let v_vec = Vector3::new(0.0, 0.0, 1.0);
                        let normals = (
                            polygon.vertices[0].normal.norm(),
                            polygon.vertices[1].normal.norm(),
                            polygon.vertices[2].normal.norm(),
                        );
                        let true_normal = Vector3::new(
                            normals.0.x * w1 + normals.1.x * w2 + normals.2.x * w3,
                            normals.0.y * w1 + normals.1.y * w2 + normals.2.y * w3,
                            normals.0.z * w1 + normals.1.z * w2 + normals.2.z * w3,
                        );
                        let z = polygon.vertices[0].position[2] as f32 * w1
                            + polygon.vertices[1].position[2] as f32 * w2
                            + polygon.vertices[2].position[2] as f32 * w3;

                        let n_vec = match self.normal_map_enabled {
                            false => true_normal,
                            true => self.get_normal_from_texture(true_normal, &[x, y, z as i32]),
                        }
                        .norm();

                        let l_vec = self.get_light(&[x, y, z as i32]);
                        let r_vec = n_vec.multiply(n_vec * l_vec * 2.0) - l_vec;
                        let rgb = self.get_color(n_vec, l_vec, v_vec, r_vec, (x as u32, y as u32));
                        (
                            (rgb.x * 255.0) as u8,
                            (rgb.y * 255.0) as u8,
                            (rgb.z * 255.0) as u8,
                        )
                    }
                };

                map[(x as usize, y as usize)] = Color32::from_rgb(r, g, b);
            }
            i += 2;
        }
    }

    fn fill_polygon(&self, polygon: &Polygon, map: &mut ColorImage) {
        let mut aet: Vec<Edge> = vec![];
        let mut edge_collection: HashMap<(usize, usize), i32> = HashMap::new();

        let ind = polygon.get_sorted_indeces();
        let positions = polygon
            .vertices
            .iter()
            .map(|v| v.position)
            .collect::<Vec<[i32; 3]>>();
        let ymin = positions[*ind.first().unwrap()][1];
        let ymax = positions[*ind.last().unwrap()][1];
        let mut k = 0;
        for y in ymin..=ymax {
            let mut points_prev_scanline: Vec<usize> = vec![];
            while positions[ind[k]][1] == y - 1 {
                points_prev_scanline.push(ind[k]);
                k += 1;
            }
            for v in points_prev_scanline {
                let prev = get_prev(v, ind.len());
                if positions[prev][1] > positions[v][1] {
                    let new_edge = Edge::new(prev, v, &positions);
                    edge_collection.insert((prev, v), new_edge.id);
                    aet.push(new_edge);
                }

                if positions[prev][1] < positions[v][1] {
                    let remove_index = if edge_collection.contains_key(&(prev, v)) {
                        edge_collection.get(&(prev, v)).unwrap()
                    } else {
                        edge_collection.get(&(v, prev)).unwrap()
                    };

                    aet.retain(|e| e.id != *remove_index);
                }
                let next = get_next(v, ind.len());

                if positions[next][1] > positions[v][1] {
                    let new_edge = Edge::new(next, v, &positions);
                    edge_collection.insert((next, v), new_edge.id);
                    aet.push(new_edge);
                }

                if positions[next][1] < positions[v][1] {
                    let remove_index = if edge_collection.contains_key(&(next, v)) {
                        edge_collection.get(&(next, v)).unwrap()
                    } else {
                        edge_collection.get(&(v, next)).unwrap()
                    };

                    aet.retain(|e| e.id != *remove_index);
                }
            }
            aet.sort_by(|a, b| a.min.partial_cmp(&b.min).unwrap());
            self.paint_line(&aet, polygon, y, map);
            for i in 0..aet.len() {
                aet[i].min += aet[i].inv;
            }
        }
    }

    pub fn paint(&mut self) -> egui::ColorImage {
        let mut map = ColorImage::new(
            [(IMAGE_SIZE + 1) as usize, (IMAGE_SIZE + 1) as usize],
            Color32::TRANSPARENT,
        );
        for j in 0..self.object.len() {
            for i in 0..self.object[j].vertices.len() {
                self.object[j].vertices[i].light =
                    self.get_light(&self.object[j].vertices[i].position);
                self.object[j].vertices[i].color = self.get_vertice_color(
                    &self.object[j].vertices[i].position,
                    self.object[j].vertices[i].normal,
                );
            }
            self.fill_polygon(&self.object[j], &mut map);
        }
        map
    }
}
