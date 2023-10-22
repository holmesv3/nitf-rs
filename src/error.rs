use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum NitfError {
    #[error("error parsing a nitf field")]
    FieldError,
    #[error("can't parse data with abridged bit-storage {size:?}")]
    DataBitError { size: usize },
    #[error("{0} is not implemented")]
    NotImplementedError(String),
    #[error("unknown error in nitf crate")]
    Unknown,
}
