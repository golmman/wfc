use crate::image::Image;

pub trait Pattern: Sized {
    fn extract(image: Image) -> Vec<Self>;
}
