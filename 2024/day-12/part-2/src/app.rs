use std::time::{Duration, Instant};

use aoc2024::{get_file_content, get_regions, Map, Region};

use color_eyre::Result;
use crossterm::event::{self, Event};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal,
};

/// RataTUI application definition
pub struct App {
    map: Map,
    text_map: Vec<String>,
    current_region_index: usize,
    regions: Vec<Region>,
    last_tick: Instant,
}

impl App {
    const TICK_RATE: Duration = Duration::from_millis(250);

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

        let regions = get_regions(&map);

        Self {
            map,
            text_map,
            current_region_index: 0,
            regions,
            last_tick: Instant::now(),
        }
    }

    /// Run the application
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            if matches!(event::read()?, Event::Key(_)) {
                break Ok(());
            } else if self.last_tick.elapsed() >= Self::TICK_RATE {
                self.on_tick();
                self.last_tick = Instant::now();
            }
        }
    }

    fn on_tick(&mut self) {
        self.current_region_index += 1;
    }
}

impl Widget for &App {
    /// Render a frame
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let iter = self.text_map.clone().into_iter();
        Paragraph::new(iter.map(Line::from).collect::<Vec<Line>>())
            .block(Block::bordered().title("Day-12"))
            .render(area, buffer);

        let region = self.regions[self.current_region_index].clone();
        let style = Style::default().fg(Color::Yellow);
        for (x, y) in region.0 {
            let character = self.map.get(&(x, y)).unwrap();
            let string = character.to_string();
            let symbol = string.as_str();
            buffer[((x as u16) + 1, (y as u16) + 1)]
                .set_symbol(symbol)
                .set_style(style);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
