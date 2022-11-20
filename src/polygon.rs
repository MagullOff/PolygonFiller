use crate::vector::Vector3;

pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

impl Polygon {
    pub fn get_sorted_indeces(&self) -> Vec<usize> {
        let mut sort_vec = self
            .vertices
            .iter()
            .map(|v| v.position)
            .enumerate()
            .collect::<Vec<(usize, [i32; 3])>>();
        sort_vec.sort_by(|(_, [x1, y1, _]), (_, [x2, y2, _])| {
            if y1 < y2 || (y1 == y2 && x1 < x2) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        sort_vec.iter().map(|elem| elem.0).collect()
    }
}

pub struct Vertex {
    pub position: [i32; 3],
    pub normal: Vector3,
    pub color: Vector3,
    pub light: Vector3,
}
