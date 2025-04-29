use image::load_image;
use image::save_image;
use image::Image;
use pattern8::Pattern8;
use superposition::ImageSuperposition;
use superposition::Wfc;

pub mod color;
pub mod image;
pub mod pattern;
pub mod pattern8;
pub mod pixel;
pub mod superposition;
pub mod vec2;
pub mod weighted;

fn main() {
    println!("Hello, world!");

    let image = load_image("./test/flowers.png");

    let mut image_sp = ImageSuperposition::<8, Pattern8>::extract(image);
    while let Some(pixel_index) = image_sp.search() {
        image_sp.collapse(pixel_index);
        image_sp.propagate(pixel_index);
    }

    let image_out = Image::from(image_sp);
    save_image(image_out, "./test/out.png");
}
