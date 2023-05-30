use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::Parser, parse_macro_input, punctuated::Punctuated, Error, ItemFn, LitStr, Result, Token,
};

#[proc_macro_attribute]
pub fn test_each_file(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    match test_each(attrs, input, Kind::File) {
        Ok(output) => output,
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn test_each_blob(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    match test_each(attrs, input, Kind::Blob) {
        Ok(output) => output,
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn test_each_path(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    match test_each(attrs, input, Kind::Path) {
        Ok(output) => output,
        Err(err) => err.into_compile_error().into(),
    }
}

enum Kind {
    File,
    Blob,
    Path,
}

fn test_each(attrs: TokenStream, input: ItemFn, kind: Kind) -> Result<TokenStream> {
    let lits = Punctuated::<LitStr, Token![,]>::parse_terminated.parse(attrs)?;
    let mut functions = vec![input.clone().to_token_stream()];

    let name = input.sig.ident;
    let vis = input.vis;
    let ret = input.sig.output;
    let n_args = input.sig.inputs.len();

    if lits.len() != 1 {
        return Err(Error::new(
            name.span(),
            "expected a single path glob literal",
        ));
    }

    let pattern = &lits[0].value();

    let files = glob::glob(pattern).map_err(|err| {
        Error::new(
            lits[0].span(),
            format!("invalid path glob pattern: {}", err),
        )
    })?;

    for (i, file) in files.enumerate() {
        let file = file.map_err(|err| {
            Error::new(lits[0].span(), format!("could not read directory: {}", err))
        })?;

        match kind {
            Kind::File | Kind::Blob if file.is_dir() => continue,
            _ => {}
        };

        let file_name = file
            .file_name()
            .map(|name| format!("{}_", make_safe_ident(&name.to_string_lossy())))
            .unwrap_or_default();

        let test_name = format_ident!("{}_{}{}", name, file_name, i);

        let path = file
            .canonicalize()
            .map_err(|err| Error::new(lits[0].span(), format!("could not read file: {}", err)))?
            .to_string_lossy()
            .to_string();

        let into_path = quote!(::std::path::PathBuf::from(#path));

        let call = match kind {
            Kind::File if n_args < 2 => quote!(#name(include_str!(#path))),
            Kind::File => quote!(#name(include_str!(#path), #into_path)),
            Kind::Blob if n_args < 2 => quote!(#name(include_bytes!(#path))),
            Kind::Blob => quote!(#name(include_bytes!(#path), #into_path)),
            Kind::Path => quote!(#name(#into_path)),
        };

        functions.push(quote! {
            #[test]
            #vis fn #test_name() #ret {
                #call
            }
        });
    }

    Ok(quote!( #(#functions)* ).into())
}

fn make_safe_ident(value: &str) -> String {
    let mut result = String::with_capacity(value.len());

    for c in value.chars() {
        if c.is_alphanumeric() {
            result.push(c);
        } else {
            result.push('_');
        }
    }

    result
}
