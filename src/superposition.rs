use std::collections::HashSet;

use oorandom::Rand32;

use crate::{
    color::Color, image::Image, pattern::Pattern, pattern8::Pattern8, stack_set::StackSet,
    vec2::Vec2, weighted::Weighted,
};

#[derive(Clone, Debug)]
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
    fn extract(&mut self, image: Image);
    fn search(&self) -> Option<usize>;
    fn collapse(&mut self, pixel_index: usize) -> usize;
    fn propagate(&mut self, pixel_index: usize) -> bool;
}

impl<const N: usize, T: Pattern<N>> Weighted for PixelSuperposition<N, T> {
    fn get_weight_at(&self, index: usize) -> Option<usize> {
        self.colors.get(index).map(|x| x.weight)
    }
}

impl<const N: usize, T: Pattern<N>> From<&ImageSuperposition<N, T>> for Image {
    fn from(image_sp: &ImageSuperposition<N, T>) -> Self {
        let mut colors = Vec::new();

        let mut wrong_pixels = 0;
        for i in 0..image_sp.pixels.len() {
            // TODO: remove fallback
            if image_sp.pixels[i].colors.len() == 0 {
                colors.push(Color(0xff000000));
                wrong_pixels += 1;
            } else if image_sp.pixels[i].colors.len() > 1 {
                colors.push(Color(0xffff0000));
            } else {
                colors.push(image_sp.pixels[i].colors[0].color);
            }
        }

        println!("save image, dead pixels: {}", wrong_pixels);

        Image {
            width: image_sp.width,
            height: image_sp.height,
            colors,
        }
    }
}

impl Wfc for ImageSuperposition<8, Pattern8> {
    fn extract(&mut self, image: Image) {
        // TODO: pixels at the borders have lower entropy: reduce possibilities
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

        self.pixels = vec![pixel_sp; (self.width * self.height) as usize];

        //ImageSuperposition {
        //    width: image.width,
        //    height: image.height,
        //    pixels: vec![pixel_sp; (image.width * image.height) as usize],
        //    rng: Rand32::new(1987082611),
        //}
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

    fn collapse(&mut self, pixel_index: usize) -> usize {
        println!(
            "collapse at: {:?}",
            Vec2::from_index(pixel_index, self.width)
        );
        //println!("  `-> {:?}", self.pixels[pixel_index]);
        //let pixel_sp = &self.pixels[pixel_index];

        // TODO: improve or remove?
        for i in 0..self.pixels[pixel_index].colors.len() {
            self.pixels[pixel_index].colors[i].weight =
                self.pixels[pixel_index].colors[i].patterns.len();
        }

        let color_index = self.pixels[pixel_index]
            .get_random_index(&mut self.rng)
            .expect("collapse is only possible if a color was chosen");

        //let color = &pixel_sp.colors[i];
        self.pixels[pixel_index] = PixelSuperposition {
            colors: vec![self.pixels[pixel_index].colors[color_index].clone()],
        };

        color_index
    }

    fn propagate(&mut self, pixel_index: usize) -> bool {
        let mut indices = StackSet::new(self.pixels.len()); // TODO: performance, make struct member?
        Pattern8::add_neighbors(&mut indices, pixel_index, self.width, self.height); // TODO: is reference to Pattern8 necessary?

        while let Some(pixel_index) = indices.pop() {
            if !self.is_collapsed_at(pixel_index) {
                if self.collapse_partially(pixel_index) {
                    if self.pixels[pixel_index].colors.len() == 0 {
                        return false;
                    }
                    Pattern8::add_neighbors(&mut indices, pixel_index, self.width, self.height); // TODO: is reference to Pattern8 necessary?
                }
            }
        }

        true
    }
}

impl<const N: usize, T: Pattern<N>> ImageSuperposition<N, T> {
    // TODO: DRY
    pub fn propagate_all(&mut self) {
        let mut indices = StackSet::full(self.pixels.len());
        while let Some(pixel_index) = indices.pop() {
            if !self.is_collapsed_at(pixel_index) {
                if self.collapse_partially(pixel_index) {
                    Pattern8::add_neighbors(&mut indices, pixel_index, self.width, self.height); // TODO: is reference to Pattern8 necessary?
                }
            }
        }
    }

    pub fn new(width: u32, height: u32) -> Self {
        let millis: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        //let millis = 1746367627610;
        println!("seed: {}", millis);
        Self {
            width,
            height,
            pixels: Vec::new(),
            //rng: Rand32::new(1987082627),
            rng: Rand32::new(millis),
        }
    }

    fn is_collapsed_at(&self, pixel_index: usize) -> bool {
        let pixel_sp = &self.pixels[pixel_index];
        is_collapsed(pixel_sp)
    }

    fn get_collapsed_color_at(&self, pixel_index: usize) -> Option<Color> {
        let pixel_sp = &self.pixels[pixel_index];
        if is_collapsed(pixel_sp) {
            // TODO: this is bad
            if pixel_sp.colors.len() == 0 {
                Some(Color(0))
            } else {
                Some(pixel_sp.colors[0].color)
            }
        } else {
            None
        }
    }

    //fn get_colors_at(&self, pixel_index: usize) -> &Vec<Color> {
    //    &self.pixels[pixel_index].colors.
    //}

    fn collapse_partially(&mut self, pixel_index: usize) -> bool {
        // TODO: better collapse: also take into account non fully collapsed pixels

        let mut has_changed = false;

        // build new colors for the current pixel
        let mut new_colors = Vec::new();
        for k in 0..self.pixels[pixel_index].colors.len() {
            let color1 = self.pixels[pixel_index].colors[k].color; // TODO: rename, without naming collision...

            // check if current color is even possible by all surrounding patterns and skip if not
            let neighbors = Pattern8::get_neighbors_opt(pixel_index, self.width, self.height);
            let mut skip_color = false;
            assert!(neighbors.len() == 8);
            for r in 0..neighbors.len() {
                if let Some(neighbor_index) = neighbors[r] {
                    let mut color_set = HashSet::new();
                    for s in 0..self.pixels[neighbor_index].colors.len() {
                        for t in 0..self.pixels[neighbor_index].colors[s].patterns.len() {
                            let pattern = &self.pixels[neighbor_index].colors[s].patterns[t];
                            if let Some(color_reverse) = pattern.get_color_at(7 - r) {
                                // TODO: 7?
                                color_set.insert(color_reverse);
                            }
                        }
                    }

                    if !color_set.contains(&color1) {
                        skip_color = true;
                        break;
                    }
                }
            }
            if skip_color {
                has_changed = true;
                continue;
            }

            // build new patterns for current color
            let mut new_patterns = Vec::new();
            for j in 0..self.pixels[pixel_index].colors[k].patterns.len() {
                let pattern = &self.pixels[pixel_index].colors[k].patterns[j];
                let neighbors_and_colors =
                    pattern.get_neighbors_and_colors(pixel_index, self.width, self.height);

                // TODO: outsource
                let mut pattern_conforms = true;
                for i in 0..neighbors_and_colors.len() {
                    let (neighbor_index, pattern_color) = neighbors_and_colors[i];

                    if neighbor_index.is_none() && pattern_color.is_none() {
                        continue;
                    }

                    if neighbor_index.is_none() && pattern_color.is_some() {
                        pattern_conforms = false;
                        break;
                    }

                    if neighbor_index.is_some() && pattern_color.is_none() {
                        pattern_conforms = false;
                        break;
                    }

                    // TODO: replace by: get all possible colors there, get all possible pattern colors here ..
                    // for each pattern position (8-neighborhood)
                    //   get list of possible colors there (other pixel)
                    //   keep only those patterns, that conform
                    //if let Some(neighbor_color) =
                    //    self.get_collapsed_color_at(neighbor_index.unwrap())
                    //{
                    //    if Some(neighbor_color) != pattern_color {
                    //        pattern_conforms = false;
                    //        break;
                    //    }
                    //}

                    // note that both options are "Some" at this point
                    let mut is_any_match = false;
                    for ci in 0..self.pixels[neighbor_index.unwrap()].colors.len() {
                        let color = self.pixels[neighbor_index.unwrap()].colors[ci].color;
                        if color == pattern_color.unwrap() {
                            is_any_match = true;
                            break;
                        }
                    }
                    if !is_any_match {
                        pattern_conforms = false;
                        break;
                    }
                }

                if pattern_conforms {
                    new_patterns.push(pattern.clone());
                } else {
                    has_changed = true
                }
            }

            if new_patterns.len() > 0 {
                new_colors.push(ColorSuperposition {
                    color: self.pixels[pixel_index].colors[k].color,
                    patterns: new_patterns,
                    weight: self.pixels[pixel_index].colors[k].weight,
                });
            }
        }

        self.pixels[pixel_index].colors = new_colors;

        has_changed
    }
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
    let total_weight = calc_total_weight(&pixel_sp);

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

        let mut image_sp = ImageSuperposition::<8, Pattern8>::new(2, 2);

        image_sp.extract(image);

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
