use image::Image;
use image::load_image;
use image::save_image;
use pattern8::Pattern8;
use snapshot::Snapshot;
use snapshot::SnapshotStack;
use superposition::ImageSuperposition;
use superposition::Wfc;

pub mod color;
pub mod image;
pub mod pattern;
pub mod pattern8;
pub mod pixel;
pub mod snapshot;
pub mod stack_set;
pub mod superposition;
pub mod vec2;
pub mod weighted;

fn main() {
    println!("Hello, world!");

    let mut snapshot_stack = SnapshotStack::<8, Pattern8>::new();
    let image = load_image("./test/Water.png");

    let mut image_sp = ImageSuperposition::<8, Pattern8>::new(50, 50);
    image_sp.extract(image);

    image_sp.propagate_all();
    save_image(
        Image::from(&image_sp),
        format!("./test/out-before-collapse.png"),
    );

    // TODO: debug
    //let mut n = 0;
    while let Some(pixel_index) = image_sp.search() {
        //save_image(Image::from(&image_sp), format!("./test/out{}.png", n));
        //n += 1;

        let image_sp_clone = image_sp.clone();

        let collapse_color_index = image_sp.collapse(pixel_index);

        snapshot_stack.push(Snapshot {
            image_sp: image_sp_clone,
            collapse_pixel_index: pixel_index,
            collapse_color_index,
        });

        //let is_contradiction = !image_sp.propagate(pixel_index);
        //if is_contradiction {
        //    // TODO: Option needed?
        //    println!("restore");
        //    let snapshot = snapshot_stack.pop().unwrap();
        //    image_sp = snapshot.image_sp;
        //    image_sp.propagate(snapshot.collapse_pixel_index);
        //}


        let mut pi = pixel_index;
        while !image_sp.propagate(pi) {
            // TODO: Option needed?
            println!("restore, stack size: {}", snapshot_stack.len());
            let snapshot = snapshot_stack.pop().unwrap();
            image_sp = snapshot.image_sp;
            pi = snapshot.collapse_pixel_index;
        }
    }

    let image_out = Image::from(&image_sp);
    save_image(image_out, "./test/out.png");
}
