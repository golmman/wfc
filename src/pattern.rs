use crate::{color::Color, image::Image, superposition::ImageSuperposition, vec2::Vec2};

pub trait Pattern<const N: usize>: Sized + Clone {
    fn get_colors(&self) -> &[Option<Color>; N];
    fn extract_pattern_at(image: &Image, pos: Vec2) -> Self;
    fn empty() -> Self;
}
