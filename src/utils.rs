use super::consts::*;
use crate::polygon::*;
use crate::vector::Vector3;
use image::io::Reader as ImageReader;
use native_dialog::FileDialog;
use wavefront::Obj;

#[derive(Debug, Copy, Clone)]
pub struct MinCords {
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
}

pub struct CoeffData {
    pub kd: f32,
    pub ks: f32,
    pub m: f32,
    pub z: f32,
}

pub fn load_min_cords(object: &Obj) -> MinCords {
    let mut min_cords = MinCords {
        max_x: f32::MIN,
        min_x: f32::MAX,
        max_y: f32::MIN,
        min_y: f32::MAX,
        max_z: f32::MIN,
        min_z: f32::MAX,
    };

    for [a, b, c] in object.triangles() {
        min_cords.max_x = (min_cords.max_x)
            .max(a.position()[0])
            .max(b.position()[0])
            .max(c.position()[0]);

        min_cords.max_y = (min_cords.max_y)
            .max(a.position()[1])
            .max(b.position()[1])
            .max(c.position()[1]);
        min_cords.max_z = (min_cords.max_z)
            .max(a.position()[2])
            .max(b.position()[2])
            .max(c.position()[2]);
        min_cords.min_x = (min_cords.min_x)
            .min(a.position()[0])
            .min(b.position()[0])
            .min(c.position()[0]);
        min_cords.min_y = (min_cords.min_y)
            .min(a.position()[1])
            .min(b.position()[1])
            .min(c.position()[1]);
        min_cords.min_z = (min_cords.min_z)
            .min(a.position()[2])
            .min(b.position()[2])
            .min(c.position()[2]);
    }
    min_cords
}

pub fn map_point(min_cords: MinCords, cords: [f32; 3]) -> [i32; 3] {
    let x_range = min_cords.max_x - min_cords.min_x;
    let y_range = min_cords.max_y - min_cords.min_y;
    let z_range = min_cords.max_z - min_cords.min_z;

    let x = (cords[0] - min_cords.min_x) / x_range * (IMAGE_SIZE as f32);
    let y = (cords[1] - min_cords.min_y) / y_range * (IMAGE_SIZE as f32);
    let z = (cords[2] - min_cords.min_z) / z_range * (IMAGE_SIZE as f32 / 2.0);
    [x as i32, y as i32, z as i32]
}

pub fn load_polygons(file_path: &str) -> Vec<Polygon> {
    match Obj::from_file(&file_path) {
        Ok(o) => {
            let min_cords = load_min_cords(&o);
            o.polygons()
                .map(|pol| {
                    let new_vertices: Vec<Vertex> = pol
                        .vertices()
                        .map(|v| {
                            let positions = map_point(min_cords, v.position());
                            Vertex {
                                position: positions,
                                normal: v.normal().map(Vector3::from_array).unwrap_or_else(|| {
                                    Vector3::new(
                                        (positions[0] - IMAGE_SIZE as i32 / 2) as f32,
                                        (positions[1] - IMAGE_SIZE as i32 / 2) as f32,
                                        positions[2] as f32,
                                    )
                                }),
                                light: Vector3::default(),
                                color: Vector3::default(),
                            }
                        })
                        .collect();
                    Polygon {
                        vertices: new_vertices,
                    }
                })
                .collect::<Vec<Polygon>>()
        }
        Err(_) => vec![],
    }
}

pub fn load_obj() -> Vec<Polygon> {
    let file = FileDialog::new()
        .add_filter("obj", &["obj"])
        .show_open_single_file()
        .unwrap();
    match file {
        Some(buff) => {
            let file_path = buff.as_path().as_os_str().to_str().unwrap();
            load_polygons(file_path)
        }
        None => vec![],
    }
}

pub fn load_image(file_path: &str) -> image::Rgb32FImage {
    ImageReader::open(file_path)
        .unwrap()
        .decode()
        .map(|i| {
            i.resize_to_fill(
                IMAGE_SIZE + 1,
                IMAGE_SIZE + 1,
                image::imageops::FilterType::Nearest,
            )
            .to_rgb32f()
        })
        .unwrap_or_else(|_| image::Rgb32FImage::new(IMAGE_SIZE + 1, IMAGE_SIZE + 1))
}

pub fn load_texture() -> Option<image::Rgb32FImage> {
    let file = FileDialog::new()
        .add_filter("img", &["png", "jpg"])
        .show_open_single_file()
        .unwrap();
    file.map(|buff| {
        let file_path = buff.as_path().as_os_str().to_str().unwrap();
        load_image(file_path)
    })
}

pub fn get_prev(i: usize, size: usize) -> usize {
    if i > 0 {
        i - 1
    } else {
        size - 1
    }
}

pub fn get_next(i: usize, size: usize) -> usize {
    (i + 1) % size
}
