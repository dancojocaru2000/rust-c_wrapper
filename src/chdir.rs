use std::ffi::CString;

use crate::{c_error::CError, c_result::CResult};

pub fn chdir<Path: Into<CString>>(path: Path) -> CResult<()> {
	let path: CString = path.into();
	match unsafe { libc::chdir(path.as_ptr()) } {
		0 => Ok(()),
		-1 => Err(CError::new_from_errno()),
		bad_return => panic!(format!("Unknown return from access: {}", bad_return)),
	}
}
