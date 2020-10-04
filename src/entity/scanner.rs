use crate::math::rect::Rect;
use crate::math::vect::Vect;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use hashers::fnv::FNV1aHasher32;
use crate::entity::{FastHash};
use crate::math::clamp;
use std::hash::Hash;

pub struct Scanner<T: Hash + Eq + Copy + Clone> {
    pub map: Vec<Vec<HashSet<T, FastHash>>>,
    tile_size: Vect,
    w: usize,
    h: usize,
}

impl<T: Hash + Eq + Copy + Clone> Scanner<T> {
    pub fn new(w: usize, h: usize, tile_size: Vect) -> Self {
        Scanner{map: vec![vec![HashSet::with_hasher(BuildHasherDefault::<FNV1aHasher32>::default()); w]; h], tile_size, w, h}
    }

    #[inline]
    pub fn get_coord(&self, pos: Vect) -> (usize, usize) {
        (clamp((pos.x / self.tile_size.x) as usize, 0, self.w-1),
         clamp((pos.y/self.tile_size.y) as usize, 0, self.h-1))
    }

    #[inline]
    pub fn insert(&mut self, pos: Vect, id: T) {
        let (x, y) = self.get_coord(pos);
        self.map[y][x].insert(id);
    }

    #[inline]
    pub fn remove(&mut self, pos: Vect, id: T) -> bool {
        let (x, y) = self.get_coord(pos);
        self.map[y][x].remove(&id)

    }

    pub fn slow_remove(&mut self, id: T) -> bool {
        for row in self.map.iter_mut() {
            for tile in row.iter_mut() {
                if tile.remove(&id) {
                    return true;
                }
            }
        }

        false
    }


    #[inline]
    pub fn update(&mut self, old: Vect, new: Vect, id: T) -> bool {
        let old = self.get_coord(old);
        let new = self.get_coord(new);

        if old == new {
            return true;
        }

        if !self.map[old.1][old.0].remove(&id) {
            return false;
        }

        self.map[new.1][new.0].insert(id);
        true
    }

    #[inline]
    pub fn query(&self, rect: &Rect, collector: &mut Vec<T>) {
        let mut min = self.get_coord(rect.min);
        let mut max = self.get_coord(rect.max);
        min = (
            if min.0 == 0 {0} else {clamp(min.0-1, 0, self.w)},
            if min.1 == 0 {0} else {clamp(min.1-1, 0, self.h)}
        );
        max = (clamp(max.0+2, 0, self.w),clamp(max.1+2, 0, self.h));
        for y in min.1..max.1 {
            for x in min.0..max.0 {
                collector.extend(&self.map[y][x]);
            }
        }
    }

    #[inline]
    pub fn query_point(&self, pos: Vect, collector: &mut Vec<T>) {
        let pos = self.get_coord(pos);
        let min = (
            if pos.0 == 0 {0} else {clamp(pos.0-1, 0, self.w)},
            if pos.1 == 0 {0} else {clamp(pos.1-1, 0, self.h)}
        );
        let max = (clamp(pos.0+2, 0, self.w),clamp(pos.1+2, 0, self.h));

        for y in min.1..max.1 {
            for x in min.0..max.0 {
                collector.extend(&self.map[y][x]);
            }
        }
    }

    pub fn get_shape_count(&self) -> usize {
        let mut count = 0;
        for row in self.map.iter() {
            for tile in row.iter() {
                count += tile.len();
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vect::Vect;
    use rand::Rng;
    use crate::entity::scanner::Scanner;

    #[test]
    fn insert_test() {
        let mut map: Scanner<usize> = Scanner::new(10, 10, Vect::new(100f32, 100f32));
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            map.insert(Vect::new(rng.gen::<f32>()*1000f32, rng.gen::<f32>()*1000f32), i);
        }

        assert_eq!(100 as usize, map.get_shape_count());
    }

    #[test]
    fn remove_test() {
        let mut map: Scanner<usize> = Scanner::new(10, 10, Vect::new(100f32, 100f32));
        let mut rng = rand::thread_rng();
        let mut poss = Vec::new();
        for i in 0..100 {
            poss.push(Vect::new(rng.gen::<f32>()*1000f32, rng.gen::<f32>()*1000f32));
        }

        for i in 0..100 {
            map.insert(poss[i], i);
        }

        for i in 0..100 {
            map.remove(poss[i], i);
        }

        assert_eq!(0 as usize, map.get_shape_count());
    }
}
