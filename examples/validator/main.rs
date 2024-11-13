#[cfg(feature = "toml")]
fn main() {
    use realme::prelude::*;
    use serde::Deserialize;
    use validator::{
        Validate,
        ValidationError,
    };

    #[derive(Debug, Validate, Deserialize)]
    struct SignupData {
        #[validate(email)]
        mail:       String,
        #[validate(url)]
        site:       String,
        #[validate(
            length(min = 1),
            custom(function = "validate_unique_username")
        )]
        #[serde(rename = "firstName")]
        first_name: String,
        #[validate(range(min = 18, max = 20))]
        age:        u32,
        #[validate(range(exclusive_min = 0.0, max = 100.0))]
        height:     f32,
    }

    fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
        if username == "xXxShad0wxXx" {
            // the value of the username will automatically be added later
            return Err(ValidationError::new("terrible_username"));
        }

        Ok(())
    }

    let c = String::from(
        r#"
        mail = "xXxShad0wxXx@gmail.com"
        site = "https://xXxShad0wxXx.com"
        firstName = "xXxShad0wxXx"
        age = 18
        height = 180.0
        "#,
    );

    let realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(&c)))
        .build()
        .expect("Building configuration object");

    let data: SignupData =
        realme.try_deserialize().expect("deserialize failed");

    println!("{:#?}", data.validate());
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}
