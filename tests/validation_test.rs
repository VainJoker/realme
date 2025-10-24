use realme::{
    Error,
    prelude::*,
};

#[test]
fn validate_non_empty_passes() -> std::result::Result<(), Error> {
    // Provide defaults prior to validation so build does not fail
    let defaults = Value::Table(Table::from_iter(vec![(
        "k".to_string(),
        Value::Integer(1),
    )]));
    let realme = Realme::builder()
        .with_defaults(defaults)
        .validate_on_build(true)
        .build()?;
    assert_eq!(realme.get("k"), Some(&Value::Integer(1)));
    Ok(())
}

#[test]
fn set_array_invalid_index() {
    // Setting array index with invalid string should raise ValidationError
    let mut root = Value::Array(vec![]);
    let err = root
        .set("abc", Value::Integer(1))
        .expect_err("should error");
    match err {
        Error::ValidationError(v) => {
            assert!(v.cause.contains("Invalid array index"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
    // Use numeric index succeeds
    let _ = root.set("0", Value::Integer(2)).expect("set index 0");
}
