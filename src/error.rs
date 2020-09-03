use thiserror::*;

pub type JvmResult<T> = Result<T, JvmError>;

#[derive(Error, Debug)]
pub enum JvmError {

}
