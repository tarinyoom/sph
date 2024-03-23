use super::Particle;

pub struct Grid<E> {
    pub elems: Vec<(E, Particle)>,
}

impl<E> Grid<E> {
    pub fn new() -> Self {
        Self { elems: Vec::new() }
    }
}
