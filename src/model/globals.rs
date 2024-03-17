pub struct Globals {
    pub bounds_min: Vec<f32>,
    pub bounds_max: Vec<f32>,
}

impl Default for Globals {
    fn default() -> Self {
        Self {
            bounds_min: vec![-100.0, -100.0],
            bounds_max: vec![100.0, 100.0],
        }
    }
}
