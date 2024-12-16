use std::{collections::HashMap, sync::OnceLock};

use crossterm::event::KeyCode;

#[derive(Default, Clone, Copy)]
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
    MoveDown,
    MoveUp,
    EditEntity,
    AddItem,
    RemoveItem,
    Confirm,
    ExitSubmenu,
    Save,
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
                return true;
            }
        }
        false
    }

    fn default_scheme() -> &'static ControlScheme {
        static SCHEME: OnceLock<ControlScheme> = OnceLock::new();
        SCHEME.get_or_init(|| {
            let mut scheme = ControlScheme {
                controls: HashMap::new(),
            };
            scheme
                .controls
                .insert(KeyCode::Char('q'), vec![UiKey::Quit]);
            scheme
                .controls
                .insert(KeyCode::Char('Q'), vec![UiKey::Quit]);
            scheme
                .controls
                .insert(KeyCode::Char('w'), vec![UiKey::MoveNorth, UiKey::MoveUp]);
            scheme
                .controls
                .insert(KeyCode::Char('a'), vec![UiKey::MoveWest, UiKey::AddItem]);
            scheme
                .controls
                .insert(KeyCode::Char('s'), vec![UiKey::MoveSouth, UiKey::MoveDown]);
            scheme
                .controls
                .insert(KeyCode::Char('d'), vec![UiKey::MoveEast]);
            scheme
                .controls
                .insert(KeyCode::Up, vec![UiKey::MoveNorth, UiKey::MoveUp]);
            scheme
                .controls
                .insert(KeyCode::Left, vec![UiKey::MoveWest]);
            scheme
                .controls
                .insert(KeyCode::Down, vec![UiKey::MoveSouth, UiKey::MoveDown]);
            scheme
                .controls
                .insert(KeyCode::Right, vec![UiKey::MoveEast]);
            scheme
                .controls
                .insert(KeyCode::Enter, vec![UiKey::Confirm, UiKey::YesToDialog]);
            scheme
                .controls
                .insert(KeyCode::Char('y'), vec![UiKey::YesToDialog]);
            scheme
                .controls
                .insert(KeyCode::Char('n'), vec![UiKey::NoToDialog]);
            scheme
                .controls
                .insert(KeyCode::Char('x'), vec![UiKey::Save]);
            scheme
                .controls
                .insert(KeyCode::Char('e'), vec![UiKey::EditEntity, UiKey::AddItem]);
            scheme
                .controls
                .insert(KeyCode::Char('r'), vec![UiKey::RemoveItem]);
            scheme
                .controls
                .insert(KeyCode::Esc, vec![UiKey::ExitSubmenu]);
            scheme
        })
    }
}

impl ControlSchemeType {
    pub fn code_yields_key(&self, code: KeyCode, key: UiKey) -> bool {
        match self {
            ControlSchemeType::Default => ControlScheme::default_scheme(),
        }
        .code_yields_key(code, key)
    }

    pub fn keys_from_code(&self, code: KeyCode) -> Option<&'static Vec<UiKey>> {
        match self {
            ControlSchemeType::Default => ControlScheme::default_scheme(),
        }
        .controls.get(&code)
    }
}
