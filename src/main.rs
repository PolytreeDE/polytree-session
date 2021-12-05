mod task;

use crate::task::*;

fn main() {
    let wm_task = WMTask::start(TaskConfig::new("polytreewm")).unwrap();
    let comp_task = CompTask::start(TaskConfig::new("picom")).unwrap();

    let wm_task_result = wm_task.wait();

    comp_task.terminate();

    unsafe { libc::exit(wm_task_result.status()) }
}

#[derive(Debug)]
struct WMTask(TaskInfo);

#[derive(Debug)]
struct CompTask(TaskInfo);

impl Task for WMTask {
    fn new(info: TaskInfo) -> Self {
        Self(info)
    }

    fn info(&self) -> &TaskInfo {
        &self.0
    }
}

impl Task for CompTask {
    fn new(info: TaskInfo) -> Self {
        Self(info)
    }

    fn info(&self) -> &TaskInfo {
        &self.0
    }
}
