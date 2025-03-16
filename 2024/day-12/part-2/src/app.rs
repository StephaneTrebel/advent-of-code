use aoc2024::{get_file_content, Map};

use color_eyre::Result;
use crossterm::event::{self, Event};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal,
};

/// RataTUI application definition
pub struct App {
    text_map: Vec<String>,
}

impl App {
    /// Create a new application
    pub fn new() -> Self {
        let content = get_file_content("assets/input");
        let map = Map::from(content.as_str());
        let (max_x, max_y) = map.get_bounding_rect();
        let mut text_map: Vec<String> = vec![];

        for y in 0..=max_y {
            let mut line: Vec<char> = vec![];
            for x in 0..=max_x {
                if let Some(c) = map.get(&(x, y)) {
                    line.push(*c);
                }
            }
            text_map.push(line.into_iter().collect());
        }

        Self { text_map }
    }

    /// Run the application
    pub fn run(&self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
            if matches!(event::read()?, Event::Key(_)) {
                break Ok(());
            }
        }
    }
}

impl Widget for &App {
    /// Render a frame
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let iter = self.text_map.clone().into_iter();
        Paragraph::new(iter.map(Line::from).collect::<Vec<Line>>())
            .block(Block::bordered().title("Day-12"))
            .render(area, buffer);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
