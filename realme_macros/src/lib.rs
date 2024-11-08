use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, LitStr
};

#[proc_macro]
pub fn realme_macro(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let input_value = input_str.value();

    println!("realme_macro was called with: {}", input_value);

    TokenStream::from(quote!())
}


#[proc_macro]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let expanded = quote! {
        impl #struct_name {
            pub fn hello(&self) -> String {
                format!("Hello from {}", stringify!(#struct_name))
            }
        }
    };

    TokenStream::from(expanded)
}
