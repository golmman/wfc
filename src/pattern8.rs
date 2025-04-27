use crate::{
    color::Color,
    image::Image,
    pattern::{ColorAndPatterns, ColorsAndPatterns, Pattern},
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

const DIRS: [Vec2; 8] = [
    Vec2 { x: -1, y: -1 },
    Vec2 { x: 0, y: -1 },
    Vec2 { x: 1, y: -1 },
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: 1 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: 1, y: 1 },
];

#[derive(Debug)]
pub struct Pattern8 {
    colors: [Option<Color>; 8],
}

impl Pattern for Pattern8 {
    fn extract(image: Image) -> Vec<ColorAndPatterns<Pattern8>> {
        let mut colors_and_patterns = Vec::new();

        for y in 0..image.height as i32 {
            for x in 0..image.width as i32 {
                let color = image
                    .get_color_at(Vec2 { x, y })
                    .expect("image index not allowed");

                let color_index = get_color_index(color, &colors_and_patterns);
                let pattern = extract_pattern_at(&image, Vec2 { x, y });

                match color_index {
                    Some(color_index) => colors_and_patterns[color_index].patterns.push(pattern),
                    None => colors_and_patterns.push(ColorAndPatterns {
                        color,
                        patterns: vec![pattern],
                    }),
                }
            }
        }

        colors_and_patterns
    }

    fn search(colors_and_patterns: &ColorsAndPatterns<Self>) -> usize {
        let mut lowest_index = usize::MAX;
        let mut lowest_entropy = f32::MAX;

        for i in 0..colors_and_patterns.len() {
            let color_and_patterns = &colors_and_patterns[i];

            
        }

        lowest_index
    }


}

fn calc_entropy<T>(color_and_patterns: &ColorAndPatterns<T>) -> f32{
    let mut entropy = 0.0;



    entropy
}

fn get_color_index<T>(
    color: Color,
    colors_and_patterns: &Vec<ColorAndPatterns<T>>,
) -> Option<usize> {
    for i in 0..colors_and_patterns.len() {
        let c = colors_and_patterns[i].color;
        if c == color {
            return Some(i);
        }
    }
    None
}

fn extract_pattern_at(image: &Image, pos: Vec2) -> Pattern8 {
    let mut pattern = Pattern8 {
        colors: [None, None, None, None, None, None, None, None],
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

        let colors_and_patterns = Pattern8::extract(image);

        assert_eq!(colors_and_patterns.len(), 1);
        assert_eq!(colors_and_patterns[0].color, Color(0));
        assert_eq!(colors_and_patterns[0].patterns.len(), 4);
        assert_eq!(
            colors_and_patterns[0].patterns[0]
                .colors
                .iter()
                .filter(|&opt| opt.is_none())
                .count(),
            5
        );

        println!("{:#?}", colors_and_patterns);
    }
}
