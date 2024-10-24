use super::Realme;
use crate::{
    Error,
    prelude::*,
};

/// A builder for creating a `Realme` instance.
///
/// This struct collects adaptors from various sources and constructs a `Realme`
/// with a configured environment.
impl RealmeBuilder {
    /// Creates a default `RealmeBuilder` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an `Adaptor` to the builder based on its source type.
    ///
    /// This method takes ownership of the builder and returns it after
    /// modifying, allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `adaptor` - The `Adaptor` to be added to the builder.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// let adaptor = Adaptor::new(...);
    /// let builder = RealmeBuilder::new().load(adaptor);
    /// ```
    #[must_use]
    pub fn load<A: Into<Adaptor>>(mut self, adaptor: A) -> Self {
        self.adaptors.push(adaptor.into());
        self
    }

    /// Sets the profile for the `Realme` instance.
    ///
    /// This method takes ownership of the builder and returns it after
    /// modifying, allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `profile` - The profile to be set for the `Realme` instance. e.g.
    ///   `dev`, `prod`. which you have seted in the adaptor.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// let builder = RealmeBuilder::new().load(...).profile("dev").build();
    /// ```
    #[must_use]
    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    /// Constructs a `Realme` instance using the accumulated adaptors and
    /// profile.
    ///
    /// This method verifies the profile, orders adaptors by priority, and
    /// assembles the `Realme` instance.
    ///
    /// If any adaptor has a watch set, use the `shared_build` method instead.
    ///
    /// # Returns
    ///
    /// * `Result<Realme, Error>` - A `Result` containing the constructed
    ///   `Realme` instance or an `Error` if the build process fails.
    pub fn build(mut self) -> Result<Realme, Error> {
        self.check_profile()?;
        self.adaptors.sort_by(|a, b| a.priority.cmp(&b.priority));
        let mut cache = Value::Table(Map::new());
        self.adaptors.iter().try_for_each(|adaptor| {
            adaptor.parse().and_then(|value| match value {
                Value::Table(table) => {
                    cache.merge(&Value::Table(table));
                    Ok(())
                }
                Value::Null => Ok(()),
                _ => Err(Error::new_build_error(
                    "Adaptor parse result is not a table".to_string(),
                )),
            })
        })?;

        Ok(Realme {
            cache,
            default: None,
            builder: self,
        })
    }

    pub(crate) fn check_profile(&mut self) -> Result<(), Error> {
        let mut profile_not_found = self.profile.is_some();
        self.adaptors.retain(|adaptor| {
            match (&adaptor.profile, &self.profile) {
                (None, _) => true,
                (Some(adaptor_profile), Some(self_profile))
                    if adaptor_profile == self_profile =>
                {
                    profile_not_found = false;
                    true
                }
                _ => false,
            }
        });
        if profile_not_found {
            return Err(Error::new_build_error(format!(
                "Can not find profile {}",
                self.profile.as_ref().expect("Profile is not set")
            )));
        }
        Ok(())
    }
}

#[cfg(feature = "toml")]
#[cfg(test)]
mod tests {
    use std::io::Write;

    use anyhow::Error;
    use tempfile::NamedTempFile;
    use toml::toml;

    use crate::prelude::*;

    fn create_temp_toml(content: &str) -> NamedTempFile {
        let mut file =
            NamedTempFile::new().expect("Failed to create temp file");
        writeln!(file, "{content}").expect("Failed to write to temp file");
        file
    }

    #[test]
    fn test_build_with_empty_adaptors() {
        let builder = RealmeBuilder::new();
        let _realme = builder.build().expect("Failed to build");
    }

    #[test]
    fn test_build_with_single_adaptor() -> Result<(), Error> {
        let config = create_temp_toml(
            r#"
            [server]
            host = "localhost"
            port = 8080
        "#,
        );

        let builder = RealmeBuilder::new().load(
            Adaptor::new(FileSource::<TomlParser>::new(config.path()))
                .priority(1),
        );
        let realme = builder.build()?;

        assert_eq!(
            realme.get("server.host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(realme.get("server.port"), Some(&Value::Integer(8080)));
        Ok(())
    }

    #[test]
    fn test_build_with_multiple_adaptors() -> Result<(), Error> {
        let config1 = create_temp_toml(
            r#"
            [server]
            host = "localhost"
            port = 8080
        "#,
        );
        let config2 = create_temp_toml(
            r#"
            [database]
            url = "postgres://localhost/mydb"
        "#,
        );

        let builder = RealmeBuilder::new()
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(config1.path()))
                    .priority(1),
            )
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(config2.path()))
                    .priority(2),
            );
        let realme = builder.build()?;

        assert_eq!(
            realme.get("server.host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(
            realme.get("database.url"),
            Some(&Value::String("postgres://localhost/mydb".to_string()))
        );
        Ok(())
    }

    #[test]
    fn test_build_with_profile() -> Result<(), Error> {
        let config1 = toml! {
            [server]
            host = "localhost"
            port = 8080
        };

        let config2 = toml! {
            [server]
            host = "example.com"
            port = 80
        };

        let builder = RealmeBuilder::new()
            .load(
                Adaptor::new(SerSource::<SerParser, _>::new(config1))
                    .profile("dev"),
            )
            .load(
                Adaptor::new(SerSource::<SerParser, _>::new(config2))
                    .profile("prod"),
            )
            .profile("dev");
        let realme = builder.build()?;

        assert_eq!(realme.get("server.port"), Some(&Value::Integer(8080)));
        Ok(())
    }

    #[test]
    fn test_build_with_non_existent_profile() {
        let config = create_temp_toml(
            r#"
            [server]
            host = "localhost"
            port = 8080
        "#,
        );

        let builder = RealmeBuilder::new()
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(config.path()))
                    .priority(1),
            )
            .profile("non_existent");
        assert!(builder.build().is_err());
    }

    #[test]
    fn test_build_with_priority_sorting() -> Result<(), Error> {
        let low_priority = create_temp_toml(
            r#"
            [server]
            host = "localhost"
            port = 8080
        "#,
        );
        let high_priority = create_temp_toml(
            "
            [server]
            port = 9000
        ",
        );

        let builder = RealmeBuilder::new()
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(
                    low_priority.path(),
                ))
                .priority(1),
            )
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(
                    high_priority.path(),
                ))
                .priority(2),
            );
        let realme = builder.build()?;

        assert_eq!(
            realme.get("server.host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(realme.get("server.port"), Some(&Value::Integer(9000)));
        Ok(())
    }

    #[test]
    fn test_build_with_invalid_adaptor() {
        let builder = RealmeBuilder::new().load(
            Adaptor::new(FileSource::<TomlParser>::new("non_existent.toml"))
                .priority(1),
        );
        assert!(builder.build().is_err());
    }

    #[test]
    // TODO: it can be build with select function in the future
    fn test_build_with_profile_filtering() -> Result<(), Error> {
        let dev_config = create_temp_toml(
            r#"
            [server.dev]
            host = "localhost"
            port = 3000
        "#,
        );
        let prod_config = create_temp_toml(
            r#"
            [server.prod]
            host = "example.com"
            port = 80
        "#,
        );

        let builder = RealmeBuilder::new()
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(dev_config.path()))
                    .profile("dev")
                    .priority(1),
            )
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(prod_config.path()))
                    .profile("prod")
                    .priority(2),
            )
            .profile("dev");
        let realme = builder.build()?;

        assert_eq!(
            realme.get("server.dev.host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(realme.get("server.dev.port"), Some(&Value::Integer(3000)));
        assert!(realme.get("server.prod.host").is_none());
        Ok(())
    }

    #[test]
    fn test_build_with_mixed_profile_and_non_profile_adaptors()
    -> Result<(), Error> {
        let common_config = create_temp_toml(
            r#"
            [database]
            url = "postgres://localhost/mydb"
        "#,
        );
        let dev_config = create_temp_toml(
            r#"
            [server.dev]
            host = "localhost"
            port = 3000
        "#,
        );

        let builder = RealmeBuilder::new()
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(
                    common_config.path(),
                ))
                .priority(1),
            )
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(dev_config.path()))
                    .profile("dev")
                    .priority(2),
            )
            .profile("dev");
        let realme = builder.build()?;

        assert_eq!(
            realme.get("database.url"),
            Some(&Value::String("postgres://localhost/mydb".to_string()))
        );
        assert_eq!(
            realme.get("server.dev.host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(realme.get("server.dev.port"), Some(&Value::Integer(3000)));
        Ok(())
    }

    #[test]
    fn test_build_with_array_values() -> Result<(), Error> {
        let config = create_temp_toml(
            r#"
            [app]
            name = "MyApp"
            version = "1.0.0"
            allowed_ips = ["127.0.0.1", "192.168.1.1", "10.0.0.1"]
        "#,
        );

        let builder = RealmeBuilder::new().load(
            Adaptor::new(FileSource::<TomlParser>::new(config.path()))
                .priority(1),
        );
        let realme = builder.build()?;

        assert_eq!(
            realme.get("app.name"),
            Some(&Value::String("MyApp".to_string()))
        );
        assert_eq!(
            realme.get("app.version"),
            Some(&Value::String("1.0.0".to_string()))
        );
        assert_eq!(
            realme.get("app.allowed_ips[0]"),
            Some(&Value::String("127.0.0.1".to_string()))
        );
        assert_eq!(
            realme.get("app.allowed_ips[1]"),
            Some(&Value::String("192.168.1.1".to_string()))
        );
        assert_eq!(
            realme.get("app.allowed_ips[2]"),
            Some(&Value::String("10.0.0.1".to_string()))
        );
        assert_eq!(realme.get("app.allowed_ips[3]"), None);
        Ok(())
    }

    #[test]
    fn test_build_with_profile_and_priority() -> Result<(), Error> {
        let config1 = create_temp_toml(
            "
            [server]
            port = 8080
            debug = true
        ",
        );
        let config2 = create_temp_toml(
            "
            [server]
            port = 9000
            debug = false
        ",
        );
        let config3 = create_temp_toml(
            "
            [server]
            port = 9000
            debug = true
        ",
        );
        let builder = RealmeBuilder::new()
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(config1.path()))
                    .profile("dev")
                    .priority(1),
            )
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(config2.path()))
                    .profile("prod")
                    .priority(2),
            )
            .load(
                Adaptor::new(FileSource::<TomlParser>::new(config3.path()))
                    .profile("prod")
                    .priority(3),
            )
            .profile("prod");
        let realme = builder.build()?;
        assert_eq!(realme.get("server.port"), Some(&Value::Integer(9000)));
        assert_eq!(realme.get("server.debug"), Some(&Value::Boolean(true)));
        Ok(())
    }
}
