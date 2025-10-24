use adaptor::AdaptorInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput,
    Lit,
    parse_macro_input,
};

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

#[proc_macro_derive(Config, attributes(config))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident.clone();
    let mut prefix: Option<String> = None;
    for attr in ast.attrs.iter().filter(|a| a.path().is_ident("config")) {
        // syn 2: use parse_nested_meta
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("prefix") {
                if let Ok(Lit::Str(litstr)) =
                    meta.value().and_then(|v| v.parse())
                {
                    prefix = Some(litstr.value());
                }
            }
            Ok(())
        });
    }

    // Generate TryFrom<Realme> implementation
    let _prefix_match = if let Some(p) = prefix {
        quote! { let _base = #p; }
    } else {
        quote! { let _base = ""; }
    };

    let expanded = quote! {
        impl ::core::convert::TryFrom<realme::Realme> for #ident {
            type Error = realme::Error;
            fn try_from(r: realme::Realme) -> ::core::result::Result<Self, Self::Error> {
                // naive full deserialize first
                let full: Self = r.try_deserialize()?;
                Ok(full)
            }
        }
        impl #ident {
            pub fn load_from(realme: &realme::Realme) -> ::core::result::Result<Self, realme::Error> {
                let cfg: Self = realme.try_deserialize()?;
                Ok(cfg)
            }
        }
    };
    expanded.into()
}
