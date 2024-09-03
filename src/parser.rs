#[derive(Debug)]
pub enum FormatParser {
    Toml,
    Json,
    Yaml,
    Xml,
    Hcl,
    Ini,
    Properties,
}
