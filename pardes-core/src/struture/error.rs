use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorStruture {
    #[error("Not compatible with Unit structures.")]
    NoFields,
}
