use std::ffi::CString;

use libc::mode_t;

use crate::{c_result::CResult, c_error::CError};

use super::FileDescriptor;

pub fn chmod<Path: Into<CString>>(pathname: Path, mode: mode_t) -> CResult<()> {
	let pathname: CString = pathname.into();
	match unsafe { libc::chmod(pathname.as_ptr(), mode) } {
		0 => Ok(()),
		-1 => Err(CError::new_from_errno()),
		bad_return => panic!(format!(
			"chmod returned {}, which is different from 0 or -1",
			bad_return
		)),
	}
}

pub fn fchmod(fd: &mut FileDescriptor, mode: mode_t) -> CResult<()> {
	match unsafe { libc::fchmod(fd.fd, mode) } {
		0 => Ok(()),
		-1 => Err(CError::new_from_errno()),
		bad_return => panic!(format!(
			"fchmod returned {}, which is different from 0 or -1",
			bad_return
		)),
	}
}

pub fn chmod_fd(fd: &mut FileDescriptor, mode: mode_t) -> CResult<()> {
	fchmod(fd, mode)	
}

pub fn fchmod_at(dir: &FileDescriptor, fd: &mut FileDescriptor, mode: mode_t) -> CResult<()> {
	match unsafe { libc::fchmodat(dir.fd, fd.fd, mode) } {
		0 => Ok(()),
		-1 => Err(CError::new_from_errno()),
		bad_return => panic!(format!(
			"fchmodat returned {}, which is different from 0 or -1",
			bad_return
		)),
	}
}

pub fn chmod_fd_at(dir: &FileDescriptor, fd: &mut FileDescriptor, mode: mode_t) -> CResult<()> {
	fchmod_at(dir, fd, mode)
}
