use super::Particle;

pub struct Grid<E> {
    pub elems: Vec<(E, Particle)>,
}

impl<E> Grid<E> {
    pub fn new() -> Self {
        Self { elems: Vec::new() }
    }
}

impl<E, It: Iterator<Item = (E, Particle)>> From<It> for Grid<E> {
    fn from(iter: It) -> Self {
        Self {
            elems: iter.collect(),
        }
    }
}
