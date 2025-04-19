use image::GenericImageView;
use image::{ImageBuffer, Rgba};
use std::path::Path;

fn main() {
    println!("Hello, world!");

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

#[derive(Clone, Copy)]
pub struct Color(u32);

impl Color {
    pub fn r(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub fn g(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    pub fn b(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    pub fn a(&self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }
}

pub struct Vec2 {
    x: i32,
    y: i32,
}

pub struct Pixel {
    pos: Vec2,
    color: Color,
}

pub struct Image {
    width: u32,
    height: u32,
    colors: Vec<Color>,
}

impl Image {
    pub fn get_color_at(&self, x: i32, y: i32) -> Color {
        self.colors[(y * self.width as i32 + x) as usize]
    }

    pub fn set_pixel(&mut self, pixel: Pixel) {
        self.colors[(pixel.pos.y * self.width as i32 + pixel.pos.x) as usize] = pixel.color;
    }
}

fn load_image<T: AsRef<Path>>(path: T) -> Image {
    let img = image::open(path).unwrap();
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
