use std::{ffi::CString, ptr::null_mut};

use crate::{c_error::CError, c_result::CResult};

pub fn exec(pathname: &str, argv: Vec<String>) -> CResult<()> {
	let pathname = CString::new(pathname).unwrap();	// A Rust String will never error
	let mut argv: Vec<_> = argv.into_iter().map(
		|arg| CString::new(arg).unwrap()
	).map(
		|arg| arg.into_raw() as *const libc::c_char	// Transfer ownership to C
	).collect();

	argv.push(null_mut());

	let argv_ptr = argv.as_ptr();

	let result = unsafe { libc::execv(pathname.as_ptr(), argv_ptr) }; 

	// At this point, the program continues only if execv encountered an error

	assert!(result == -1);

	Err(CError::new_from_errno())
}

pub fn exece(pathname: &str, argv: Vec<String>, env: Vec<String>) -> CResult<()> {
	let pathname = CString::new(pathname).unwrap();	// A Rust String will never error
	let mut argv: Vec<_> = argv.into_iter().map(
		|arg| CString::new(arg).unwrap()
	).map(
		|arg| arg.into_raw() as *const libc::c_char	// Transfer ownership to C
	).collect();

	argv.push(null_mut());

	let argv_ptr = argv.as_ptr();

	let mut env: Vec<_> = env.into_iter().map(
		|arg| CString::new(arg).unwrap()
	).map(
		|arg| arg.into_raw() as *const libc::c_char	// Transfer ownership to C
	).collect();

	env.push(null_mut());

	let env_ptr = env.as_ptr();

	let result = unsafe { libc::execve(pathname.as_ptr(), argv_ptr, env_ptr) };

	// At this point, the program continues only if execv encountered an error

	assert!(result == -1);

	Err(CError::new_from_errno())
}

pub fn execp(file: &str, argv: Vec<String>) -> CResult<()> {
	let file = CString::new(file).unwrap();	// A Rust String will never error
	let mut argv: Vec<_> = argv.into_iter().map(
		|arg| CString::new(arg).unwrap()
	).map(
		|arg| arg.into_raw() as *const libc::c_char	// Transfer ownership to C
	).collect();

	argv.push(null_mut());

	let argv_ptr = argv.as_ptr();

	let result = unsafe { libc::execvp(file.as_ptr(), argv_ptr) };

	// At this point, the program continues only if execv encountered an error

	assert!(result == -1);

	Err(CError::new_from_errno())
}

#[cfg(target_os = "linux")]
pub fn execpe(file: &str, argv: Vec<String>, env: Vec<String>) -> CResult<()> {
	let file = CString::new(file).unwrap();	// A Rust String will never error
	let mut argv: Vec<_> = argv.into_iter().map(
		|arg| CString::new(arg).unwrap()
	).map(
		|arg| arg.into_raw() as *const libc::c_char	// Transfer ownership to C
	).collect();

	argv.push(null_mut());

	let argv_ptr = argv.as_ptr();

	let mut env: Vec<_> = env.into_iter().map(
		|arg| CString::new(arg).unwrap()
	).map(
		|arg| arg.into_raw() as *const libc::c_char	// Transfer ownership to C
	).collect();

	env.push(null_mut());

	let env_ptr = env.as_ptr();

	let result = unsafe { libc::execvpe(file.as_ptr(), argv_ptr, env_ptr) };

	// At this point, the program continues only if execv encountered an error

	assert!(result == -1);

	Err(CError::new_from_errno())
}
