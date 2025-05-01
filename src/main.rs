use image::load_image;
use image::save_image;
use image::Image;
use pattern8::Pattern8;
use superposition::ColorSuperposition;
use superposition::ImageSuperposition;
use superposition::PixelSuperposition;
use superposition::Wfc;

pub mod color;
pub mod image;
pub mod pattern;
pub mod pattern8;
pub mod pixel;
pub mod superposition;
pub mod stack_set;
pub mod vec2;
pub mod weighted;

fn main() {
    println!("Hello, world!");

    let image = load_image("./test/flowers.png");
    let image2 = image.clone();

    let mut image_sp = ImageSuperposition::<8, Pattern8>::extract(image);

    // TODO: remove
    //collapse_corners(image2, &mut image_sp);

    while let Some(pixel_index) = image_sp.search() {
        image_sp.collapse(pixel_index);
        image_sp.propagate(pixel_index);
    }

    let image_out = Image::from(image_sp);
    save_image(image_out, "./test/out.png");
}

//fn collapse_corners(image: Image, image_sp: &mut ImageSuperposition<8, Pattern8>) {
//    image_sp.pixels[0] = PixelSuperposition {
//        colors: vec![ColorSuperposition {
//            color: image.colors[0],
//            patterns: Vec::new(),
//            weight: 0,
//        }],
//    };
//
//    image_sp.pixels[14] = PixelSuperposition {
//        colors: vec![ColorSuperposition {
//            color: image.colors[14],
//            patterns: Vec::new(),
//            weight: 0,
//        }],
//    };
//
//    image_sp.pixels[345] = PixelSuperposition {
//        colors: vec![ColorSuperposition {
//            color: image.colors[345],
//            patterns: Vec::new(),
//            weight: 0,
//        }],
//    };
//
//    image_sp.pixels[359] = PixelSuperposition {
//        colors: vec![ColorSuperposition {
//            color: image.colors[359],
//            patterns: Vec::new(),
//            weight: 0,
//        }],
//    };
//}
