use super::{cache::RealmCache, Realm};
use crate::{adaptor::source::SourceType, Adaptor, RealmError, Value};

#[derive(Debug, Default)]
pub struct RealmBuilder {
    env: Vec<Adaptor>,
    str: Vec<Adaptor>,
    cmd: Vec<Adaptor>,
    set: Vec<Adaptor>,
}

impl RealmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn load(mut self, adaptor: Adaptor) -> Self {
        match adaptor.source_type() {
            SourceType::Env => self.env.push(adaptor),
            SourceType::Str => self.str.push(adaptor),
            SourceType::Cmd => self.cmd.push(adaptor),
            SourceType::Set => self.set.push(adaptor),
        }
        self
    }

    pub fn build(&self) -> Result<Realm, RealmError> {
        let mut cache = RealmCache::new();

        for adaptor in &self.env {
            cache.handle_adaptor(adaptor, true)?;
        }
        let all_adaptors = [&self.str, &self.cmd, &self.set];
        for adaptors in &all_adaptors {
            for adaptor in *adaptors {
                cache.handle_adaptor(adaptor, false)?;
            }
        }
        Ok(Realm {
            cache: Value::Table(cache.cache),
        })
    }
}
