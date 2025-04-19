use crate::{color::Color, image::Image, pattern::Pattern};

pub const NW: usize = 0;
pub const N: usize = 1;
pub const NE: usize = 2;
pub const W: usize = 3;
pub const E: usize = 4;
pub const SW: usize = 5;
pub const S: usize = 6;
pub const SE: usize = 7;

pub struct Pattern8 {
    colors: [Color; 8],
}

impl Pattern for Pattern8 {
    fn extract(image: Image) -> Vec<Pattern8> {
        let patterns = Vec::new();

        for y in 0..image.height {
            for x in 0..image.width {
                
            }
        }

        patterns
    }
}
