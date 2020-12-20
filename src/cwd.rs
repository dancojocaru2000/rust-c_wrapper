use std::ffi::CStr;

use crate::c_result::CResult;
use crate::c_error::CError;

static INITIAL_BUFFER_SIZE: usize = 100;

pub fn getcwd() -> CResult<String> {
	let mut buffer = Vec::<libc::c_char>::new();
	buffer.resize(INITIAL_BUFFER_SIZE, 0);

	loop {
		let ptr = unsafe { libc::getcwd(buffer.as_mut_ptr(), buffer.len()) };
		if ptr == std::ptr::null_mut() {
			match CError::new_from_errno() {
				CError::Range => buffer.resize(buffer.len() * 2, 0),
				other => break Err(other),
			};
		}
		else {
			let str = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string();
			break Ok(str);
		}
	}
}
