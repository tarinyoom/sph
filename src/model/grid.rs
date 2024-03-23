use std::hash::Hash;

use rustc_hash::FxHashMap;

use super::Particle;

pub struct Grid<E> {
    pub elems: Vec<E>,
    pub map: FxHashMap<E, Particle>,
}

impl<E: Eq + Hash + Copy> Grid<E> {
    pub fn new() -> Self {
        Self {
            elems: Vec::new(),
            map: FxHashMap::default(),
        }
    }

    pub fn neighbors(&self, e: E) -> Vec<&Particle> {
        self.elems
            .iter()
            .filter(|e_| **e_ != e)
            .map(|e| self.map.get(e).unwrap())
            .collect()
    }
}

impl<E: Eq + Hash + Copy, It: Iterator<Item = (E, Particle)>> From<It> for Grid<E> {
    fn from(iter: It) -> Self {
        let map: FxHashMap<E, Particle> = iter.collect();
        let elems = map.iter().map(|(e, _)| *e).collect();
        Self {
            elems: elems,
            map: map,
        }
    }
}
