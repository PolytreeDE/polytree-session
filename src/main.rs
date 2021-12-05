mod task;

use crate::task::*;

use std::ffi::CString;

fn main() {
    let wm_task_config = TaskConfig::new("polytreewm");
    let wm_task = WMTask::start(wm_task_config).unwrap();
    unsafe { libc::exit(wm_task.wait().status()) }
}

struct WMTask {
    pid: libc::pid_t,
}

impl Task for WMTask {
    fn start(config: TaskConfig) -> Result<Self, String> {
        unsafe {
            let pid = libc::fork();

            if pid == -1 {
                return Err("fork".into())
            }

            if pid == 0 {
                let arg0 = CString::new(config.exe().as_bytes()).unwrap();
                let args = vec![arg0.as_ptr(), std::ptr::null()];
                libc::execvp(arg0.as_ptr(), args.as_ptr());
                libc::exit(libc::EXIT_FAILURE);
            }

            Ok(Self { pid })
        }
    }

    fn wait(self) -> TaskResult {
        unsafe {
            let status: i32 = 0;
            libc::waitpid(self.pid, status as *mut i32, 0);
            let status = libc::WEXITSTATUS(status);
            TaskResult::new(status)
        }
    }
}
