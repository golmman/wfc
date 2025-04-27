use crate::color::Color;

pub struct ImageSuperposition<const N: usize, T: Pattern<N>> {
  pub width: u32,
  pub height: u32,
  pub pixels: Vec<PixelSuperposition<N, T>>,
}

// = ColorsAndPatterns
pub struct PixelSuperposition<const N: usize, T: Pattern<N>> {
  pub possible_colors: Vec<ColorSuperposition<N, T>>,
}

// = ColorAndPatterns
pub struct ColorSuperposition<const N: usize, T: Pattern<N>> {
  pub color: Color,
  pub patterns: Vec<T>,
}

struct Pattern8 {
  colors: [Option<Color>; 8],
}

trait Pattern<const N: usize> {
  fn get_colors(&self) -> [Option<Color>; N];
}

impl Pattern<8> for Pattern8 {
    fn get_colors(&self) -> [Option<Color>; 8] {
        todo!()
    }
}

