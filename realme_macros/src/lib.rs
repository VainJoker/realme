use adaptor::AdaptorInput;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod adaptor;

#[proc_macro]
pub fn file(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AdaptorInput);
    adaptor::file_adaptor(input).into()
}

#[proc_macro]
pub fn toml(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as AdaptorInput);
    input.set_parser("TomlParser");
    adaptor::file_adaptor(input).into()
}
