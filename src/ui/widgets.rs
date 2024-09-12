use ratatui::widgets::{Block, BorderType, Borders};

pub fn default_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
}
