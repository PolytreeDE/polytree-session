pub trait Task: Sized {
    fn start() -> Result<Self, String>;
    fn wait(self) -> TaskResult;
}

pub struct TaskResult {
    status: i32,
}

impl TaskResult {
    pub fn new(status: i32) -> Self {
        Self { status }
    }

    pub fn status(&self) -> i32 {
        self.status
    }
}
