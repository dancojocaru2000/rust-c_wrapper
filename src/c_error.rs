use std::ffi::CStr;

pub enum CError {
	EAGAIN,
	ENOMEM,
	ENOSYS,
	Unknown(libc::c_int)
}

impl CError {
    pub fn new_from_errno() -> Self {
        let errno = errno::errno();
        Self::from(errno.0)
    }
}

impl From<libc::c_int> for CError {
    fn from(errno: libc::c_int) -> Self {
        match errno {
			libc::EAGAIN => Self::EAGAIN,
			libc::ENOMEM => Self::ENOMEM,
			libc::ENOSYS => Self::ENOSYS,
			_ => Self::Unknown(errno)
		}
    }
}

impl Into<libc::c_int> for &CError {
    fn into(self) -> libc::c_int {
        match self {
            CError::EAGAIN => libc::EAGAIN,
            CError::ENOMEM => libc::ENOMEM,
            CError::ENOSYS => libc::ENOSYS,
            CError::Unknown(errno) => *errno
        }
    }
}

impl std::fmt::Display for CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message_c = unsafe { CStr::from_ptr(
            libc::strerror(self.into())
        ) }; // strerror should always return a string!
        let error_message = match error_message_c.to_str() {
            Ok(e) => e,
            Err(_) => {
                return Err(std::fmt::Error);
            }
        };
        write!(f, "{}", error_message)?;
        Ok(())
    }
}