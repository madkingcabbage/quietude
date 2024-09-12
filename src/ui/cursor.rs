use crossterm::cursor::SetCursorStyle;

use crate::types::Coords3D;

pub type CursorStyle = SetCursorStyle; 

pub struct Cursor {
    pub coords: Coords3D,
    pub style: CursorStyle,
    pub visible: bool,
}

impl Cursor {
    pub fn toggle_blinking(&mut self) {
        self.style = match self.style {
            CursorStyle::SteadyBar => CursorStyle::BlinkingBar,
            CursorStyle::BlinkingBar => CursorStyle::SteadyBar,
            CursorStyle::SteadyBlock => CursorStyle::BlinkingBlock,
            CursorStyle::BlinkingBlock => CursorStyle::SteadyBlock,
            CursorStyle::SteadyUnderScore => CursorStyle::BlinkingUnderScore,
            CursorStyle::BlinkingUnderScore => CursorStyle::SteadyUnderScore,
            CursorStyle::DefaultUserShape => CursorStyle::BlinkingBar,
        };
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            coords: Coords3D(0, 0, 0),
            style: CursorStyle::SteadyBlock,
            visible: false,
        }
    }
}
