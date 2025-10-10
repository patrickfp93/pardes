#[cfg(test)]
pub(crate) mod tests;
pub(crate) mod samples;
pub mod struture;
pub mod error{
    use crate::struture;
    use thiserror::Error;
    #[derive(Error, Debug)]
    pub enum PardesError{
        #[error("Struct:{0}")]
        Struct(#[from] struture::error::ErrorStruture)
    }

}

pub type Result<T> = std::result::Result<T,error::PardesError>;

#[allow(unused)]
use crate::samples::struture::*;
