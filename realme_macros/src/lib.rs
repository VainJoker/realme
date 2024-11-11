use proc_macro::TokenStream;
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
    parse_macro_input,
};

#[derive(Debug)]
struct AdaptorInput {
    raw:      LitStr,
    // source:   Option<Ident>,
    parser:   Option<Path>,
    priority: Option<LitInt>,
    profile:  Option<LitStr>,
    // watch:        LitBool,
}

impl Parse for AdaptorInput {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Err(input.error("missing necessary arguments"));
        }
        let raw: LitStr = input.parse()?;

        input.parse::<Token![,]>()?;

        if input.is_empty() {
            return Err(input.error("missing necessary arguments"));
        }

        // Parse the named arguments
        // let mut source = None;
        let mut parser = None;
        let mut priority = None;
        let mut profile = None;

        // Parse remaining name = value pairs
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match ident.to_string().as_str() {
                // "source" => source = input.parse()?,
                "parser" => parser = Some(input.parse()?),
                "priority" => priority = input.parse()?,
                "profile" => profile = Some(input.parse()?),
                _ => return Err(input.error("unexpected parameter")),
            }

            // Parse optional trailing comma
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(AdaptorInput {
            raw,
            // source,
            parser,
            priority,
            profile,
        })
    }
}

impl ToTokens for AdaptorInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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

#[proc_macro]
pub fn realme_file(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AdaptorInput);

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

    TokenStream::from(res)
}
