use std::cmp::Ordering;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use ratatui::widgets::{ScrollbarState, TableState};
use sysinfo::{Pid, Process, System, Users};
use crate::proc_table_component::SortTableParam;
use crate::process_object::ProcessObject;


pub struct App {
    pub tick_rate: u64,

    pub system_state: System,       // System state
    pub system_uptime: String,      // System uptime

    // Memory
    pub total_mem: f64,
    pub usage_mem: f64,
    pub total_mem_per: f64,
    pub usage_mem_per: f64,

    // Swap
    pub total_swap: f64,
    pub usage_swap: f64,
    pub total_swap_per: f64,
    pub usage_swap_per: f64,

    // CPU usage
    cpu_usage_vec: Vec<f32>,
    pub cpu_usage_human: f32,

    // Trends state
    pub cpu_usage_trend_vec: Vec<(f64, f64)>,
    pub mem_usage_trend_vec: Vec<(f64, f64)>,

    // Clock
    pub clock: DateTime<Local>,

    // Process table state
    pub process_table_items_vec: Vec<ProcessObject>,
    pub process_table_state: TableState,
    pub process_table_scroll_state: ScrollbarState,

    // Process table sort
    pub process_table_sort_by_user_function: Option<fn(&ProcessObject, &ProcessObject) -> Ordering>,
    pub process_table_sort_by_pid_function: Option<fn(&ProcessObject, &ProcessObject) -> Ordering>,
    pub process_table_sort_by_cpu_function: Option<fn(&ProcessObject, &ProcessObject) -> Ordering>,
    pub process_table_sort_by_mem_function: Option<fn(&ProcessObject, &ProcessObject) -> Ordering>,
    pub process_table_sort_by_time_function: Option<fn(&ProcessObject, &ProcessObject) -> Ordering>,
    pub process_table_sort_by_name_function: Option<fn(&ProcessObject, &ProcessObject) -> Ordering>,
    pub process_table_sort_by_command_function: Option<fn(&ProcessObject, &ProcessObject) -> Ordering>,

    pub process_table_sort_active_function: Option<fn(&ProcessObject, &ProcessObject) -> Ordering>,
    pub process_table_sort_param: SortTableParam,

    // Info string
    pub info_string: String,
}

impl App {
    pub fn new(daemon_on: bool, csv_data_file_path: String, tick_rate: u64) -> App {

        App {
            tick_rate,

            system_state: System::new(),
            system_uptime: String::new(),

            total_mem: 0.0,
            usage_mem: 0.0,
            total_mem_per: 0.0,
            usage_mem_per: 0.0,

            total_swap: 0.0,
            usage_swap: 0.0,
            total_swap_per: 0.0,
            usage_swap_per: 0.0,

            cpu_usage_vec: vec![0.0, 0.0, 0.0],
            cpu_usage_human: 0.0,

            cpu_usage_trend_vec: match daemon_on {
                false => Vec::from([(0.0, System::new().global_cpu_usage() as f64)]),
                true => App::load_data_from_csv(&csv_data_file_path).0,
            },

            mem_usage_trend_vec: match daemon_on {
                false => Vec::from([(0.0, System::new().used_memory() as f64)]),
                true => App::load_data_from_csv(&csv_data_file_path).1,
            },

            clock: Local::now(),

            process_table_items_vec: Vec::new(),
            process_table_state: TableState::default().with_selected(1),
            process_table_scroll_state: ScrollbarState::new(1),

            process_table_sort_by_pid_function: Some(|a: &ProcessObject, b: &ProcessObject| a.pid.parse::<u32>().unwrap().partial_cmp(&b.pid.parse::<u32>().unwrap()).unwrap()),
            process_table_sort_by_user_function: Some(|a: &ProcessObject, b: &ProcessObject| a.user().partial_cmp(b.user()).unwrap()),
            process_table_sort_by_cpu_function: Some(|a: &ProcessObject, b: &ProcessObject| b.cpu.parse::<f32>().unwrap().partial_cmp(&a.cpu.parse::<f32>().unwrap()).unwrap()),
            process_table_sort_by_mem_function: Some(|a: &ProcessObject, b: &ProcessObject| b.mem.parse::<f64>().unwrap().partial_cmp(&a.mem.parse::<f64>().unwrap()).unwrap()),
            process_table_sort_by_time_function: Some(|a: &ProcessObject, b: &ProcessObject| a.time_sec.partial_cmp(&b.time_sec).unwrap()),
            process_table_sort_by_name_function: Some(|a: &ProcessObject, b: &ProcessObject| a.name().partial_cmp(b.name()).unwrap()),
            process_table_sort_by_command_function: Some(|a: &ProcessObject, b: &ProcessObject| a.command().partial_cmp(b.command()).unwrap()),

            process_table_sort_active_function: Some(|a: &ProcessObject, b: &ProcessObject| b.cpu.parse::<f32>().unwrap().partial_cmp(&a.cpu.parse::<f32>().unwrap()).unwrap()),
            process_table_sort_param: crate::proc_table_component::SortTableParam::Cpu,

            info_string: "Down/PageDown/Up/PageUp; Sort: 1 - Pid, 2 - User, 3 - Cpu, 4 - Mem, 5 - Time, 6 - Name, 7 - Command; \
            F9 - kill selected process; q / F10 - for quit".to_string(),
        }
    }

    pub fn load_data_from_csv(csv_data_file_path: &String) -> (Vec::<(f64, f64)>, Vec::<(f64, f64)>) {

        let mut cpu_data = Vec::<(f64, f64)>::new();
        let mut mem_data = Vec::<(f64, f64)>::new();

        let reader = csv::Reader::from_path(csv_data_file_path);
        for record in reader.unwrap().records() {
            let record = record.unwrap();
            let mem_used_in_bytes =  (&record[2].to_string()).parse::<u64>().unwrap();
            let mut sysinfo = System::new();
            sysinfo.refresh_all();
            let mem_used_per = (mem_used_in_bytes as f64 / sysinfo.total_memory() as f64) * 100.0;
            cpu_data.push(((&record[0].to_string()).parse::<f64>().unwrap(), (&record[1].to_string()).parse::<f64>().unwrap()));
            mem_data.push(((&record[0].to_string()).parse::<f64>().unwrap(), mem_used_per));
        }
        (cpu_data, mem_data)
    }

    pub fn update_state(&mut self) {
        self.system_state.refresh_all();                            // Refresh state
        self.system_uptime = self.uptime_calc(System::uptime());    // Calc uptime

        // RAM calc
        (self.total_mem, self.usage_mem, self.total_mem_per, self.usage_mem_per) =
            self.mem_calc(self.system_state.total_memory(), self.system_state.used_memory());

        self.cpu_usage_human = self.usage_calc();                   // Usage calc

        // Usage swap
        (self.total_swap, self.usage_swap, self.total_swap_per, self.usage_swap_per) =
            self.mem_calc(self.system_state.total_swap(), self.system_state.used_swap());

        self.clock = self.clock_update();                           // Clock update

        // Process table update
        self.system_state.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        self.process_table_items_vec = self.convert_processes_to_table_items(self.system_state.processes(), self.process_table_sort_active_function);
    }

    fn convert_processes_to_table_items<F>(&self, processes: &HashMap<Pid, Process>, sort_fn: Option<F>) -> Vec<ProcessObject>
    where F: FnMut(&ProcessObject, &ProcessObject) -> Ordering {

        let mut process_object_vec = Vec::<ProcessObject>::new();
        let users = Users::new_with_refreshed_list();
        for (pid, process) in processes {
            let p_obj = ProcessObject {
                pid: pid.to_string(),
                user: match process.user_id() {
                    None => "".to_string(),
                    Some(user_id) => match users.get_user_by_id(user_id) {
                        None => "root".to_string(),
                        Some(user) => user.name().to_string(),
                    }
                },
                cpu: process.cpu_usage().to_string(),
                mem: ((process.memory() as f64 / self.system_state.total_memory() as f64) * 100.0).to_string(),
                time_sec: process.run_time(),
                time: self.proc_time_calc(process.run_time()),
                name: match process.name().to_str() {
                    Some(name) => name.to_string(),
                    None => "".to_string(),
                },
                command: match process.exe() {
                    Some(exe) => exe.to_str().unwrap().to_string(),
                    None => "".to_string(),
                },
            };
            process_object_vec.push(p_obj);
        }

        match sort_fn {
            None => {},
            Some(sort_fn) => {
                process_object_vec.sort_by(sort_fn);
            }
        }
        process_object_vec
    }

    fn mem_calc(&self, total_mem_in_byte: u64, used_mem_in_byte: u64) -> (f64, f64, f64, f64) {
        (
            total_mem_in_byte as f64 / (1024.0 * 1024.0 * 1024.0),
            used_mem_in_byte as f64 / (1024.0 * 1024.0 * 1024.0),
            100.0,
            (used_mem_in_byte as f64 / total_mem_in_byte as f64) * 100.0,
        )
    }

    fn usage_calc(&mut self) -> f32 {
        self.cpu_usage_vec.remove(0);
        self.cpu_usage_vec.push(self.system_state.global_cpu_usage());
        let sum: f32 = self.cpu_usage_vec.iter().sum();
        let avg = sum / self.cpu_usage_vec.len() as f32;
        avg
    }

    fn time_to_human_str(&self, system_uptime: u64) -> (u64, u64, u64, u64) {
        let day = system_uptime / 86400;            // 1 day = 86400 sec
        let mut sec = system_uptime % 86400;
        let hour = sec / 3600;                      // 1 hour = 3600 sec
        sec %=  3600;
        let min = sec / 60;                         // 1 min = 60 sec
        sec %=  60;

        (day, hour, min, sec)
    }

    fn uptime_calc(&self, system_uptime: u64) -> String {
        let (day, hour, min, sec) = self.time_to_human_str(system_uptime);
        format!("{day} days, {hour}:{min}:{sec}")
    }

    fn proc_time_calc(&self, system_uptime: u64) -> String {
        let (day, hour, min, sec) = self.time_to_human_str(system_uptime);
        let hour = day * 24 + hour;
        format!("{hour}:{min}:{sec}")
    }

    fn clock_update(&mut self) -> DateTime<Local>{
        Local::now()
    }

    pub fn process_table_next_row(&mut self) {
        let i = match self.process_table_state.selected() {
            Some(i) => {
                if i >= self.process_table_items_vec.len() - 1 { i } else { i + 1 }
            }
            None => 0,
        };
        self.process_table_state.select(Some(i));
        self.process_table_scroll_state = self.process_table_scroll_state.position(i);
    }

    pub fn process_table_previous_row(&mut self) {
        let i = match self.process_table_state.selected() {
            Some(i) => {
                if i == 0 { 0 } else { i - 1 }
            }
            None => 0,
        };
        self.process_table_state.select(Some(i));
        self.process_table_scroll_state = self.process_table_scroll_state.position(i);
    }

    pub fn process_table_pagedown_row(&mut self, row_count: usize) {
        let i = match self.process_table_state.selected() {
            Some(i) => {
                if i >= self.process_table_items_vec.len() - 1 { i }
                else if i + row_count >= self.process_table_items_vec.len() - 1 { self.process_table_items_vec.len() - 1 }
                else { i + row_count }
            }
            None => 0,
        };
        self.process_table_state.select(Some(i));
        self.process_table_scroll_state = self.process_table_scroll_state.position(i);
    }

    pub fn process_table_pageup_row(&mut self, row_count: usize) {
        let i = match self.process_table_state.selected() {
            Some(i) => {
                if i == 0 { 0 }
                else if i as isize - row_count as isize <= 0 { 0 }
                else { i - row_count }
            }
            None => 0,
        };
        self.process_table_state.select(Some(i));
        self.process_table_scroll_state = self.process_table_scroll_state.position(i);
    }

    pub fn kill_selected_process_from_table(&mut self) {
        let selected_item_num = self.process_table_state.selected().unwrap();
        let selected_item_pid = self.process_table_items_vec[selected_item_num].pid.parse::<u32>().unwrap();
        self.system_state.process(Pid::from_u32(selected_item_pid)).unwrap().kill();
        self.update_state();
    }
}