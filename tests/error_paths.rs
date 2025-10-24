use realme::{
    Error,
    prelude::*,
};

#[cfg(feature = "toml")]
#[test]
fn file_not_found_error() {
    let builder = Realme::builder().load(
        Adaptor::new(FileSource::<TomlParser>::new("__does_not_exist.toml"))
            .priority(1),
    );
    let err = builder.build().expect_err("should error");
    match err {
        Error::BuildError(msg) => assert!(msg.contains("Adaptor parse result")),
        Error::ReadFileError(_) | Error::SourceError(_) => { /* acceptable */ }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[cfg(feature = "toml")]
#[test]
fn profile_missing_error() {
    let builder = Realme::builder()
        .load(
            Adaptor::new(FileSource::<TomlParser>::new("config/default.toml"))
                .profile("dev"),
        )
        .profile("prod");
    let err = builder.build().expect_err("should error");
    match err {
        Error::BuildError(msg) => assert!(msg.contains("Can not find profile")),
        _ => panic!("unexpected variant"),
    }
}

#[test]
fn validation_empty_cache_error() {
    let err = Realme::builder()
        .validate_on_build(true)
        .build()
        .expect_err("should error");
    match err {
        Error::ValidationError(v) => {
            assert!(v.cause.contains("Cache is empty"));
        }
        _ => panic!("unexpected variant"),
    }
}
