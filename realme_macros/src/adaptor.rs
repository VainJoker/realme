pub(crate) mod file;
pub use file::file_adaptor;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};
use syn::{
    Ident,
    LitInt,
    LitStr,
    Path,
    Token,
    parse::{
        Parse,
        ParseStream,
        Result,
    },
};

/// Input parameters for configuring an adaptor
#[derive(Debug, Clone)]
pub(crate) struct AdaptorInput {
    raw:      LitStr,
    parser:   Option<Path>,
    priority: Option<LitInt>,
    profile:  Option<LitStr>,
}

impl AdaptorInput {
    pub(crate) fn set_parser(&mut self, parser: &str) {
        self.parser =
            Some(syn::Path::from(syn::Ident::new(parser, self.raw.span())));
    }
}

impl Parse for AdaptorInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse required raw string
        let raw = input.parse().map_err(|_| {
            input.error("expected string literal as first argument")
        })?;

        if !input.is_empty() {
            // Require comma after raw string
            input.parse::<Token![,]>().map_err(|_| {
                input.error("expected comma after string literal")
            })?;
        }

        let mut parser = None;
        let mut priority = None;
        let mut profile = None;

        // Parse named arguments
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match ident.to_string().as_str() {
                "parser" => parser = Some(input.parse()?),
                "priority" => priority = Some(input.parse()?),
                "profile" => profile = Some(input.parse()?),
                name => {
                    return Err(
                        input.error(format!("unknown parameter '{}'", name))
                    );
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(AdaptorInput {
            raw,
            parser,
            priority,
            profile,
        })
    }
}

impl ToTokens for AdaptorInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let raw = &self.raw;
        let mut expr = if let Some(parser) = &self.parser {
            quote! {
                Adaptor::new(FileSource::<#parser>::new(#raw))
            }
        } else {
            quote! {
                Adaptor::new(FileSource::new(#raw))
            }
        };

        // Add priority if specified
        if let Some(priority) = &self.priority {
            expr = quote! {
                #expr.priority(#priority)
            };
        }

        // Add profile if specified
        if let Some(profile) = &self.profile {
            expr = quote! {
                #expr.profile(#profile)
            };
        }

        expr.to_tokens(tokens);
    }
}
