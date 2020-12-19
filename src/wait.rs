use crate::{c_error::CError, c_result::CResult};

pub fn wait() -> CResult<WaitResult> {
	let mut wstatus: libc::c_int = 0;
	match unsafe { libc::wait(&mut wstatus) } {
		-1 => Err(CError::new_from_errno()),
		pid => Ok(WaitResult::new(
			pid,
			wstatus,
		))
	}
}

pub fn waitpid(pid: libc::pid_t) -> CResult<WaitResult> {
	waitpid_with_options(pid, 0)
}

pub fn waitpid_with_options(pid: libc::pid_t, options: libc::c_int) -> CResult<WaitResult> {
	let mut wstatus: libc::c_int = 0;
	match unsafe { libc::waitpid(pid, &mut wstatus, options) } {
		-1 => Err(CError::new_from_errno()),
		pid => Ok(WaitResult::new(
			pid,
			wstatus,
		))
	}
}


pub struct WaitResult {
	pub pid: libc::pid_t,
	pub status: WaitStatus,
}

impl WaitResult {
	pub fn new(pid: libc::pid_t, wstatus: libc::c_int) -> Self {
		Self {
			pid,
			status: WaitStatus(wstatus),
		}
	}
}

pub struct WaitStatus(pub libc::c_int);

impl WaitStatus {
	// Corresponds to WIFEXITED
	pub fn exited_normally(&self) -> bool {
		self.terminating_signal() == 0
	}

	// Corresponds to WTERMSIG
	pub fn terminating_signal(&self) -> libc::c_int {
		self.0 & 0x7f
	}

	// Corresponds to WEXITSTATUS
	pub fn exit_status(&self) -> u8 {
		((self.0 & 0xff00) >> 8) as u8
	}

	// Corresponds to WSTOPSIG
	pub fn stopping_signal(&self) -> libc::c_int {
		self.terminating_signal()
	}

	// Corresponds to WIFSIGNALED
	pub fn terminated_by_signal(&self) -> bool {
		((self.terminating_signal() + 1) >> 1) > 0
	}

	pub fn is_stopped(&self) -> bool {
		(self.0 & 0xff) == 0x7f
	}

	pub fn dumped_core(&self) -> bool {
		let __wcoreflag = 0x80;
		(self.0 & __wcoreflag) != 0
	}
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
