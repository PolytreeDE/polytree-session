use std::ffi::CString;
use std::thread;

fn main() {
	unsafe { unsafe_main() }
}

unsafe fn unsafe_main() {
	let wm_pid = libc::fork();

	if wm_pid == -1 {
		let msg = CString::new(b"polytree-session: WM fork" as &[u8]).unwrap();
		libc::perror(msg.as_ptr());
		libc::exit(libc::EXIT_FAILURE);
	}

	if wm_pid == 0 {
		let arg0 = CString::new(b"polytreewm" as &[u8]).unwrap();
		let args = vec![arg0.as_ptr(), std::ptr::null()];
		libc::execvp(arg0.as_ptr(), args.as_ptr());
		let msg = CString::new(b"polytree-session: WM exec" as &[u8]).unwrap();
		libc::perror(msg.as_ptr());
		libc::exit(libc::EXIT_FAILURE);
	}

	let status_thread = thread::spawn(move || {
		let pid = libc::fork();

		if pid == -1 { return };

		if pid == 0 {
			let arg0 = CString::new(b"slstatus" as &[u8]).unwrap();
			let args = vec![arg0.as_ptr(), std::ptr::null()];
			libc::execvp(arg0.as_ptr(), args.as_ptr());
			let msg = CString::new(b"polytree-session: slstatus exec" as &[u8])
				.unwrap();
			libc::perror(msg.as_ptr());
			libc::exit(libc::EXIT_FAILURE);
		}
	});

	let wm_status: i32 = 0;
	libc::waitpid(wm_pid, wm_status as *mut i32, 0);

	status_thread.join().unwrap();

	libc::exit(libc::WEXITSTATUS(wm_status));
}
