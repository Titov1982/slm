use std::cmp::Ordering;

#[derive(Clone)]
pub struct ProcessObject {
    pub pid: String,
    pub user: String,
    pub cpu: String,
    pub mem: String,
    pub time_sec: u64,
    pub time: String,
    pub name: String,
    pub command: String,
}

impl ProcessObject {
    pub fn new() -> Self {
        Self {
            pid: "".to_string(),
            user: "".to_string(),
            cpu: "".to_string(),
            mem: "".to_string(),
            time_sec: 0,
            time: "".to_string(),
            name: "".to_string(),
            command: "".to_string(),
        }
    }

    pub const fn ref_array(&self) -> [&String; 7] {
        [&self.pid, &self.user, &self.cpu, &self.mem, &self.time, &self.name, &self.command]
    }

    pub fn pid(&self) -> &str {
        &self.pid
    }
    pub fn user(&self) -> &str {
        &self.user
    }
    pub fn cpu(&self) -> &str {
        &self.cpu
    }
    pub fn mem(&self) -> &str {
        &self.mem
    }
    pub fn time(&self) -> &str {
        &self.time
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn command(&self) -> &str {
        &self.command
    }
}

impl PartialEq for ProcessObject {
    fn eq(&self, other: &Self) -> bool {
        self.pid.parse::<u32>() == other.pid.parse::<u32>()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for ProcessObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pid.partial_cmp(&other.pid)
    }

    fn lt(&self, other: &Self) -> bool {
        self.pid.parse::<u32>().unwrap() < other.pid.parse::<u32>().unwrap()
    }

    fn le(&self, other: &Self) -> bool {
        self.pid.parse::<u32>().unwrap() <= other.pid.parse::<u32>().unwrap()
    }

    fn gt(&self, other: &Self) -> bool {
        self.pid.parse::<u32>().unwrap() > other.pid.parse::<u32>().unwrap()
    }

    fn ge(&self, other: &Self) -> bool {
        self.pid.parse::<u32>().unwrap() >= other.pid.parse::<u32>().unwrap()
    }
}
impl Eq for ProcessObject{

}

impl Ord for ProcessObject {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pid.cmp(&other.pid)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized
    {
        if self.pid > other.pid {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized
    {
        if self.pid < other.pid {
            self
        } else {
            other
        }
    }

    fn clamp(self, _min: Self, _max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd
    {
        todo!()
    }
}
