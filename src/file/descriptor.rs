use std::{io::{Error, ErrorKind}, mem::size_of};

use crate::c_result::CResult;
use crate::c_error::CError;

pub struct FileDescriptor {
	pub(crate) fd: libc::c_int
}

// stdin, stdout, stderr
impl FileDescriptor {
	pub unsafe fn from_stdin() -> Self {
		Self::from_unowned(constants::STDIN_FILENO)
	}

	pub unsafe fn from_stdout() -> Self {
		Self::from_unowned(constants::STDIN_FILENO)
	}

	pub unsafe fn from_stderr() -> Self {
		Self::from_unowned(constants::STDIN_FILENO)
	}

	pub fn wrap_stdin<CB: FnOnce(&mut FileDescriptor) -> T, T>(callback: CB) -> T {
		Self::wrap_unowned(constants::STDIN_FILENO, callback)
	}

	pub fn wrap_stdout<CB: FnOnce(&mut FileDescriptor) -> T, T>(callback: CB) -> T {
		Self::wrap_unowned(constants::STDOUT_FILENO, callback)
	}

	pub fn wrap_stderr<CB: FnOnce(&mut FileDescriptor) -> T, T>(callback: CB) -> T {
		Self::wrap_unowned(constants::STDERR_FILENO, callback)
	}

	pub fn try_clone_stdin() -> CResult<Self> {
		Self::wrap_unowned(constants::STDIN_FILENO, |fd| {
			fd.try_clone()
		})
	}

	pub fn try_clone_stdout() -> CResult<Self> {
		Self::wrap_unowned(constants::STDOUT_FILENO, |fd| {
			fd.try_clone()
		})
	}

	pub fn try_clone_stderr() -> CResult<Self> {
		Self::wrap_unowned(constants::STDERR_FILENO, |fd| {
			fd.try_clone()
		})
	}
}

impl std::io::Read for FileDescriptor {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match unsafe { libc::read(self.fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) } {
			-1 => Err(match CError::new_from_errno() {
				CError::Again | CError::WouldBlock => Error::from(ErrorKind::WouldBlock),
				CError::Interrupted => Error::from(ErrorKind::Interrupted),
				CError::Fault => Error::from(ErrorKind::PermissionDenied),
				err => Error::from_raw_os_error(err.into())
			}),
			n => Ok(n as usize)
		}
    }
}

impl std::io::Write for FileDescriptor {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match unsafe { libc::write(self.fd, buf.as_ptr() as *const libc::c_void, buf.len()) } {
			-1 => Err(match CError::new_from_errno() {
				CError::Again | CError::WouldBlock => Error::from(ErrorKind::WouldBlock),
				CError::DestinationAddressRequired => Error::from(ErrorKind::NotConnected),
				CError::Fault | CError::Perm => Error::from(ErrorKind::PermissionDenied),
				CError::Interrupted => Error::from(ErrorKind::Interrupted),
				CError::BrokenPipe => Error::from(ErrorKind::BrokenPipe),
				err => Error::from_raw_os_error(err.into())
			}),
			n => Ok(n as usize)
		}
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl FileDescriptor {
	pub fn redirect_from(&mut self, other_fd: &FileDescriptor) -> CResult<()> {
		match unsafe { libc::dup2(other_fd.fd, self.fd) } {
			-1 => Err(CError::new_from_errno()),
			fd if fd == self.fd => Ok(()),
			bad_fd => panic!(format!(
				"dup2 returned {}, which is different from -1 or fd: {}",
				bad_fd,
				self.fd
			))
		}
	}
}

// unsafe stuff
impl FileDescriptor {
	pub unsafe fn from_unowned(fd: libc::c_int) -> Self {
		Self {
			fd
		}
	}

	pub unsafe fn to_unowned(mut self) -> libc::c_int {
		let result = self.fd;
		self.fd = -1;
		result
	}

	pub unsafe fn get_fd(&mut self) -> libc::c_int {
		self.fd
	}

	pub fn wrap_unowned<CB, T>(fd: libc::c_int, callback: CB) -> T where CB: FnOnce(&mut FileDescriptor) -> T {
		let mut fd = unsafe { FileDescriptor::from_unowned(fd) };
		let result = callback(&mut fd);
		unsafe { fd.to_unowned() };
		result
	}

	pub fn set_nonblocking(&mut self, nonblocking: bool) -> CResult<()> {
		let previous = self.fcntl(libc::F_GETFL)?;
		let current = if nonblocking {
			previous | libc::O_NONBLOCK
		}
		else {
			previous & !libc::O_NONBLOCK
		};
		if current != previous {
			self.fcntl_with_arg(libc::F_SETFL, current)?;
		}
		Ok(())
	}

	pub fn fcntl(&self, command: libc::c_int) -> CResult<libc::c_int> {
		match unsafe { libc::fcntl(self.fd, command) } {
			-1 => Err(CError::new_from_errno()),
			any => Ok(any),
		}
	}

	pub fn fcntl_with_arg<Arg>(&self, command: libc::c_int, argument: Arg) -> CResult<libc::c_int> {
		match unsafe { libc::fcntl(self.fd, command, argument) } {
			-1 => Err(CError::new_from_errno()),
			any => Ok(any),
		}
	}
}

impl FileDescriptor {
	pub fn close(&mut self) -> CResult<()> {
		let result = unsafe { libc::close(self.fd) };

		self.fd = -1;

		if result == -1 {
			Err(CError::new_from_errno())
		}
		else {
			Ok(())
		}
	}

	pub fn try_clone(&self) -> CResult<Self> {
		match unsafe { libc::dup(self.fd) } {
			-1 => Err(CError::new_from_errno()),
			new_fd => Ok(Self {
				fd: new_fd
			})
		}
	}

	pub fn read_bytes(&mut self, bytes: usize) -> CResult<Vec<u8>> {
		let mut result = vec![];
		result.resize(bytes, 0);

		match unsafe { libc::read(self.fd, result.as_mut_ptr() as *mut libc::c_void, result.len())} {
			-1 => Err(CError::new_from_errno()),
			bytes_read => {
				result.resize(bytes_read as usize, 0);
				Ok(result)
			}			
		}
	}

	pub fn read_exact(&mut self, bytes: usize) -> CResult<Vec<u8>> {
		let mut result = vec![];
		result.resize(bytes, 0);
		let mut bytes_read = 0;

		while bytes_read < bytes {
			match unsafe { libc::read(self.fd, result[bytes_read..].as_mut_ptr() as *mut libc::c_void, result.len() - bytes_read)} {
				-1 => return Err(CError::new_from_errno()),
				br => {
					bytes_read += br as usize;
				}			
			}
		}

		Ok(result)
	}

	pub unsafe fn read_any<T: Sized>(&mut self) -> CResult<T> {
		let size = size_of::<T>();
		// let mut reconstructed = unsafe { core::mem::zeroed() };
		let mut reconstructed = core::mem::zeroed();
		let result = self.read_exact(size)?;
		let reconstructed_ptr: *mut T = &mut reconstructed;
		// unsafe {
			let mut reconstructed_bin_ptr: *mut u8 = core::mem::transmute(reconstructed_ptr);
			for i in 0..size {
				*reconstructed_bin_ptr = result[i];
				reconstructed_bin_ptr = reconstructed_bin_ptr.add(1);
			}
		// }
		Ok(reconstructed)
	}

	pub unsafe fn write_raw<T>(&mut self, ptr: *const T, size: usize) -> CResult<usize> {
		match libc::write(self.fd, ptr as *const libc::c_void, size) {
			-1 => Err(CError::new_from_errno()),
			bytes_written => Ok(bytes_written as usize)
		}
	}

	pub fn write_slice(&mut self, data: &[u8]) -> CResult<usize> {
		unsafe { self.write_raw(data.as_ptr(), data.len()) }
		// match unsafe { libc::write(self.fd, data.as_ptr() as *const libc::c_void, data.len()) } {
		// 	-1 => Err(CError::new_from_errno()),
		// 	bytes_written => Ok(bytes_written as usize)
		// }
	}

	pub fn write<T : Into<Vec<u8>>>(&mut self, data: T) -> CResult<usize> {
		let data = data.into();
		self.write_slice(&data)
	}

	pub fn write_any<T : Sized>(&mut self, data: T) -> CResult<usize> {
		let size = size_of::<T>();
		unsafe { self.write_raw(&data, size) }
	}

	pub fn is_a_tty(&self) -> bool {
		unsafe { libc::isatty(self.fd) == 1 } 
	}
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        if self.fd != -1 {
			let _ = self.close();
		}
    }
}

pub mod constants {
	pub use libc::{
		STDIN_FILENO,
		STDOUT_FILENO,
		STDERR_FILENO,
	};
}
