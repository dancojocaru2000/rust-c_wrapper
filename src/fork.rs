use crate::{c_error::CError, c_result::CResult};

pub fn fork() -> CResult<ForkResult> {
	match unsafe { libc::fork() } {
		-1 => Err(CError::new_from_errno()),
		0 => Ok(ForkResult::Child),
		child_pid => Ok(ForkResult::Parent(child_pid)),
	}
}

pub enum ForkResult {
	Child,
	Parent(libc::pid_t),
}