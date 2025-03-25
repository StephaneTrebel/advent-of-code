use std::time::{Duration, Instant};

use aoc2024::{
    get_area, get_exploded_map, get_exploded_region, get_file_content, get_perimeter, get_regions,
    get_sides, Map, Region,
};

use color_eyre::{eyre::Ok, Result};
use crossterm::event::{self, poll, Event};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

/// RataTUI application definition
pub struct App {
    map: Map,
    exploded_map: Map,
    text_map: Vec<String>,
    current_region_index: usize,
    regions: Vec<Region>,
    last_tick: Instant,

    area: usize,
    perimeter: usize,
    sides: usize,
    part1: usize,
    part2: usize,
}

impl App {
    const TICK_RATE: Duration = Duration::from_millis(100);

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
            exploded_map: get_exploded_map(&map),
            map,
            text_map,
            current_region_index: 0,
            regions,
            last_tick: Instant::now(),

            area: 0,
            perimeter: 0,
            sides: 0,
            part1: 0,
            part2: 0,
        }
    }

    /// Run the application
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            // Check for events
            if poll(Self::TICK_RATE)? {
                if let Event::Key(_) = event::read()? {
                    break Ok(());
                }
            }

            // Update state every tick
            if self.last_tick.elapsed() >= Self::TICK_RATE {
                self.update_state();
                self.last_tick = Instant::now();
            }

        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let chunks =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Fill(1)]).split(area);

        let map = Paragraph::new(
            self.text_map
                .clone()
                .into_iter()
                .map(Line::from)
                .collect::<Vec<Line>>(),
        )
        .block(Block::bordered().title("Day-12"));
        frame.render_widget(map, chunks[0]);

        let control_tower = Paragraph::new("").block(Block::bordered().title("Control Tower"));
        frame.render_widget(control_tower, chunks[1]);

        let control_tower_layout = Layout::vertical([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Fill(1),
        ])
        .split(chunks[1].inner(Margin::new(1, 1)));

        let region_area_widget = Paragraph::new(format!("Area : {}", self.area));
        frame.render_widget(region_area_widget, control_tower_layout[0]);
        let region_perimeter_widget = Paragraph::new(format!("Perimeter : {}", self.perimeter));
        frame.render_widget(region_perimeter_widget, control_tower_layout[1]);
        let region_sides_widget = Paragraph::new(format!("Sides : {}", self.sides));
        frame.render_widget(region_sides_widget, control_tower_layout[2]);
        let region_count_widget =
            Paragraph::new(format!("Count : {}", self.current_region_index + 1));
        frame.render_widget(region_count_widget, control_tower_layout[3]);
        let region_part1_widget = Paragraph::new(format!("Part1 : {}", self.part1));
        frame.render_widget(region_part1_widget, control_tower_layout[4]);
        let region_part2_widget = Paragraph::new(format!("Part2 : {}", self.part2));
        frame.render_widget(region_part2_widget, control_tower_layout[5]);

        let block = Block::bordered().title("Region");
        let region_paragraph = Paragraph::new("").block(block);
        frame.render_widget(region_paragraph, control_tower_layout[6]);

        if self.current_region_index < self.regions.len() {
            let region = self.regions[self.current_region_index].clone();
            let plant = *self
                .map
                .get(region.0.iter().next().expect("YOLO"))
                .expect("YOLO");

            let region_in_control_tower_widget =
                RegionInControlTowerWidget::new(region.clone(), plant);
            frame.render_widget(
                region_in_control_tower_widget,
                control_tower_layout[6].inner(Margin {
                    horizontal: 1,
                    vertical: 1,
                }),
            );

            let region_widget = RegionInMapWidget::new(region.clone(), plant);
            frame.render_widget(region_widget, chunks[0]);
        }
    }

    fn update_state(&mut self) {
        if self.current_region_index < self.regions.len() {
            let region = self.regions[self.current_region_index].clone();
            let exploded_region = get_exploded_region(&region);
            self.area = get_area(&region);
            self.perimeter = get_perimeter(&self.map, &region);
            self.sides = get_sides(&self.exploded_map, &exploded_region);
            self.part1 += self.area * self.perimeter;
            self.part2 += self.area * self.sides;
            self.current_region_index += 1;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct RegionInMapWidget {
    region: Region,
    plant: char,
}

impl RegionInMapWidget {
    pub fn new(region: Region, plant: char) -> Self {
        RegionInMapWidget { region, plant }
    }
}

impl Widget for RegionInMapWidget {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        for (x, y) in self.region.0 {
            if ((x as u16) < area.width - 2) && ((y as u16) < area.height - 2) {
                buffer.set_string(
                    area.x + (x as u16) + 1,
                    area.y + (y as u16) + 1,
                    self.plant.to_string(),
                    Style::default().fg(Color::Yellow),
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
struct RegionInControlTowerWidget {
    region: Region,
    plant: char,
}

impl RegionInControlTowerWidget {
    pub fn new(region: Region, plant: char) -> Self {
        RegionInControlTowerWidget { region, plant }
    }
}

impl Widget for RegionInControlTowerWidget {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let ((min_x, min_y), _) = self.region.get_bounding_rect();
        for (x, y) in self.region.0 {
            let abs_x = (x - min_x) as u16;
            let abs_y = (y - min_y) as u16;
            if (abs_x < area.width - 1) && (abs_y < area.height - 1) {
                buffer.set_string(
                    area.x + abs_x,
                    area.y + abs_y,
                    self.plant.to_string(),
                    Style::default().fg(Color::LightBlue),
                );
            }
        }
    }
}
