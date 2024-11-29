use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style, Widget};
use ratatui::style::Stylize;
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, Padding, Paragraph};

#[derive(Default)]
pub struct BarComponent {
    bar_name: String,
    bar_name_alignment: Alignment,
    bar_value: f32,
    max_bar_value: f64,
    high_level: f32,
    high_high_level: f32,
    text_value: String,
    direction: Direction,
}

impl BarComponent {

    pub fn new(bar_name:  String, bar_value: f32, max_bar_value: f64,
               high_level: f32, high_high_level: f32, text_value: String,
               direction: Direction, bar_name_alignment: Alignment) -> Self {
        Self {
            bar_name,
            bar_name_alignment,
            bar_value,
            max_bar_value,
            high_level,
            high_high_level,
            text_value,
            direction,
        }
    }

    fn bar_chart(&self, bar_value: f32, max_bar_value: u64) -> BarChart<'static> {

        let bar = Bar::default().value(bar_value as u64)
            .style(self.bar_style(bar_value, self.high_level, self.high_high_level));

        let bar = if self.direction == Direction::Horizontal {
            bar.text_value(format!("{}{bar_value}", self.text_value))
        } else {
            bar
        };

        let bar_chart = BarChart::default()
            .value_style(self.bar_style(bar_value, self.high_level, self.high_high_level).reversed())
            .data(BarGroup::default().bars(&[bar]))
            .max(max_bar_value)
            .direction(self.direction);

        if self.direction == Direction::Vertical {
            bar_chart.bar_width(3)
        } else {
            bar_chart.bar_width(1)
        }
    }

    fn bar_style(&self, value: f32, high: f32, high_high: f32) -> Style {
        if value > high_high {
            Style::new().fg(Color::Red)
        } else if value > high {
            Style::new().fg(Color::Yellow)
        } else {
            Style::new().fg(Color::Green)
        }
    }
}

impl Widget for BarComponent {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let outer_block = if self.direction == Direction::Vertical {
            Block::bordered().title(self.bar_name.clone())
                .title_alignment(Alignment::Center)
                .padding(Padding::horizontal(1))
        } else {
            Block::bordered().title(self.bar_name.clone())
                .title_alignment(self.bar_name_alignment)
                .padding(Padding::new(1, 1, 1, 0))
        };

        let inner = outer_block.inner(area);

        outer_block.render(area, buf);

        let [bar_layout, grid_layout] = if self.direction == Direction::Vertical {
            Layout::horizontal([
                Constraint::Length(4),
                Constraint::Length(6),
            ]).areas(inner)
        } else {
            Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(2),
            ]).areas(inner)
        };

        let constraints = Constraint::from_percentages([25; 4]);

        let grid_layouts: [Rect; 4] = if self.direction == Direction::Vertical {
            Layout::vertical(constraints).areas(grid_layout)
        } else {
            Layout::horizontal(constraints).areas(grid_layout)
        };

        if self.direction == Direction::Vertical {
            // 0% position calc
            let mut n_str: String = String::new();
            let l = grid_layouts[0].bottom() - grid_layouts[0].top();
            for _ in 0..l - 1 {
                n_str.push('\n');
            }

            let par_0 = Paragraph::new(format!("{n_str}- 0%"));
            let par_25 = Paragraph::new("- 25%".to_string());
            let par_50 = Paragraph::new("- 50%".to_string());
            let par_75 = Paragraph::new("- 75%".to_string());
            let par_100 = Paragraph::new("- 100%".to_string());

            par_0.render(grid_layouts[3], buf);
            par_25.render(grid_layouts[3], buf);
            par_50.render(grid_layouts[2], buf);
            par_75.render(grid_layouts[1], buf);
            par_100.render(grid_layouts[0], buf);

        } else {

            let par_0 = Paragraph::new("|\n0%".to_string()).alignment(Alignment::Left);
            let par_25 = Paragraph::new("|\n25%".to_string()).alignment(Alignment::Left);
            let par_50 = Paragraph::new("|\n50%".to_string()).alignment(Alignment::Left);
            let par_75 = Paragraph::new("|\n75%".to_string()).alignment(Alignment::Left);
            let par_100 = Paragraph::new("|\n100%".to_string()).alignment(Alignment::Right);

            par_0.render(grid_layouts[0], buf);
            par_25.render(grid_layouts[1], buf);
            par_50.render(grid_layouts[2], buf);
            par_75.render(grid_layouts[3], buf);
            par_100.render(grid_layouts[3], buf);
        }

        // // 0% position calc
        // let mut n_str: String = String::new();
        // let l = grid_layouts[0].bottom() - grid_layouts[0].top();
        // for _ in 0..l - 1 {
        //     n_str.push('\n');
        // }
        //
        // let par_0 = Paragraph::new(format!("{n_str}- 0%"));
        // let par_25 = Paragraph::new(format!("- 25%"));
        // let par_50 = Paragraph::new(format!("- 50%"));
        // let par_75 = Paragraph::new(format!("- 75%"));
        // let par_100 = Paragraph::new(format!("- 100%"));
        //
        // par_0.render(grid_layouts[3], buf);
        // par_25.render(grid_layouts[3], buf);
        // par_50.render(grid_layouts[2], buf);
        // par_75.render(grid_layouts[1], buf);
        // par_100.render(grid_layouts[0], buf);

        let bar = self.bar_chart(self.bar_value, self.max_bar_value as u64);
        bar.render(bar_layout, buf);
    }
}