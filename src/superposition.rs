use crate::{color::Color, image::Image, pattern::Pattern};

#[derive(Debug)]
pub struct ImageSuperposition<const N: usize, T: Pattern<N>> {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<PixelSuperposition<N, T>>,
}

// = ColorsAndPatterns
#[derive(Clone, Debug)]
pub struct PixelSuperposition<const N: usize, T: Pattern<N>> {
    pub possible_colors: Vec<ColorSuperposition<N, T>>,
}

// = ColorAndPatterns
#[derive(Clone, Debug)]
pub struct ColorSuperposition<const N: usize, T: Pattern<N>> {
    pub color: Color,
    pub patterns: Vec<T>,
}
