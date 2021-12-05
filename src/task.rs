pub trait Task: Sized {
    fn start(config: TaskConfig) -> Result<Self, String>;
    fn wait(self) -> TaskResult;
}

pub struct TaskConfig {
    exe: String,
}

pub struct TaskResult {
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

impl TaskResult {
    pub fn new(status: i32) -> Self {
        Self { status }
    }

    pub fn status(&self) -> i32 {
        self.status
    }
}
