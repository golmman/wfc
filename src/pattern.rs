use crate::{color::Color, image::Image, stack_set::StackSet, vec2::Vec2};

pub trait Pattern<const N: usize>: Sized + Clone {
    fn add_neighbors(indices: &mut StackSet, index: usize, width: u32, height: u32);
    fn empty() -> Self;
    fn extract_pattern_at(image: &Image, pos: Vec2) -> Self;
    fn get_colors(&self) -> &[Option<Color>; N];
    fn get_neighbors(index: usize, width: u32, height: u32) -> Vec<usize>;
    fn get_neighbors_and_colors(&self, index: usize, width: u32, height: u32) -> Vec<(usize, Color)>;
}
