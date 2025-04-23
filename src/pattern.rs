use crate::{color::Color, image::Image};

#[derive(Debug)]
pub struct ColorAndPatterns<T> {
    pub color: Color,
    pub patterns: Vec<T>,
}

pub trait Pattern: Sized {
    fn extract(image: Image) -> Vec<ColorAndPatterns<Self>>;
}
