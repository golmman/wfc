use crate::{color::Color, image::Image};

pub type ColorsAndPatterns<T> = Vec<(Color, Vec<T>)>;

pub trait Pattern: Sized {
    fn extract(image: Image) -> ColorsAndPatterns<Self>;
}
