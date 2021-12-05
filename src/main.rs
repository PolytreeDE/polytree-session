use std::ffi::CString;

fn main() {
    let wm_task = WMTask::start().unwrap();
    unsafe { libc::exit(wm_task.wait()) }
}

trait Task: Sized {
    fn start() -> Result<Self, String>;
    fn wait(self) -> i32;
}

struct WMTask {
    pid: libc::pid_t,
}

impl Task for WMTask {
    fn start() -> Result<Self, String> {
        unsafe {
            let pid = libc::fork();

            if pid == -1 {
                return Err("fork".into())
            }

            if pid == 0 {
                let arg0 = CString::new(b"polytreewm" as &[u8]).unwrap();
                let args = vec![arg0.as_ptr(), std::ptr::null()];
                libc::execvp(arg0.as_ptr(), args.as_ptr());
                libc::exit(libc::EXIT_FAILURE);
            }

            Ok(Self { pid })
        }
    }

    fn wait(self) -> i32 {
        unsafe {
            let status: i32 = 0;
            libc::waitpid(self.pid, status as *mut i32, 0);
            libc::WEXITSTATUS(status)
        }
    }
}
