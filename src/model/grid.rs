use std::hash::Hash;

use rustc_hash::FxHashMap;

use super::{Globals, Particle};

pub struct Grid<E> {
    pub elems: Vec<E>,
    pub map: FxHashMap<E, Particle>,
}

impl<E: Eq + Hash + Copy> Grid<E> {
    pub fn new(_: &Globals) -> Self {
        Self {
            elems: Vec::new(),
            map: FxHashMap::default(),
        }
    }

    pub fn fill<It: Iterator<Item = (E, Particle)>>(&mut self, iter: It) {
        self.map = iter.collect();
        self.elems = self.map.iter().map(|(e, _)| *e).collect();
    }

    pub fn neighbors(&self, e: E) -> Vec<&Particle> {
        self.elems
            .iter()
            .filter(|e_| **e_ != e)
            .map(|e| self.map.get(e).unwrap())
            .collect()
    }
}
