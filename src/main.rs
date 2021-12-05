mod task;

use crate::task::*;

fn main() {
    let wm_task_config = TaskConfig::new("polytreewm");
    let wm_task = WMTask::start(wm_task_config).unwrap();
    let wm_task_result = wm_task.wait();
    let wm_task_status = wm_task_result.status();
    unsafe { libc::exit(wm_task_status) }
}

#[derive(Debug)]
struct WMTask(TaskInfo);

impl Task for WMTask {
    fn new(info: TaskInfo) -> Self {
        Self(info)
    }

    fn info(&self) -> &TaskInfo {
        &self.0
    }
}
