use crate::{color::Color, image::Image};



#[derive(Debug)]
pub struct ColorAndPatterns<T> {
    pub color: Color,
    pub patterns: Vec<T>,
}

pub type ColorsAndPatterns<T> = Vec<ColorAndPatterns<T>>;

pub trait Pattern: Sized {
    fn extract(image: Image) -> ColorsAndPatterns<Self>;
    fn search(colors_and_patterns: &ColorsAndPatterns<Self>) -> usize;
}
