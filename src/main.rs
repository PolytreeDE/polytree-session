mod task;

use crate::task::*;

use std::ffi::CString;

fn main() {
    let wm_task_config = TaskConfig::new("polytreewm");
    let wm_task = WMTask::start(wm_task_config).unwrap();
    let wm_task_result = wm_task.wait();
    let wm_task_status = wm_task_result.status();
    unsafe { libc::exit(wm_task_status) }
}

struct WMTask(TaskInfo);

impl Task for WMTask {
    fn start(config: TaskConfig) -> Result<Self, String> {
        unsafe {
            let pid = libc::fork();

            if pid == -1 {
                return Err("fork".into());
            }

            if pid == 0 {
                let arg0 = CString::new(config.exe().as_bytes()).unwrap();
                let args = vec![arg0.as_ptr(), std::ptr::null()];
                libc::execvp(arg0.as_ptr(), args.as_ptr());
                libc::exit(libc::EXIT_FAILURE);
            }

            Ok(Self(TaskInfo::new(config, pid)))
        }
    }

    fn wait(self) -> TaskResult {
        unsafe {
            let status: i32 = 0;
            libc::waitpid(self.0.pid(), status as *mut i32, 0);
            let status = libc::WEXITSTATUS(status);
            TaskResult::new(self.0, status)
        }
    }
}
