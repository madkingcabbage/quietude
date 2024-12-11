use std::{collections::HashMap, fmt::{Debug, Formatter}, sync::OnceLock};

use anyhow::{anyhow, Result};
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::world::world::World;

use super::{control_scheme::ControlSchemeType, ui_callback::UiCallbackPreset};

pub trait Screen {
    fn update(&mut self, world: &World) -> Result<()>;

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()>;

    fn handle_key_events(&mut self, key_event: KeyEvent, scheme: ControlSchemeType, world: &World)
        -> Option<UiCallbackPreset>;

    fn refresh_rate(&self) -> u16;
}

pub trait ChoiceAttribute: StringLookup where Self: 'static {
    fn choices() -> Vec<&'static str> {
        let mut choices = vec![];
        for (_, v) in &Self::dictionary().dict {
            choices.push(*v);
        }
        choices
    }

    fn from_str(s: &str) -> Result<&Self> {
        for (k, v) in &Self::dictionary().dict {
            if s == *v {
                return Ok(k)
            }
        }
        Err(anyhow!("could not convert {s} to desired type"))
    }
}

pub struct StringLookupDictionary<T: Sized + 'static> {
    pub dict: Vec<(&'static T, &'static str)>,
}

pub trait StringLookup: Sized {
    fn dictionary() -> &'static StringLookupDictionary<Self>;
}

impl StringLookup for bool {
    fn dictionary() -> &'static StringLookupDictionary<Self> {
        static DICT: OnceLock<StringLookupDictionary<bool>> = OnceLock::new();
        DICT.get_or_init(|| {
            let dict = vec![
                (&false, "false"),
                (&true, "true"),
            ];
            let dict = StringLookupDictionary {
                dict
            };
            dict
        })
    }
}

impl ChoiceAttribute for bool {}

impl Debug for dyn Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Screen {:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_string_lookup() {
        assert_eq!(bool::choices(), ["false", "true"]);
        assert_eq!(*bool::from_str("false").unwrap(), false);
        assert_eq!(*bool::from_str("true").unwrap(), true);
        assert!(bool::from_str("test").is_err());
    }
}
