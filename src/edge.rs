use rand::prelude::*;
#[derive(Debug)]
pub struct Edge {
    pub max: f32,
    pub min: f32,
    pub inv: f32,
    pub id: i32,
}

impl PartialEq for Edge {
    fn eq(&self, rhs: &Edge) -> bool {
        self.id == rhs.id
    }
}

impl Edge {
    pub fn new(v1: usize, v2: usize, positions: &[[i32; 3]]) -> Edge {
        Edge {
            min: if positions[v1][1] > positions[v2][1] {
                positions[v2][0] as f32
            } else {
                positions[v1][0] as f32
            },
            max: positions[v1][0].max(positions[v2][0]) as f32,
            id: {
                let mut rng = rand::thread_rng();
                rng.gen()
            },
            inv: if positions[v1][1] == positions[v2][1] {
                0.0
            } else {
                ((positions[v1][0] as f32) - (positions[v2][0] as f32))
                    / (positions[v1][1] - positions[v2][1]) as f32
            },
        }
    }
}
