use crate::{color::Color, image::Image, pattern::Pattern, stack_set::StackSet, vec2::Vec2};

pub const NW: usize = 0;
pub const N: usize = 1;
pub const NE: usize = 2;
pub const W: usize = 3;
pub const E: usize = 4;
pub const SW: usize = 5;
pub const S: usize = 6;
pub const SE: usize = 7;

const PATTERN_SIZE: usize = 8;

const DIRS: [Vec2; PATTERN_SIZE] = [
    Vec2 { x: -1, y: -1 },
    Vec2 { x: 0, y: -1 },
    Vec2 { x: 1, y: -1 },
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: 1 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: 1, y: 1 },
];

const REVERSE_INDICES: [usize; PATTERN_SIZE] = [7, 6, 5, 4, 3, 2, 1, 0];

const REVERSE_DIRS: [Vec2; PATTERN_SIZE] = [
    Vec2 { x: 1, y: 1 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: -1, y: 1 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 1, y: -1 },
    Vec2 { x: 0, y: -1 },
    Vec2 { x: -1, y: -1 },
];

#[derive(Clone, Debug)]
pub struct Pattern8 {
    colors: [Option<Color>; PATTERN_SIZE],
}

impl Pattern<PATTERN_SIZE> for Pattern8 {
    fn get_colors(&self) -> &[Option<Color>; PATTERN_SIZE] {
        &self.colors
    }


    fn get_color_at(&self, index: usize) -> Option<Color> {
        self.colors[index]
    }

    fn extract_pattern_at(image: &Image, pos: Vec2) -> Self {
        let mut pattern = Pattern8 {
            colors: [None; PATTERN_SIZE],
        };

        for i in 0..DIRS.len() {
            let dir = DIRS[i];
            let color = image.get_color_at(pos + dir);
            pattern.colors[i] = color;
        }

        pattern
    }

    fn empty() -> Self {
        Pattern8 {
            colors: [None; PATTERN_SIZE],
        }
    }

    fn add_neighbors(indices: &mut StackSet, index: usize, width: u32, height: u32) {
        let pos = Vec2::from_index(index, width);
        for i in 0..DIRS.len() {
            let dir = DIRS[i];
            let p = pos + dir;

            if p.is_inside(width, height) {
                indices.push(p.into_index(width));
            }
        }
    }

    fn get_neighbors_opt(index: usize, width: u32, height: u32) -> Vec<Option<usize>> {
        // TODO: merge with add_neighbors?
        // TODO: return [Option<usize>; 8] ?
        let mut neighbors = Vec::new();

        let pos = Vec2::from_index(index, width);
        for i in 0..DIRS.len() {
            let dir = DIRS[i];
            let p = pos + dir;

            if p.is_inside(width, height) {
                neighbors.push(Some(p.into_index(width)));
            } else {
                neighbors.push(None);
            }
        }

        neighbors
    }

    fn get_neighbors(index: usize, width: u32, height: u32) -> Vec<usize> {
        // TODO: merge with add_neighbors?
        let mut neighbors = Vec::new();

        let pos = Vec2::from_index(index, width);
        for i in 0..DIRS.len() {
            let dir = DIRS[i];
            let p = pos + dir;

            if p.is_inside(width, height) {
                neighbors.push(p.into_index(width));
            }
        }

        neighbors
    }

    fn get_neighbors_and_colors(
        &self,
        index: usize,
        width: u32,
        height: u32,
    ) -> Vec<(Option<usize>, Option<Color>)> {
        let mut neighbors_and_colors = Vec::new();

        let pos = Vec2::from_index(index, width);
        for i in 0..DIRS.len() {
            let dir = DIRS[i];
            let p = pos + dir;

            let color = self.colors[i];
            if p.is_inside(width, height) {
                let neighbor = p.into_index(width);
                neighbors_and_colors.push((Some(neighbor), color));
            } else {
                neighbors_and_colors.push((None, color));
            }
        }

        neighbors_and_colors
    }
}
