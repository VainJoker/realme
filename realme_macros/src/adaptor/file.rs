use proc_macro2::TokenStream;
use quote::quote;

use super::AdaptorInput;

pub fn file_adaptor(input: AdaptorInput) -> TokenStream {
    let raw = input.raw.value();
    let parser = match input.parser {
        Some(parser) => parser,
        None => {
            return syn::Error::new(input.raw.span(), "parser is required")
                .to_compile_error()
                .into();
        }
    };
    let priority = input.priority;
    let profile = input.profile;

    let res = quote! {
        Adaptor::new(FileSource::<#parser>::new(#raw))
    };

    let res = if let Some(priority) = priority {
        quote! {
            #res.priority(#priority)
        }
    } else {
        res
    };

    let res = if let Some(profile) = profile {
        quote! {
            #res.profile(#profile)
        }
    } else {
        res
    };

    res
}
