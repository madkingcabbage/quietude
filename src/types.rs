use std::fmt::{write, Display};

use ratatui::{
    style::Style,
    text::{Line, Span},
};
use serde::{Deserialize, Serialize};

use crate::{ui::popup_message::PopupStyle, world::log::LogStyle};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coords4D(pub i32, pub i32, pub i32, pub i32);

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coords3D(pub i32, pub i32, pub i32);

#[derive(Default, PartialEq)]
pub struct Coords2D(pub i32, pub i32);

#[derive(Clone, Copy, PartialEq)]
pub enum Direction3D {
    North,
    Northeast,
    Northwest,
    South,
    Southeast,
    Southwest,
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

#[derive(Clone, Copy, PartialEq)]
pub enum Direction1D {
    Up,
    Down,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Message {
    Popup(FormattedString<PopupStyle>),
    Log(FormattedString<LogStyle>),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct FormattedString<T> {
    texts: Vec<FormattedText<T>>,
    origin: Option<Coords3D>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct FormattedText<T> {
    s: String,
    style: T,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub enum GenericStyle {
    #[default]
    Default,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub enum Color {
    #[default]
    White,
    Red,
    Green,
    Blue,
}

pub struct LineSegment2D {
    pub start: Coords2D,
    pub end: Coords2D,
}

pub struct LineSegment3D {
    pub start: Coords3D,
    pub end: Coords3D,
}

impl Coords4D {
    pub fn to_str(&self) -> String {
        String::from(format!("{},{},{},{}", self.0, self.1, self.2, self.3))
    }
}

impl Display for Coords4D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.0, self.1, self.2, self.3)
    }
}

impl Display for Coords3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.0, self.1, self.2)
    }
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

    pub fn move_in_direction(&mut self, direction: &Direction3D) {
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
            Direction3D::Northeast => {
                self.move_in_direction(&Direction3D::North);
                self.move_in_direction(&Direction3D::East);
            }
            Direction3D::Northwest => {
                self.move_in_direction(&Direction3D::North);
                self.move_in_direction(&Direction3D::West);
            }
            Direction3D::Southeast => {
                self.move_in_direction(&Direction3D::South);
                self.move_in_direction(&Direction3D::East);
            }
            Direction3D::Southwest => {
                self.move_in_direction(&Direction3D::South);
                self.move_in_direction(&Direction3D::West);
            }
        }
    }

    pub fn direction_to(&self, coords: &Coords3D) -> Direction3D {
        let delta_x = self.0 - coords.0;
        let delta_y = self.1 - coords.1;
        let delta_z = self.2 - coords.2;

        let mut direction = Direction3D::North;
        if (delta_y > delta_x.abs()) && (delta_y >= 0) {
            if delta_y as f64 / 2.0 >= delta_x as f64 {
                direction = Direction3D::South;
            } else if delta_x > 0 {
                direction = Direction3D::Southeast;
            } else if delta_x <= 0 {
                direction = Direction3D::Southwest;
            }
        } else if (delta_y.abs() >= delta_x.abs()) && (delta_y <= 0) {
            if delta_y.abs() as f64 / 2.0 > delta_x.abs() as f64 {
                direction = Direction3D::North;
            } else if delta_x > 0 {
                direction = Direction3D::Northeast;
            } else if delta_x <= 0 {
                direction = Direction3D::Northwest;
            }
        } else if (delta_x >= delta_y.abs()) && (delta_x >= 0) {
            if delta_x.abs() as f64 / 2.0 > delta_y.abs() as f64 {
                direction = Direction3D::East;
            } else if delta_y > 0 {
                direction = Direction3D::Southeast;
            } else if delta_y <= 0 {
                direction = Direction3D::Northeast;
            }
        } else if (delta_x >= delta_y.abs()) && (delta_x <= 0) {
            if delta_x.abs() as f64 / 2.0 > delta_y.abs() as f64 {
                direction = Direction3D::West;
            } else if delta_y > 0 {
                direction = Direction3D::Southwest;
            } else if delta_y <= 0 {
                direction = Direction3D::Northwest;
            }
        }

        if (delta_x == 0) && (delta_y == 0) {
            if delta_z < 0 {
                direction = Direction3D::Down;
            } else if delta_z > 0 {
                direction = Direction3D::Up;
            }
        }

        direction
    }
}

impl PartialOrd for Coords3D {
    fn lt(&self, other: &Self) -> bool {
        if self.2 < other.2 {
            return true;
        } else if self.2 > other.2 {
            return false;
        }

        if self.1 < other.1 {
            return true;
        } else if self.1 > other.1 {
            return false;
        }

        if self.0 < other.0 {
            return true;
        } else if self.0 > other.0 {
            return false;
        }

        false

    }
    
    fn gt(&self, other: &Self) -> bool {
        if self.2 > other.2 {
            return true;
        } else if self.2 < other.2 {
            return false;
        }

        if self.1 > other.1 {
            return true;
        } else if self.1 < other.1 {
            return false;
        }

        if self.0 > other.0 {
            return true;
        } else if self.0 < other.0 {
            return false;
        }

        false
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.2.partial_cmp(&other.2) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.1.partial_cmp(&other.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.0.partial_cmp(&other.0)
    }
}

impl Direction3D {
    pub fn invert(&mut self) {
        *self = match self {
            Direction3D::North => Direction3D::South,
            Direction3D::Northeast => Direction3D::Southwest,
            Direction3D::Northwest => Direction3D::Southeast,
            Direction3D::South => Direction3D::North,
            Direction3D::Southeast => Direction3D::Northwest,
            Direction3D::Southwest => Direction3D::Northeast,
            Direction3D::East => Direction3D::West,
            Direction3D::West => Direction3D::East,
            Direction3D::Up => Direction3D::Down,
            Direction3D::Down => Direction3D::Up,
        };
    }
}

impl LineSegment2D {
    pub fn intersects(&self) -> Vec<Coords2D> {
        const ITERATIONS: usize = 100;

        let mut intersections = vec![];
        let delta_x = self.start.0 - self.end.0;
        let delta_y = self.start.1 - self.end.1;

        let x_step = (delta_x as f64) / ITERATIONS as f64;
        let y_step = (delta_y as f64) / ITERATIONS as f64;

        let (mut current_x, mut current_y) = (self.start.0 as f64, self.start.1 as f64);

        for _ in 0..ITERATIONS {
            current_x += x_step;
            current_y += y_step;

            if !intersections.contains(&Coords2D(current_x as i32, current_y as i32)) {
                intersections.push(Coords2D(current_x as i32, current_y as i32));
            }
        }

        intersections
    }
}

impl LineSegment3D {
    pub fn intersects(&self) -> Vec<Coords3D> {
        const ITERATIONS: usize = 100;

        let mut intersections = vec![];
        let delta_x = self.start.0 - self.end.0;
        let delta_y = self.start.1 - self.end.1;
        let delta_z = self.start.2 - self.end.2;

        let x_step = (delta_x as f64) / ITERATIONS as f64;
        let y_step = (delta_y as f64) / ITERATIONS as f64;
        let z_step = (delta_z as f64) / ITERATIONS as f64;

        let (mut current_x, mut current_y, mut current_z) = (
            self.start.0 as f64,
            self.start.1 as f64,
            self.start.2 as f64,
        );

        for _ in 0..ITERATIONS {
            current_x += x_step;
            current_y += y_step;
            current_z += z_step;

            if !intersections.contains(&Coords3D(
                current_x as i32,
                current_y as i32,
                current_z as i32,
            )) {
                intersections.push(Coords3D(
                    current_x as i32,
                    current_y as i32,
                    current_z as i32,
                ));
            }
        }

        intersections
    }
}

impl<T> Display for FormattedString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.texts.iter().map(|text| text.s.clone()).collect();
        write!(f, "{s}")
    }
}

impl<T: Into<Style> + Clone + Default> FormattedString<T> {
    pub fn from(origin: &Option<Coords3D>, text: FormattedText<T>) -> Self {
        FormattedString {
            origin: origin.clone(),
            texts: vec![text],
        }
    }

    pub fn insert(&mut self, index: usize, text: FormattedText<T>) {
        self.texts.insert(index, text);
    }

    pub fn raw(origin: &Option<Coords3D>, s: &str) -> Self {
        let texts = vec![FormattedText { s: String::from(s), style: <T>::default() }];
        FormattedString {
            origin: origin.clone(),
            texts,
        }
    }

    pub fn push(&mut self, text: FormattedText<T>) {
        self.texts.push(text);
    }

    pub fn origin(&self) -> &Option<Coords3D> {
        &self.origin
    }

    pub fn truncate(&self, max_chars: usize) -> FormattedString<T> {
        let mut char_count = 0;
        let mut string = FormattedString {
            texts: vec![],
            origin: self.origin,
        };
        
        let max_chars = max_chars - 3;

        for text in &self.texts {
            char_count += text.s.len();
            if char_count < max_chars {
                string.texts.push(text.clone());
            } else {
                let difference = char_count - max_chars;
                let text = FormattedText::new(&text.s[0..=text.s.len() - difference].to_string(), text.style.clone());
                string.texts.push(text);
                string.texts.push(FormattedText::new("...", Default::default()));
                break;
            }
        }

        string
    }

    pub fn into_spans(s: &FormattedString<T>) -> Vec<Span> {
        let spans = s
            .texts
            .iter()
            .map(|text| Span::styled(text.s.clone(), text.style.clone()))
            .collect();

        spans
    }
}

impl<T: Into<Style> + Clone> FormattedText<T> {
    pub fn new(s: &str, style: T) -> Self {
        FormattedText {
            s: String::from(s),
            style,
        }
    }

    pub fn truncate(&self, max_chars: usize) -> FormattedText<T> {
        let max_chars = max_chars - 3;
        if self.s.len() < max_chars as usize {
            FormattedText::new(&self.s, self.style.clone())
        } else {
            let mut s = String::from(&self.s[0..=max_chars]);
            s.push_str("...");
            FormattedText::new(&s, self.style.clone())
        }
    }
}

impl From<GenericStyle> for Style {
    fn from(value: GenericStyle) -> Self {
        Style::new()
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

    #[test]
    fn test_intersects() {
        let start = Coords3D(0, 0, 0);
        let end = Coords3D(5, 10, 0);
        let line_segment = LineSegment3D { start, end };
        let intersections = vec![
            Coords3D(0, 0, 0),
            Coords3D(0, 1, 0),
            Coords3D(1, 1, 0),
            Coords3D(1, 2, 0),
            Coords3D(1, 3, 0),
            Coords3D(2, 3, 0),
            Coords3D(2, 4, 0),
            Coords3D(2, 5, 0),
            Coords3D(3, 5, 0),
            Coords3D(3, 6, 0),
            Coords3D(3, 7, 0),
            Coords3D(4, 7, 0),
            Coords3D(4, 8, 0),
            Coords3D(4, 9, 0),
            Coords3D(5, 9, 0),
            Coords3D(5, 10, 0),
        ];

        assert_eq!(intersections, line_segment.intersects());
    }

    #[test]
    fn test_partial_cmp_coords_3d() {
        assert!(Coords3D(5, 5, 1) > Coords3D(6, 6, 0));
        assert!(Coords3D(5, 1, 8) > Coords3D(6, 0, 8));
        assert!(Coords3D(7, 9, 2) > Coords3D(6, 9, 2));
        
        assert!(!(Coords3D(5, 5, 1) < Coords3D(6, 6, 0)));
        assert!(!(Coords3D(5, 1, 8) < Coords3D(6, 0, 8)));
        assert!(!(Coords3D(7, 9, 2) < Coords3D(6, 9, 2)));
    }
}
