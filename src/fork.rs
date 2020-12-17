use crate::c_error::CError;

pub unsafe fn fork() -> ForkResult {
	match libc::fork() {
		-1 => ForkResult::Error(CError::from(errno::errno().0)),
		0 => ForkResult::Child,
		child_pid => ForkResult::Parent(child_pid)
	}
}

pub enum ForkResult {
	Child,
	Parent(libc::pid_t),
	Error(CError)
}