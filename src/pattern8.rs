use crate::{
    color::Color,
    image::Image,
    pattern::Pattern,
    superposition::{ColorSuperposition, ImageSuperposition, PixelSuperposition},
    vec2::Vec2,
};

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

#[derive(Clone, Debug)]
pub struct Pattern8 {
    colors: [Option<Color>; PATTERN_SIZE],
}

impl Pattern<PATTERN_SIZE> for Pattern8 {
    fn get_colors(&self) -> &[Option<Color>; PATTERN_SIZE] {
        &self.colors
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
}
