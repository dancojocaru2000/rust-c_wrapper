use crate::{c_error::CError, c_result::CResult};

pub fn wait() -> CResult<WaitResult> {
	let mut wstatus: libc::c_int = 0;
	match unsafe { libc::wait(&mut wstatus) } {
		-1 => Err(CError::new_from_errno()),
		pid => Ok(WaitResult {
			pid,
			wstatus
		})
	}
}

pub fn waitpid(pid: libc::pid_t) -> CResult<WaitResult> {
	waitpid_with_options(pid, 0)
}

pub fn waitpid_with_options(pid: libc::pid_t, options: libc::c_int) -> CResult<WaitResult> {
	let mut wstatus: libc::c_int = 0;
	match unsafe { libc::waitpid(pid, &mut wstatus, options) } {
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

pub mod options {
	pub use libc::{
		WNOHANG,
		WUNTRACED,
	};

	#[cfg(target_os = "linux")]
	pub use libc::{
		WCONTINUED,
	};
}
