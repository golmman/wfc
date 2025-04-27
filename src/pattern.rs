use crate::{color::Color, image::Image, superposition::ImageSuperposition};

pub trait Pattern<const N: usize>: Sized + Clone {
    fn get_colors(&self) -> [Option<Color>; N];
    fn extract(image: Image) -> ImageSuperposition<N, Self>;
    fn search(image_sp: &ImageSuperposition<N, Self>) -> usize;
}
