use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coords3D(pub i32, pub i32, pub i32);

#[derive(Default)]
pub struct Coords2D(pub i32, pub i32);

#[derive(Clone, Copy, PartialEq)]
pub enum Direction3D {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

pub enum Direction2D {
    North,
    South,
    East,
    West,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Message {
    Popup(String),
    Log(String),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub enum Color {
    #[default]
    White,
    Red,
    Green,
    Blue,
}

impl Coords3D {
    /// This approach uses the Pythagorean theorem to find the distance between
    /// the x and z coordinates, but doesn't account for diagonal movement 
    /// alonog the y axis. This is because it's ordinarily much harder to move
    /// along the y axis compared to x and z.
    pub fn distance_from(&self, coords: &Coords3D) -> f64 {
        let x_delta = self.0 - coords.0;
        let y_delta = self.1 - coords.1;
        let z_delta = self.2 - coords.2;

        let horizontal_distance = (((x_delta.pow(2)) + (z_delta.pow(2))) as f64).sqrt();

        horizontal_distance + (y_delta as f64).abs()
    }

    pub fn move_in_direction(&mut self, direction: Direction3D) {
        match direction {
            Direction3D::East => {
                self.0 += 1;
            }
            Direction3D::West => {
                self.0 -= 1;
            }
            Direction3D::South => {
                self.1 += 1;
            }
            Direction3D::North => {
                self.1 -= 1;
            }
            Direction3D::Up => {
                self.2 += 1;
            }
            Direction3D::Down => {
                self.2 -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance_from() {
        let coords_1 = Coords3D(3, 9, 8);
        let coords_2 = Coords3D(6, 4, 4);

        let distance = coords_1.distance_from(&coords_2);
        assert!(distance > 9.95 && distance < 10.05);
    }
}
