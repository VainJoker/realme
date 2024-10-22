use realme::prelude::*;

#[derive(Debug, Clone)]
pub struct PemParser;

impl<T: AsRef<str>> Parser<T> for PemParser {
    type Item = Value;
    type Error = realme::errors::Error;

    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let key_type = vec!["PUBLIC", "PRIVATE"]
            .into_iter()
            .find(|s| args.as_ref().contains(s));

        let key = match key_type {
            Some("PRIVATE") => "private_key",
            Some("PUBLIC") => "public_key",
            _ => {
                return Err(realme::errors::Error::new_parse_error(
                    args.as_ref().to_string(),
                    "PEM file did not contain a Private or Public key"
                        .to_string(),
                ));
            }
        };
        let mut result = Value::Null;
        result.set(key, Value::String(args.as_ref().to_string()))?;
        Ok(result)
    }
}

fn main() {
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<PemParser>::new(
            "./examples/custom_format/pem/private.pem",
        )))
        .load(Adaptor::new(FileSource::<PemParser>::new(
            "./examples/custom_format/pem/public.pem",
        )))
        .build()
        .expect("Building configuration object");
    println!("{realme:#?}");
}
