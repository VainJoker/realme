use thiserror::Error;

#[derive(Error, Debug)]
pub enum RealmError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
