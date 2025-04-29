use oorandom::Rand32;

use crate::{color::Color, image::Image, pattern::Pattern, pattern8::Pattern8, vec2::Vec2};

#[derive(Debug)]
pub struct ImageSuperposition<const N: usize, T: Pattern<N>> {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<PixelSuperposition<N, T>>,
    pub rng: Rand32,
}

#[derive(Clone, Debug)]
pub struct PixelSuperposition<const N: usize, T: Pattern<N>> {
    pub colors: Vec<ColorSuperposition<N, T>>,
}

#[derive(Clone, Debug)]
pub struct ColorSuperposition<const N: usize, T: Pattern<N>> {
    pub color: Color,
    pub patterns: Vec<T>,
    pub weight: usize,
}

pub trait Wfc {
    fn extract(image: Image) -> Self;
    fn search(&self) -> Option<usize>;
    fn collapse(&mut self, pixel_index: usize);
    fn propagate(&mut self, pixel_index: usize);
}

impl<const N: usize, T: Pattern<N>> From<ImageSuperposition<N, T>> for Image {
    fn from(image_sp: ImageSuperposition<N, T>) -> Self {
        let mut colors = Vec::new();

        for i in 0..image_sp.pixels.len() {
            let color = image_sp.pixels[i].colors[0].color;
            colors.push(color);
        }

        Image {
            width: image_sp.width,
            height: image_sp.height,
            colors,
        }
    }
}

impl Wfc for ImageSuperposition<8, Pattern8> {
    fn extract(image: Image) -> Self {
        let mut pixel_sp = PixelSuperposition { colors: Vec::new() };

        for y in 0..image.height as i32 {
            for x in 0..image.width as i32 {
                let color = image
                    .get_color_at(Vec2 { x, y })
                    .expect("image index not allowed");

                let color_index = get_color_index(color, &pixel_sp);
                let pattern = Pattern8::extract_pattern_at(&image, Vec2 { x, y });

                match color_index {
                    Some(color_index) => {
                        pixel_sp.colors[color_index].patterns.push(pattern);
                        pixel_sp.colors[color_index].weight =
                            pixel_sp.colors[color_index].patterns.len();
                    }
                    None => pixel_sp.colors.push(ColorSuperposition {
                        color,
                        patterns: vec![pattern],
                        weight: 1,
                    }),
                }
            }
        }

        ImageSuperposition {
            width: image.width,
            height: image.height,
            pixels: vec![pixel_sp; (image.width * image.height) as usize],
            rng: Rand32::new(19870826),
        }
    }

    fn search(&self) -> Option<usize> {
        let mut min_index = None;
        let mut min_entropy = f32::MAX;

        for i in 0..self.pixels.len() {
            let pixel_sp = &self.pixels[i];
            if is_collapsed(&pixel_sp) {
                continue;
            }

            let entropy = calc_entropy(pixel_sp);

            if entropy < min_entropy {
                min_entropy = entropy;
                min_index = Some(i);
            }
        }

        min_index
    }

    fn collapse(&mut self, pixel_index: usize) {
        let pixel_sp = &self.pixels[pixel_index];
        let mut max_index: Option<usize> = None;
        let mut max_weight = 0;

        for i in 0..pixel_sp.colors.len() {
            let color = &pixel_sp.colors[i];
            if color.weight > max_weight {
                max_weight = color.weight;
                max_index = Some(i);
            }
        }

        let i = max_index.expect("collapse is only possible if a color was chosen");
        let color = &pixel_sp.colors[i];
        self.pixels[pixel_index] = PixelSuperposition {
            colors: vec![color.clone()],
        };
    }

    fn propagate(&mut self, pixel_index: usize) {}
}

// TODO: impl PixelSuperposition
fn is_collapsed<const N: usize, T: Pattern<N>>(pixel_sp: &PixelSuperposition<N, T>) -> bool {
    pixel_sp.colors.len() <= 1
}

// TODO: impl PixelSuperposition
fn calc_weights<const N: usize, T: Pattern<N>>(pixel_sp: &PixelSuperposition<N, T>) -> usize {
    let mut total_weight = 0;
    for i in 0..pixel_sp.colors.len() {
        let color = &pixel_sp.colors[i];
        total_weight += color.patterns.len();
    }
    total_weight
}

// TODO: impl PixelSuperposition
fn calc_total_weight<const N: usize, T: Pattern<N>>(pixel_sp: &PixelSuperposition<N, T>) -> usize {
    let mut total_weight = 0;
    for i in 0..pixel_sp.colors.len() {
        let color = &pixel_sp.colors[i];
        total_weight += color.patterns.len();
    }
    total_weight
}

// TODO: impl PixelSuperposition
fn calc_entropy<const N: usize, T: Pattern<N>>(pixel_sp: &PixelSuperposition<N, T>) -> f32 {
    let mut total_weight = calc_total_weight(&pixel_sp);

    let mut entropy = 0.0;
    for i in 0..pixel_sp.colors.len() {
        let color = &pixel_sp.colors[i];
        let color_weight = color.patterns.len();
        let color_probability = color_weight as f32 / total_weight as f32;

        entropy += color_probability * color_probability.ln();
    }

    -entropy
}

// TODO: impl PixelSuperposition
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

        let image_sp = ImageSuperposition::extract(image);

        assert_eq!(image_sp.pixels.len(), 4);
        assert_eq!(image_sp.pixels[0].colors.len(), 1);
        assert_eq!(image_sp.pixels[0].colors[0].color, Color(0));
        assert_eq!(image_sp.pixels[0].colors[0].patterns.len(), 4);
        assert_eq!(
            image_sp.pixels[0].colors[0].patterns[0]
                .get_colors()
                .iter()
                .filter(|&opt| opt.is_none())
                .count(),
            5
        );
    }

    #[test]
    fn it_calculates_the_entropy_of_a_pixel_superposition() {
        let pattern = Pattern8::empty();
        let color_sp1 = ColorSuperposition {
            color: Color(0),
            patterns: vec![pattern.clone(); 2],
            weight: 0,
        };
        let color_sp2 = ColorSuperposition {
            color: Color(0),
            patterns: vec![pattern.clone(); 3],
            weight: 0,
        };
        let color_sp3 = ColorSuperposition {
            color: Color(0),
            patterns: vec![pattern.clone(); 5],
            weight: 0,
        };
        let pixel_sp = PixelSuperposition {
            colors: vec![color_sp1, color_sp2, color_sp3],
        };
        let total = (2 + 3 + 5) as f32;

        let entropy = calc_entropy(&pixel_sp);

        assert_eq!(
            entropy,
            -(2.0 / total * (2.0 / total).ln()
                + 3.0 / total * (3.0 / total).ln()
                + 5.0 / total * (5.0 / total).ln())
        );
    }
}
