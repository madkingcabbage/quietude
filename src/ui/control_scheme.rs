use std::{collections::HashMap, default, sync::OnceLock};

use crossterm::event::KeyCode;

#[derive(Default)]
pub enum ControlSchemeType {
    #[default]
    Default,
}

struct ControlScheme {
    controls: HashMap<KeyCode, Vec<UiKey>>,
}

#[derive(PartialEq)]
pub enum UiKey {
    YesToDialog,
    NoToDialog,
    MoveNorth,
    MoveEast,
    MoveWest,
    MoveSouth,
    Quit,
}


impl ControlScheme {
    fn code_yields_key(&self, code: KeyCode, key: UiKey) -> bool {
        let keys = self.controls.get(&code);
        if keys.is_none() {
            return false;
        }
        let keys = keys.unwrap();
        for key_try in keys {
            if key == *key_try {
                return true
            }
        }
        false
    }

    fn default_scheme() -> &'static ControlScheme {
        static SCHEME: OnceLock<ControlScheme> = OnceLock::new();
        SCHEME.get_or_init(|| {
            let mut scheme = ControlScheme { controls: HashMap::new() };
            scheme.controls.insert(KeyCode::Char('q'), vec![UiKey::Quit]);
            scheme.controls.insert(KeyCode::Char('Q'), vec![UiKey::Quit]);
            scheme.controls.insert(KeyCode::Char('w'), vec![UiKey::MoveNorth]);
            scheme.controls.insert(KeyCode::Char('a'), vec![UiKey::MoveWest]);
            scheme.controls.insert(KeyCode::Char('s'), vec![UiKey::MoveSouth]);
            scheme.controls.insert(KeyCode::Char('d'), vec![UiKey::MoveEast]);
            scheme
        })
    }
}

impl ControlSchemeType {
    pub fn code_yields_key(&self, code: KeyCode, key: UiKey) -> bool {
        match self {
            ControlSchemeType::Default => ControlScheme::default_scheme()
        }.code_yields_key(code, key)
    }
}
