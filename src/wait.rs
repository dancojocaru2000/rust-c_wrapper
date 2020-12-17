use crate::{c_error::CError, c_result::CResult};

pub unsafe fn wait() -> CResult<WaitResult> {
	let mut wstatus: libc::c_int = 0;
	match libc::wait(&mut wstatus) {
		-1 => Err(CError::new_from_errno()),
		pid => Ok(WaitResult {
			pid,
			wstatus
		})
	}
}

pub unsafe fn waitpid(pid: libc::pid_t) -> CResult<WaitResult> {
	waitpid_with_options(pid, 0)
}

pub unsafe fn waitpid_with_options(pid: libc::pid_t, options: libc::c_int) -> CResult<WaitResult> {
	let mut wstatus: libc::c_int = 0;
	match libc::waitpid(pid, &mut wstatus, options) {
		-1 => Err(CError::new_from_errno()),
		pid => Ok(WaitResult {
			pid,
			wstatus
		})
	}
}


pub struct WaitResult {
	pub pid: libc::pid_t,
	pub wstatus: libc::c_int,
}