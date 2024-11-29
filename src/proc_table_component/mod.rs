use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::style::Color::{Black, Cyan};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Cell, HighlightSpacing, Row, StatefulWidget, Table, TableState};
use crate::process_object::ProcessObject;

pub enum SortTableParam {
    Pid,
    User,
    Cpu,
    Mem,
    Time,
    Name,
    Command,
}

pub struct ProcTableComponent<'a> {
    process_table_items_vec: &'a Vec<ProcessObject>,
        process_table_sort_param: &'a SortTableParam,
}

impl<'a> ProcTableComponent<'a> {
    pub fn new(process_table_items_vec: &'a Vec<ProcessObject>, process_table_sort_param: &'a SortTableParam) -> Self {
        Self {
            process_table_items_vec,
            process_table_sort_param,
        }
    }
}

impl StatefulWidget for ProcTableComponent<'_> {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {

        let sort_param = match self.process_table_sort_param {
            SortTableParam::Pid => "PID",
            SortTableParam::User => "USER",
            SortTableParam::Cpu => "CPU%",
            SortTableParam::Mem => "MEM%",
            SortTableParam::Time => "TIME",
            SortTableParam::Name => "Name",
            SortTableParam::Command => "Command",
        };

        let header = ["PID", "USER", "CPU%", "MEM%", "TIME", "Name", "Command"]
            .into_iter()
            .map(|i| {
                if sort_param == i {
                    Cell::from(Line::from(i).alignment(Alignment::Left).bg(Cyan))
                } else {
                    Cell::from(Line::from(i).alignment(Alignment::Left))
                }
            })
            .collect::<Row>()
            .style(Style::new().fg(Color::Rgb(0, 0, 0)).bg(Color::Green))
            .height(1);

        let rows = self.process_table_items_vec.iter().map(|data| {
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("{}", content))))
                .collect::<Row>()
                .style(Style::new())
                .height(1)
        });

        let bar = " ► ";

        let t = Table::new(
            rows,
            [
                Constraint::Length(5),
                Constraint::Length(10),
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(12),
                Constraint::Length(25),
                Constraint::Fill(1),
            ],
        )
            .header(header)
            .row_highlight_style(Style::new().fg(Black).bg(Cyan))
            .highlight_symbol(Text::from(vec![
                bar.into(),
            ]))
            .highlight_spacing(HighlightSpacing::Always);

        t.render(area, buf, state);
    }
}