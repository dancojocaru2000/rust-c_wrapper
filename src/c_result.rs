use crate::c_error::CError;

pub type CResult<T> = Result<T, CError>;