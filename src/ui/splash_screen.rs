use std::{default, iter};

use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

use crate::world::world::World;

use super::{traits::Screen, ui_callback::UiCallbackPreset};

#[derive(Debug)]
pub struct SplashScreen {
    outline: Vec<Vec<bool>>,
    contents: Vec<Vec<char>>,
    pub state: State,
    tick: u32,
    direction: Direction,
    title: &'static str,
    title_color: Color,
    refresh_rate: u16,
}

#[derive(Default, Debug, PartialEq)]
pub enum State {
    #[default]
    Start,
    Sweep,
    Wait,
    Unsweep,
    Done,
}

#[derive(Default, Debug)]
pub enum Direction {
    #[default]
    SouthWest,
}

const TITLES: [&'static str; 3] = ["quietude", "quintessence", "quark"];
const COLORS: [Color; 5] = [
    Color::Red,
    Color::Yellow,
    Color::Magenta,
    Color::Green,
    Color::Blue,
];

const BYLINE: &'static str = "from liv, with love";

const PRESET_Q: [&'static str; 24] = [
    "                QQQQQQQQQQQQQQQQ                ",
    "           QQQQQQQQQQQQQQQQQQQQQQQQQ            ",
    "         QQQQQQQQQQQQQQQQQQQQQQQQQQQQQQ         ",
    "      QQQQQQQQQQQ              QQQQQQQQQQ       ",
    "     QQQQQQQQ                      QQQQQQQQ     ",
    "   QQQQQQQQ                          QQQQQQQQ   ",
    "  QQQQQQQQ                            QQQQQQQ   ",
    "  QQQQQQQ                              QQQQQQQ  ",
    " QQQQQQQ                                QQQQQQQ ",
    " QQQQQQ                                 QQQQQQQ ",
    "QQQQQQQ                                  QQQQQQ ",
    "QQQQQQQ                                  QQQQQQ ",
    "QQQQQQQ                                  QQQQQQ ",
    " QQQQQQQ                                QQQQQQQ ",
    " QQQQQQQ                                QQQQQQQ ",
    "  QQQQQQQ                              QQQQQQQ  ",
    "   QQQQQQQ                 QQQQ       QQQQQQQ   ",
    "   QQQQQQQQ               QQQQQQQ   QQQQQQQQ    ",
    "     QQQQQQQQQ             QQQQQQQQQQQQQQQQ     ",
    "       QQQQQQQQQQQ           QQQQQQQQQQQQ       ",
    "         QQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQ      ",
    "            QQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQ   ",
    "                  QQQQQQQQQQQQ        QQQQQQQQ  ",
    "                                         QQQ    ",
];

const PRESET_TEST: [&'static str; 3] = ["Q Q   ", "   QQ ", "Q     "];

impl SplashScreen {
    pub fn new() -> Self {
        let preset_contents: Vec<String> = PRESET_Q
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();

        let outline: Vec<Vec<bool>> = preset_contents
            .iter()
            .map(|s| {
                s.chars()
                    .map(|ch| if ch == 'Q' { true } else { false })
                    .collect()
            })
            .collect();

        let mut contents: Vec<Vec<char>> = Vec::with_capacity(outline.len());
        for _ in 0..outline.len() {
            contents.push(Vec::with_capacity(outline[0].len()));
        }

        for line in &mut contents {
            for _ in 0..outline[0].len() {
                line.push(' ');
            }
        }

        Self {
            outline,
            contents,
            direction: Direction::default(),
            state: State::default(),
            title: TITLES.choose(&mut thread_rng()).unwrap(),
            title_color: *COLORS.choose(&mut thread_rng()).unwrap(),
            tick: 0,
            refresh_rate: 100,
        }
    }

    fn sweep(&mut self) {
        if self.state != State::Start {
            return;
        }
        self.state = State::Sweep;
    }

    fn unsweep(&mut self) {
        if self.state != State::Wait {
            return;
        }
        self.state = State::Unsweep;
        self.tick = 0;
    }

}

impl Screen for SplashScreen {
    fn handle_key_events(&mut self, _key: KeyEvent, _world: &World) -> Option<UiCallbackPreset> {
        match self.state {
            State::Start => self.sweep(),
            State::Wait => self.unsweep(),
            _ => {}
        }

        None
    }

    fn update(&mut self, _world: &World) -> Result<()> {
        if self.state == State::Unsweep
            && self.tick > self.outline.len() as u32 + self.outline[0].len() as u32 + 20
        {
            self.state = State::Done;
        }

        if self.state == State::Sweep
            && self.tick > self.outline.len() as u32 + self.outline[0].len() as u32 + 20
        {
            self.state = State::Wait;
        }

        if self.state != State::Sweep && self.state != State::Unsweep {
            return Ok(());
        }

        self.tick += 1;

        for line in &mut self.contents {
            for ch in line {
                *ch = match self.state {
                    State::Sweep => match ch {
                        '.' => '●',
                        '●' => '※',
                        '※' => 'Q',
                        'Q' => 'Q',
                        _ => ' ',
                    },
                    State::Unsweep => match ch {
                        'Q' => 'Q',
                        '.' => ' ',
                        '●' => '.',
                        '※' => '●',
                        _ => ' ',
                    },
                    _ => panic!("tick attempted without being prepared"),
                }
            }
        }

        let len = self.outline.len() * 2;
        let mut col = 0;
        let mut row = 0;

        // used to be iterations
        let corrector = len as i32 - (len as i32 - (self.tick as i32)).abs();
        let iterations = self.outline[0].len();

        if self.tick as usize > len {
            row = ((len as u32 - (corrector as u32)) / 2) as usize;
            if row == self.outline.len() {
                row -= 1;
            }
        } else {
            col = len - (corrector as usize);
        }

        for i in 0..iterations {
            if self.outline[row][col] {
                let target = match self.state {
                    State::Sweep => ' ',
                    State::Unsweep => 'Q',
                    _ => ' ',
                };
                if self.contents[row][col] == target {
                    self.contents[row][col] = match self.state {
                        State::Sweep => '.',
                        State::Unsweep => '※',
                        _ => panic!("tick attempted without being prepared"),
                    };
                }
            }

            let mut col_increment = i as usize % 1;
            let mut row_increment = i as usize % 2;

            if row_increment == 0 {
                row_increment = 1;
            } else {
                row_increment = 0;
            }

            if col_increment == 0 {
                col_increment = 1;
            } else {
                col_increment = 0;
            }

            if col + col_increment < self.outline[0].len() {
                col += col_increment;
            }
            if row + row_increment < self.outline.len() {
                row += row_increment;
            }
        }

        Ok(())
    }

    fn render(&mut self, f: &mut Frame, _world: &World, area: Rect) -> Result<()> {
        let screen = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(5),
                Constraint::Length(self.outline.len() as u16),
                Constraint::Min(5),
            ])
            .split(area);

        let sweep_row = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(self.outline[0].len() as u16),
                Constraint::Min(1),
            ])
            .split(screen[1]);

        let title_row = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(screen[2]);

        f.render_widget(Paragraph::new(self.to_string()), sweep_row[1]);

        let title_lines = vec![
            Line::from(Span::styled(
                self.title.to_uppercase(),
                Style::default().fg(self.title_color),
            )),
            Line::from(String::from("\n")),
            Line::from(Span::styled(BYLINE, Style::default())),
        ];

        f.render_widget(
            Paragraph::new(Text::from(title_lines)).alignment(Alignment::Center),
            title_row[1],
        );

        Ok(())
    }

    fn get_refresh_rate(&self) -> u16 {
        self.refresh_rate
    }
}

impl ToString for SplashScreen {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for line in &self.contents {
            for ch in line {
                s.push(*ch);
            }
            s.push('\n');
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contents_to_string() {
        let sweep = SplashScreen {
            outline: vec![],
            contents: vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
            ],
            direction: Direction::default(),
            state: State::default(),
            title_color: Color::default(),
            title: TITLES[1],
            tick: 0,
        };

        assert_eq!(sweep.to_string(), "abc\ndef\nghi\n");
    }
}
