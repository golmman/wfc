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

    fn extract(image: Image) -> ImageSuperposition<PATTERN_SIZE, Self> {
        let mut pixel_sp = PixelSuperposition { colors: Vec::new() };

        for y in 0..image.height as i32 {
            for x in 0..image.width as i32 {
                let color = image
                    .get_color_at(Vec2 { x, y })
                    .expect("image index not allowed");

                let color_index = get_color_index(color, &pixel_sp);
                let pattern = extract_pattern_at(&image, Vec2 { x, y });

                match color_index {
                    Some(color_index) => pixel_sp.colors[color_index].patterns.push(pattern),
                    None => pixel_sp.colors.push(ColorSuperposition {
                        color,
                        patterns: vec![pattern],
                    }),
                }
            }
        }

        ImageSuperposition {
            width: image.width,
            height: image.height,
            pixels: vec![pixel_sp; (image.width * image.height) as usize],
        }
    }

    fn search(image_sp: &ImageSuperposition<PATTERN_SIZE, Self>) -> usize {
        let mut lowest_index = usize::MAX;
        let mut lowest_entropy = f32::MAX;

        for i in 0..image_sp.pixels.len() {
            //let color_and_patterns = &colors_and_patterns[i];
        }

        lowest_index
    }
}

fn calc_entropy<const N: usize, T: Pattern<N>>(pixel_sp: PixelSuperposition<N, T>) -> f32 {
    let mut total_weight = 0;
    for i in 0..pixel_sp.colors.len() {
        let color = &pixel_sp.colors[i];
        total_weight += color.patterns.len();
    }

    let mut entropy = 0.0;
    for i in 0..pixel_sp.colors.len() {
        let color = &pixel_sp.colors[i];
        let color_weight = color.patterns.len();
        let color_probability = color_weight as f32 / total_weight as f32;

        entropy += color_probability * color_probability.ln();
    }

    -entropy
}

fn get_color_index<const N: usize, T: Pattern<N>>(
    color: Color,
    pixel_sp: &PixelSuperposition<N, T>,
) -> Option<usize> {
    for i in 0..pixel_sp.colors.len() {
        let c = pixel_sp.colors[i].color;
        if c == color {
            return Some(i);
        }
    }
    None
}

fn extract_pattern_at(image: &Image, pos: Vec2) -> Pattern8 {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_extracts_1_pattern_from_a_simple_image() {
        let image = Image {
            width: 2,
            height: 2,
            colors: vec![Color(0), Color(0), Color(0), Color(0)],
        };

        let image_sp = Pattern8::extract(image);

        assert_eq!(image_sp.pixels.len(), 4);
        assert_eq!(image_sp.pixels[0].colors.len(), 1);
        assert_eq!(image_sp.pixels[0].colors[0].color, Color(0));
        assert_eq!(image_sp.pixels[0].colors[0].patterns.len(), 4);
        assert_eq!(
            image_sp.pixels[0].colors[0].patterns[0]
                .colors
                .iter()
                .filter(|&opt| opt.is_none())
                .count(),
            5
        );
    }

    #[test]
    fn it_calculates_the_entropy_of_a_pixel_superposition() {
        let pattern = Pattern8 {
            colors: [None; PATTERN_SIZE],
        };
        let color_sp1 = ColorSuperposition {
            color: Color(0),
            patterns: vec![pattern.clone(); 2],
        };
        let color_sp2 = ColorSuperposition {
            color: Color(0),
            patterns: vec![pattern.clone(); 3],
        };
        let color_sp3 = ColorSuperposition {
            color: Color(0),
            patterns: vec![pattern.clone(); 5],
        };
        let pixel_sp = PixelSuperposition {
            colors: vec![color_sp1, color_sp2, color_sp3],
        };
        let total = (2 + 3 + 5) as f32;

        let entropy = calc_entropy(pixel_sp);

        assert_eq!(
            entropy,
            -(2.0 / total * (2.0 / total).ln()
                + 3.0 / total * (3.0 / total).ln()
                + 5.0 / total * (5.0 / total).ln())
        );
    }
}
