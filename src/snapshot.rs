use crate::{pattern::Pattern, superposition::ImageSuperposition};

pub struct Snapshot<const N: usize, T: Pattern<N>> {
    pub image_sp: ImageSuperposition<N, T>,
    pub collapse_pixel_index: usize,
    pub collapse_color_index: usize,
}

pub struct SnapshotStack<const N: usize, T: Pattern<N>> {
    stack: Vec<Snapshot<N, T>>,
}

impl<const N: usize, T: Pattern<N>> SnapshotStack<N, T> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, snapshot: Snapshot<N, T>) {
        self.stack.push(snapshot);
    }

    pub fn pop(&mut self) -> Option<Snapshot<N, T>> {
        if let Some(mut snapshot) = self.stack.pop() {
            let pixel_index = snapshot.collapse_pixel_index;
            let color_index = snapshot.collapse_color_index;

            snapshot.image_sp.pixels[pixel_index]
                .colors
                .swap_remove(color_index);

            return Some(snapshot);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

//fn get_collapse_color_index<const N: usize, T: Pattern<N>>(
//    colors: &Vec<ColorSuperposition<N, T>>,
//    color: Color,
//) -> usize {
//    for i in 0..colors.len() {
//        if colors[i].color == color {
//            return i;
//        }
//    }
//
//    panic!("collapsed color must exist");
//}
