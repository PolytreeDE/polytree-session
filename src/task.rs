pub trait Task: Sized {
    fn start(config: TaskConfig) -> Result<Self, String>;
    fn wait(self) -> TaskResult;
}

pub struct TaskConfig {
    exe: String,
}

pub struct TaskInfo {
    config: TaskConfig,
    pid: libc::pid_t,
}

pub struct TaskResult {
    info: TaskInfo,
    status: i32,
}

impl TaskConfig {
    pub fn new<Exe: Into<String>>(exe: Exe) -> Self {
        Self { exe: exe.into() }
    }

    pub fn exe(&self) -> &String {
        &self.exe
    }
}

impl TaskInfo {
    pub fn new(config: TaskConfig, pid: libc::pid_t) -> Self {
        Self { config, pid }
    }

    #[allow(dead_code)]
    pub fn config(&self) -> &TaskConfig {
        &self.config
    }

    pub fn pid(&self) -> libc::pid_t {
        self.pid
    }
}

impl TaskResult {
    pub fn new(info: TaskInfo, status: i32) -> Self {
        Self { info, status }
    }

    #[allow(dead_code)]
    pub fn info(&self) -> &TaskInfo {
        &self.info
    }

    pub fn status(&self) -> i32 {
        self.status
    }
}
