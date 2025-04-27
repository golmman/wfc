use crate::{
    color::Color,
    image::Image,
    superposition::{ImageSuperposition, PixelSuperposition},
};

pub trait Pattern<const N: usize>: Sized {
    fn get_colors(&self) -> [Option<Color>; N];
    fn extract(image: Image) -> PixelSuperposition<N, Self>; // TODO: make this return ImageSuperposition
    fn search(image_sp: &ImageSuperposition<N, Self>) -> usize;
}
