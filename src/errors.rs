use thiserror::Error;

#[derive(Error, Debug)]
pub enum RealmError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl serde::de::Error for RealmError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Anyhow(anyhow::anyhow!(msg.to_string()))
    }
}
