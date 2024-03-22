pub struct Globals {
    pub bounds_min: Vec<f64>,
    pub bounds_max: Vec<f64>,
    pub radius: f64,
}

impl Default for Globals {
    fn default() -> Self {
        Self {
            bounds_min: vec![-300.0, -200.0],
            bounds_max: vec![300.0, 200.0],
            radius: 75.0,
        }
    }
}
