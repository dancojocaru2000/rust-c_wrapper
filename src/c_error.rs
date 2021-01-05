use std::ffi::CStr;
use serde::{Serialize, Deserialize};

macro_rules! generate_cerror {
    ($($rust_names:ident; to C $c_names:ident),+$(,)*) => {
        #[derive(Deserialize, Serialize, Clone, Copy, Debug)]
        pub enum CError {
            $(
                $rust_names,
            )+
            Unknown(libc::c_int),
        }

        impl From<libc::c_int> for CError {
            fn from(errno: libc::c_int) -> Self {
                match errno {
                    $(
                        #[allow(unreachable_patterns)]
                        libc::$c_names => Self::$rust_names,
                    )+
                    _ => Self::Unknown(errno)
                }
            }
        }

        impl Into<libc::c_int> for &CError {
            fn into(self) -> libc::c_int {
                match self {
                    $(
                        CError::$rust_names => libc::$c_names,
                    )+
                    CError::Unknown(errno) => *errno
                }
            }
        }

        impl Into<libc::c_int> for CError {
            fn into(self) -> libc::c_int {
                match self {
                    $(
                        CError::$rust_names => libc::$c_names,
                    )+
                    CError::Unknown(errno) => errno
                }
            }
        }
    };
}


generate_cerror!(
    Again; to C EAGAIN,
    NoMemory; to C ENOMEM,
    NoSys; to C ENOSYS,
    Child; to C ECHILD,
    Invalid; to C EINVAL,
    Interrupted;  to C EINTR,
    TooBig; to C E2BIG,
    BadFileDescriptor; to C EBADF,
    MFile; to C EMFILE,
    IO; to C EIO,
    NoSpace; to C ENOSPC,
    DiskQuota; to C EDQUOT,
    WouldBlock; to C EWOULDBLOCK,
    DestinationAddressRequired; to C EDESTADDRREQ,
    Fault; to C EFAULT,
    FileTooLarge; to C EFBIG,
    Perm; to C EPERM,
    BrokenPipe; to C EPIPE,
    IsDirectory; to C EISDIR,
    NFile; to C ENFILE,
    Range; to C ERANGE,
    NotFound; to C ENOENT,
    PermissionDenied; to C EACCES,
);

impl CError {
    pub fn new_from_errno() -> Self {
        let errno = errno::errno();
        Self::from(errno.0)
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
