use std::ffi::CString;

use libc::{c_int, mode_t};

use crate::{c_error::CError, c_result::CResult};

use super::descriptor::FileDescriptor;

pub fn open<Path: Into<CString>>(pathname: Path) -> CResult<FileDescriptor> {
	open_with_flags(pathname, 0)
}

pub fn open_with_flags<Path: Into<CString>>(pathname: Path, flags: c_int) -> CResult<FileDescriptor> {
	let pathname: CString = pathname.into();
	match unsafe { libc::open(pathname.as_c_str().as_ptr(), flags) } {
		-1 => Err(CError::new_from_errno()),
		fd => Ok(unsafe { FileDescriptor::from_unowned(fd) }),
	}
}

pub fn open_with_mode<Path: Into<CString>>(pathname: Path, flags: c_int, mode: mode_t) -> CResult<FileDescriptor> {
	let pathname: CString = pathname.into();
	match unsafe { libc::open(pathname.as_ptr(), flags, mode) } {
		-1 => Err(CError::new_from_errno()),
		fd => Ok(unsafe { FileDescriptor::from_unowned(fd) }),
	}
}

pub fn create<Path: Into<CString>>(pathname: Path, mode: mode_t) -> CResult<FileDescriptor> {
	creat(pathname, mode)
}

pub fn creat<Path: Into<CString>>(pathname: Path, mode: mode_t) -> CResult<FileDescriptor> {
	let pathname: CString = pathname.into();
	match unsafe { libc::creat(pathname.as_ptr(), mode) } {
		-1 => Err(CError::new_from_errno()),
		fd => Ok(unsafe { FileDescriptor::from_unowned(fd) }),
	}
}

pub fn openat<Path: Into<CString>>(dir: &FileDescriptor, pathname: Path) -> CResult<FileDescriptor> {
	openat_with_flags(dir, pathname, 0)
}

pub fn openat_with_flags<Path: Into<CString>>(dir: &FileDescriptor, pathname: Path, flags: c_int) -> CResult<FileDescriptor> {
	let pathname: CString = pathname.into();
	match unsafe { libc::openat(dir.fd, pathname.as_ptr(), flags) } {
		-1 => Err(CError::new_from_errno()),
		fd => Ok(unsafe { FileDescriptor::from_unowned(fd) }),
	}
}

pub fn openat_with_mode<Path: Into<CString>>(dir: &FileDescriptor, pathname: Path, flags: c_int, mode: mode_t) -> CResult<FileDescriptor> {
	let pathname: CString = pathname.into();
	match unsafe { libc::openat(dir.fd, pathname.as_ptr(), flags, mode) } {
		-1 => Err(CError::new_from_errno()),
		fd => Ok(unsafe { FileDescriptor::from_unowned(fd) }),
	}
}

pub mod flags {
	pub use libc::{
		O_APPEND,
		O_ASYNC,
		O_CREAT,
		O_DIRECT,
		O_DIRECTORY,
		O_DSYNC,
		O_EXCL,
		O_LARGEFILE,
		O_NOCTTY,
		O_NOFOLLOW,
		O_NONBLOCK,
		O_NDELAY,
		O_SYNC,
		O_TRUNC,
	};

	#[cfg(target_os = "linux")]
	pub use libc::{
		O_CLOEXEC,
		O_NOATIME,
		O_PATH,
		O_TMPFILE,
	};
}
