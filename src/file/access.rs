use std::ffi::CString;

use crate::{c_result::CResult, c_error::CError};

pub fn access<Path: Into<CString>>(path: Path, check_for: AccessCheck) -> CResult<bool> {
	let path: CString = path.into();
	match unsafe { libc::access(path.as_ptr(), check_for.into()) } {
		0 => Ok(true),
		-1 => match CError::new_from_errno() {
			CError::PermissionDenied if check_for != AccessCheck::FileExists => Ok(false),
			CError::NotFound if check_for == AccessCheck::FileExists => Ok(false),
			err => Err(err),
		},
		bad_return => panic!(format!("Unknown return from access: {}", bad_return)),
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessCheck {
	FileExists,
	Read,
	Write,
	Execute,
	ReadWrite,
	ReadExecute,
	WriteExecute,
	AllPermissions,
}

impl From<libc::c_int> for AccessCheck {
    fn from(bits: libc::c_int) -> Self {
        if bits == constants::F_OK {
			Self::FileExists
		}
		else if bits == (constants::R_OK | constants::W_OK | constants::X_OK) {
			Self::AllPermissions
		}
		else if bits == (constants::R_OK | constants::W_OK) {
			Self::ReadWrite
		}
		else if bits == (constants::R_OK | constants::X_OK) {
			Self::ReadExecute
		}
		else if bits == (constants::W_OK | constants::X_OK) {
			Self::WriteExecute
		}
		else if bits == constants::R_OK {
			Self::Read
		}
		else if bits == constants::W_OK {
			Self::Write
		}
		else if bits == constants::X_OK {
			Self::Execute
		}
		else {
			panic!(format!("Unknown number given for conversion: {}", bits))
		}
    }
}

impl From<AccessCheck> for libc::c_int {
    fn from(ac: AccessCheck) -> Self {
        match ac {
            AccessCheck::FileExists => constants::F_OK,
            AccessCheck::Read => constants::R_OK,
            AccessCheck::Write => constants::W_OK,
            AccessCheck::Execute => constants::X_OK,
            AccessCheck::ReadWrite => constants::R_OK | constants::W_OK,
            AccessCheck::ReadExecute => constants::R_OK | constants::X_OK,
            AccessCheck::WriteExecute => constants::W_OK | constants::X_OK,
            AccessCheck::AllPermissions => constants::R_OK | constants::W_OK | constants::X_OK,
        }
    }
}

pub mod constants {
	pub use libc::{
		F_OK,
		R_OK,
		W_OK,
		X_OK,
	};
}
