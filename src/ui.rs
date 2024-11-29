use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::style::palette::material::DEEP_ORANGE;
use ratatui::widgets::{Block, Padding, Paragraph};
use sysinfo::System;

use crate::app::App;
use crate::bar_component::BarComponent;
use crate::proc_table_component::ProcTableComponent;
use crate::trend_component::TrendComponent;

// todo - перенести виджеты в отдельную папку
pub(crate) fn draw(frame: &mut Frame, app: &mut App) {

    // --------------- Components --------------- //

    let title_string = Paragraph::new(
        "SYSTEM LOAD MANAGER")
        .alignment(Alignment::Center)
        .bg(Color::Cyan)
        .fg(Color::Rgb(0, 0, 0))
        .add_modifier(Modifier::BOLD);

    let system_info_left = Paragraph::new(
        format!("CPU arch: {}\nHost name: {}\nKernel version: {}\nOS version: {}",
                System::cpu_arch().unwrap(), System::host_name().unwrap(),
                System::kernel_version().unwrap(), System::long_os_version().unwrap()))
        .alignment(Alignment::Left)
        .fg(Color::Yellow)
        .block(Block::new().padding(Padding::new(1, 0 ,1, 1)));

    let system_info_center = Paragraph::new(
        format!("Load CPU: {:.2}\nSystem uptime: {}\nLoad average: {:.2} {:.2} {:.2}\nMemory: {:.6}GB / {}GB",
                app.cpu_usage_human, app.system_uptime,
                System::load_average().one, System::load_average().five,
                System::load_average().fifteen, app.usage_mem, app.total_mem))
        .alignment(Alignment::Left)
        .fg(Color::Yellow)
        .block(Block::new().padding(Padding::new(1, 0 ,1, 1)));

    let system_info_right = Paragraph::new(
        format!("Swap: {:.6}GB / {:.4}GB",
                app.usage_swap, app.total_swap))
        .alignment(Alignment::Left)
        .fg(Color::Yellow)
        .block(Block::new().padding(Padding::new(1, 0 ,1, 1)));

    // Create CPU vertical widget
    let cpu_bar = BarComponent::new(" CPU ".to_string(),
                                    app.cpu_usage_human, 100.0,
                                    50.0, 80.0, "".to_string(),
                                    Direction::Vertical, Alignment::Center);

    // Create MEM widget
    let mem_bar = BarComponent::new(" MEM ".to_string(),
                                    app.usage_mem_per as f32, 100.0,
                                    70.0, 90.0, "".to_string(),
                                    Direction::Vertical, Alignment::Center);

    // Create SWAP widget
    let swap_bar = BarComponent::new(" SWAP ".to_string(),
                                     app.usage_swap_per as f32, app.total_swap_per,
                                     40.0, 70.0, "".to_string(),
                                     Direction::Vertical, Alignment::Center);

    // Create info string
    let info_string = Paragraph::new(
        app.info_string.to_owned())
        .alignment(Alignment::Center)
        .bg(Color::Cyan)
        .fg(Color::Rgb(0, 0, 0))
        .add_modifier(Modifier::BOLD);

    // Create CPU usage trend
    let cpu_usage_trend = TrendComponent::new("CPU usage".to_string(),
                                              Color::Cyan, 100.0, 0.0, "%".to_string(),
                                              "tick".to_string(), 500, app.cpu_usage_human as f64);

    // Create MEM usage trend
    let mem_usage_trend = TrendComponent::new("MEM usage".to_string(),
                                              DEEP_ORANGE.a200, 100.0, 0.0, "%".to_string(),
                                              "tick".to_string(), 500, app.usage_mem_per as f64);

    // Create clock
    let clock_string = Paragraph::new(
        format!("{}", app.clock.format("%H:%M:%S").to_string()))
        .alignment(Alignment::Left)
        .bg(Color::Cyan)
        .fg(Color::Rgb(0, 0, 0))
        .add_modifier(Modifier::BOLD);

    // Create process table
    let proc_table = ProcTableComponent::new(&app.process_table_items_vec, &app.process_table_sort_param);


    // --------------- Layouts --------------- //

    // area layout
    let [title_layout, system_info_layout, data_layout, bottom_layout] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(6),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
        .areas(frame.area());

    // system_info_layout
    let [system_info_left_layout, system_info_center_layout, system_info_right_layout] = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
        .spacing(1)
        .areas(system_info_layout);

    // data_layout
    let [cpu_usage_bar_layout, mem_usage_bar_layout, swap_usage_bar_layout, data_right_layout] = Layout::horizontal([
        Constraint::Length(14),
        Constraint::Length(14),
        Constraint::Length(14),
        Constraint::Percentage(100),
    ])
        .spacing(1)
        .areas(data_layout);

    // data_right_layout
    let [top_data_right_layout, proc_table_layout] = Layout::vertical([
        Constraint::Percentage(33),
        Constraint::Fill(1),
    ])
        .areas(data_right_layout);

    // top_data_right_layout
    let [cpu_usage_trend_layout, mem_usage_trend_layout] = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
        .areas(top_data_right_layout);

    // bottom_layout
    let [info_bottom_layout, clock_bottom_layout] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(10),
    ])
        .areas(bottom_layout);




    // --------------- Rendering --------------- //

    // Render system info
    frame.render_widget(system_info_left, system_info_left_layout);
    frame.render_widget(system_info_center, system_info_center_layout);
    frame.render_widget(system_info_right, system_info_right_layout);

    // Render CPU usage bar
    frame.render_widget(cpu_bar, cpu_usage_bar_layout);

    // Render MEM usage bar
    frame.render_widget(mem_bar, mem_usage_bar_layout);

    // Render SWAP usage bar
    // frame.render_widget(swap_bar_hor, swap_usage_bar_layout_hor);
    frame.render_widget(swap_bar, swap_usage_bar_layout);

    // Render title string
    frame.render_widget(title_string, title_layout);

    // Render CPU usage trend
    frame.render_stateful_widget(cpu_usage_trend, cpu_usage_trend_layout, &mut app.cpu_usage_trend_vec);
    // Render MEM usage trend
    frame.render_stateful_widget(mem_usage_trend, mem_usage_trend_layout, &mut app.mem_usage_trend_vec);

    // Render bottom info
    frame.render_widget(info_string, info_bottom_layout);
    // Render clock
    frame.render_widget(clock_string, clock_bottom_layout);

    // Render process table
    frame.render_stateful_widget(proc_table, proc_table_layout, &mut app.process_table_state);
}