pub struct Globals {
    pub bounds_min: Vec<f64>,
    pub bounds_max: Vec<f64>,
    pub radius: f64,
}

impl Default for Globals {
    fn default() -> Self {
        Self {
            bounds_min: vec![-100.0, -100.0],
            bounds_max: vec![100.0, 100.0],
            radius: 50.0,
        }
    }
}
