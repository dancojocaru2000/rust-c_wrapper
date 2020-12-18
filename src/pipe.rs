use crate::{c_result::CResult, file::FileDescriptor};
use crate::c_error::CError;

pub fn pipe() -> CResult<PipeResult> {
	let mut pipes_fd = [-1, -1];
	match unsafe { libc::pipe(pipes_fd.as_mut_ptr()) } {
		-1 => Err(CError::new_from_errno()),
		_ => Ok(unsafe { PipeResult::new_from_slice(&pipes_fd) })
	}
}

#[cfg(target_os = "linux")]
pub fn pipe_with_flags(flags: libc::c_int) -> CResult<PipeResult> {
	let mut pipes_fd = [-1, -1];
	match unsafe { libc::pipe2(pipes_fd.as_mut_ptr(), flags) } {
		-1 => Err(CError::new_from_errno()),
		_ => Ok(unsafe { PipeResult::new_from_slice(&pipes_fd) })
	}
}

pub struct PipeResult {
	pub read_fd: FileDescriptor,
	pub write_fd: FileDescriptor
}

impl PipeResult {
	pub(crate) unsafe fn new_from_slice(slice: &[libc::c_int]) -> Self {
		assert!(slice.len() == 2);
		Self::from_unowned(slice[0], slice[1])
	}

	pub unsafe fn from_unowned(read_fd: libc::c_int, write_fd: libc::c_int) -> Self {
		Self {
			read_fd: FileDescriptor::from_unowned(read_fd),
			write_fd: FileDescriptor::from_unowned(write_fd)
		}
	}	

	pub unsafe fn to_unowned(self) -> (libc::c_int, libc::c_int) {
		(self.read_fd.to_unowned(), self.write_fd.to_unowned())
	}

	pub fn drop_read(self) -> FileDescriptor {
		self.write_fd
	}

	pub fn drop_write(self) -> FileDescriptor {
		self.read_fd
	}
}

pub mod flags {
	pub use libc::{
		O_CLOEXEC,
		O_NONBLOCK,
	};

	#[cfg(target_os = "linux")]
	pub use libc::{
		O_DIRECT,
	};
}
