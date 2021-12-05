use std::ffi::CString;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct TaskConfig {
    exe: String,
}

#[derive(Clone, Debug)]
pub struct TaskInfo {
    config: TaskConfig,
    pid: libc::pid_t,
}

#[derive(Clone, Debug)]
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

pub trait Task: Debug + Sized {
    fn new(info: TaskInfo) -> Self;
    fn info(&self) -> &TaskInfo;

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

            Ok(Self::new(TaskInfo::new(config, pid)))
        }
    }

    fn wait(self) -> TaskResult {
        unsafe {
            let status: i32 = 0;
            libc::waitpid(self.info().pid(), status as *mut i32, 0);
            let status = libc::WEXITSTATUS(status);
            TaskResult::new(self.info().clone(), status)
        }
    }

    fn terminate(self) -> TaskResult {
        unsafe {
            libc::kill(self.info().pid(), libc::SIGKILL);
            let status: i32 = 0;
            libc::waitpid(self.info().pid(), status as *mut i32, 0);
            let status = libc::WEXITSTATUS(status);
            TaskResult::new(self.info().clone(), status)
        }
    }
}
