use std::ops::Add;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vec2 {
    pub fn from_index(index: usize, width: u32) -> Vec2 {
        Vec2 {
            x: (index as u32 % width) as i32,
            y: (index as u32 / width) as i32,
        }
    }

    pub fn into_index(&self, width: u32) -> usize {
        (width as i32 * self.y + self.y) as usize
    }

    pub fn is_inside(&self, width: u32, height: u32) -> bool {
        if self.x < 0 {
            return false;
        }
        if self.y < 0 {
            return false;
        }
        if self.x >= width as i32 {
            return false;
        }
        if self.y >= height as i32 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_calculates_the_position_from_an_index() {
        let pos = Vec2::from_index(23, 10);
        assert_eq!(pos.x, 3);
        assert_eq!(pos.y, 2);
    }

    #[test]
    fn it_ensures_to_be_inside_a_rectangle() {
        let pos = Vec2 { x: 0, y: 0 };
        assert!(pos.is_inside(10, 10));

        let pos = Vec2 { x: 1, y: 1 };
        assert!(pos.is_inside(10, 10));

        let pos = Vec2 { x: 1, y: 9 };
        assert!(pos.is_inside(10, 10));

        let pos = Vec2 { x: 9, y: 1 };
        assert!(pos.is_inside(10, 10));
    }

    #[test]
    fn it_is_outside_of_rectangle() {
        let pos = Vec2 { x: -1, y: 5 };
        assert!(!pos.is_inside(10, 10));

        let pos = Vec2 { x: 1, y: -10 };
        assert!(!pos.is_inside(10, 10));

        let pos = Vec2 { x: 1, y: 10 };
        assert!(!pos.is_inside(10, 10));

        let pos = Vec2 { x: 11, y: 10 };
        assert!(!pos.is_inside(10, 10));
    }
}
