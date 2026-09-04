use std::ops::{Index, IndexMut};

use crate::Tile;

#[derive(Clone)]
pub struct TileMap<T>([T; 64]);

impl<T> Index<Tile> for TileMap<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, t: Tile) -> &T {
        unsafe { self.0.get_unchecked(t.index()) }
    }
}

impl<T> IndexMut<Tile> for TileMap<T> {
    #[inline(always)]
    fn index_mut(&mut self, t: Tile) -> &mut T {
        unsafe { self.0.get_unchecked_mut(t.index()) }
    }
}

impl<T: Copy + Default> TileMap<T> {
    pub fn empty() -> Self {
        TileMap([T::default(); 64])
    }
}

impl<T> TileMap<T> {
    pub const fn new(values: [T; 64]) -> Self {
        Self(values)
    }
}
