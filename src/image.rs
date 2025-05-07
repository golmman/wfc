use crate::color::Color;
use crate::pixel::Pixel;
use crate::vec2::Vec2;
use ::image::GenericImageView;
use ::image::open;
use ::image::{ImageBuffer, Rgba};
use image::DynamicImage;
use std::path::Path;

#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub colors: Vec<Color>,
}

impl Image {
    pub fn get_color_at(&self, pos: Vec2) -> Option<Color> {
        if pos.x < 0 || pos.x >= self.width as i32 {
            return None;
        }
        if pos.y < 0 || pos.y >= self.height as i32 {
            return None;
        }

        Some(self.colors[(pos.y * self.width as i32 + pos.x) as usize])
    }

    pub fn set_pixel(&mut self, pixel: Pixel) {
        self.colors[(pixel.pos.y * self.width as i32 + pixel.pos.x) as usize] = pixel.color;
    }
}

// TODO: make part of impl
pub fn load_image<T: AsRef<Path>>(path: T) -> Image {
    let img = open(path).unwrap().into_rgba8();
    let img = DynamicImage::ImageRgba8(img);

    let (width, height) = img.dimensions();
    let bytes = img.into_bytes();

    let mut colors = vec![Color(0); (width * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let i = (width * y + x) as usize;
            let rgba = (bytes[4 * i] as u32)
                | (bytes[4 * i + 1] as u32) << 8
                | (bytes[4 * i + 2] as u32) << 16
                | (bytes[4 * i + 3] as u32) << 24;

            colors[i] = Color(rgba);
        }
    }

    Image {
        width,
        height,
        colors,
    }
}

// TODO: make part of impl
pub fn save_image<T: AsRef<Path>>(image: Image, path: T) {
    let mut img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(image.width, image.height);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let index = (y * image.width + x) as usize;
        if index < image.colors.len() {
            let color = image.colors[index];
            *pixel = Rgba([color.r(), color.g(), color.b(), color.a()]);
        }
    }

    img_buffer.save(path).expect("Failed to save image");
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::vec2::Vec2;

    #[test]
    fn it_should_read_and_then_write_a_red_line_to_an_image() {
        let mut image = load_image("./test/flowers.png");

        let y = 5;
        for x in 3..10 {
            image.set_pixel(Pixel {
                pos: Vec2 { x, y },
                color: Color(0xFF0000FF),
            });
        }

        save_image(image, "./test/flowers_test.png");
    }
}
