
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Style};
use ratatui::style::Color;
use ratatui::symbols;
use ratatui::widgets::{Axis, Block, Chart, Dataset, StatefulWidget, Widget};

pub struct TrendComponent {
    trend_name: String,
    trend_color: Color,
    y_max: f64,
    y_min: f64,
    y_title: String,
    x_title: String,

    trend_value: f64,
    chart_window: usize,
}

impl TrendComponent {
    pub fn new(trend_name: String, trend_color: Color,
               y_max: f64, y_min: f64, y_title: String,
               x_title: String, chart_window: usize, trend_value: f64) -> Self {
        Self {
            trend_name,
            trend_color,
            y_max,
            y_min,
            y_title,
            x_title,

            trend_value,
            chart_window,
        }
    }
}

impl StatefulWidget for TrendComponent {
    type State = Vec::<(f64, f64)>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {

        // Update state
        let tick = state[state.len() - 1].0;
        if state.len() > self.chart_window {
            state.remove(0);
        }
        state.push((tick + 1_f64, self.trend_value));

        let database = vec![
            Dataset::default()
                .name(self.trend_name)
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(self.trend_color))
                .data(state),
        ];

        let y_center = (self.y_max - self.y_min) / 2.0;
        let y_1_4 = y_center / 2.0;
        let y_3_4 = y_center + y_1_4;

        let chart = Chart::new(database)
            .block(Block::bordered())
            .x_axis(
                Axis::default()
                    .title(self.x_title)
                    .style(Style::default().fg(Color::Gray))
                    .bounds([state[0].0, state[state.len() - 1].0])
            )
            .y_axis(
                Axis::default()
                    .title(self.y_title)
                    .style(Style::default().fg(Color::Gray))
                    .labels([self.y_min.to_string(), y_1_4.to_string(), y_center.to_string(), y_3_4.to_string(), self.y_max.to_string()])
                    .bounds([self.y_min, self.y_max]),
            );

        chart.render(area, buf);
    }
}