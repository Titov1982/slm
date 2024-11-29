
mod trend_component;
mod bar_component;
mod proc_table_component;
mod app;
mod ui;
mod process_object;

use std::error;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crate::app::App;
use crate::proc_table_component::SortTableParam;
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// todo - обработка командной строки
fn main() -> std::io::Result<()> {

    let mut terminal = ratatui::init();

    let mut app = App::new();
    let result = run(&mut terminal, &mut app);
    ratatui::restore();

    result
}

fn run(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> std::io::Result<()> {

    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if handle_events(app)? {
                break Ok(());
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update_state();
            last_tick = Instant::now();
        }
    }
}

fn handle_events(app: &mut App) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::F(10) => return Ok(true),
            KeyCode::Down => app.process_table_next_row(),
            KeyCode::Up => app.process_table_previous_row(),
            KeyCode::PageDown => app.process_table_pagedown_row(20),
            KeyCode::PageUp => app.process_table_pageup_row(20),
            KeyCode::F(9) => app.kill_selected_process_from_table(),

            KeyCode::Char('1') => {app.process_table_sort_active_function = app.process_table_sort_by_pid_function; app.process_table_sort_param = SortTableParam::Pid},
            KeyCode::Char('2') => { app.process_table_sort_active_function = app.process_table_sort_by_user_function; app.process_table_sort_param = SortTableParam::User},
            KeyCode::Char('3') => { app.process_table_sort_active_function = app.process_table_sort_by_cpu_function; app.process_table_sort_param = SortTableParam::Cpu},
            KeyCode::Char('4') => { app.process_table_sort_active_function = app.process_table_sort_by_mem_function; app.process_table_sort_param = SortTableParam::Mem},
            KeyCode::Char('5') => { app.process_table_sort_active_function = app.process_table_sort_by_time_function; app.process_table_sort_param = SortTableParam::Time},
            KeyCode::Char('6') => { app.process_table_sort_active_function = app.process_table_sort_by_name_function; app.process_table_sort_param = SortTableParam::Name},
            KeyCode::Char('7') => { app.process_table_sort_active_function = app.process_table_sort_by_command_function; app.process_table_sort_param = SortTableParam::Command},

            // handle other key events
            _ => {}
        },
        // Event::Mouse(mouse_event) => match mouse_event.kind {
        //     MouseEventKind::Down(MouseButton::Left) => {
        //         let selected_column = mouse_event.column;
        //         match selected_column {
        //             0 => app.process_table_sort_active_function = app.process_table_sort_by_pid,
        //             1 => app.process_table_sort_active_function = app.process_table_sort_by_cpu,
        //             2 => app.process_table_sort_active_function = app.process_table_sort_by_mem,
        //             3 => app.process_table_sort_active_function = app.process_table_sort_by_time,
        //             4 => app.process_table_sort_active_function = app.process_table_sort_by_name,
        //             5 => app.process_table_sort_active_function = app.process_table_sort_by_command,
        //
        //             _ => app.process_table_sort_active_function = app.process_table_sort_by_cpu,
        //         }
        //     }
        // // handle other events
        //     _ => {}
        // }
        _ => {}
    }
    Ok(false)
}