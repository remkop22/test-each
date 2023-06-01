use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    meta::ParseNestedMeta, parse_macro_input, spanned::Spanned, Error, ItemFn, LitInt, LitStr,
    Result,
};

struct Attrs {
    glob: Option<String>,
    segments: usize,
    index: bool,
    extension: bool,
}

impl Attrs {
    pub fn new() -> Self {
        Self {
            glob: None,
            segments: 1,
            index: false,
            extension: false,
        }
    }

    fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("glob") {
            let glob: LitStr = meta.value()?.parse()?;
            self.glob = Some(glob.value());
        } else if meta.path.is_ident("name") {
            meta.parse_nested_meta(|nested| {
                if nested.path.is_ident("segments") {
                    let path_segments: LitInt = nested.value()?.parse()?;
                    self.segments = path_segments.base10_parse()?;
                } else if nested.path.is_ident("index") {
                    self.index = true
                } else if nested.path.is_ident("extension") {
                    self.extension = true
                } else {
                    return Err(nested.error(
                        "unsupported property, specify `segments = <num>`, `index` or `extension`",
                    ));
                }

                Ok(())
            })?;
        } else {
            return Err(meta.error("unsupported property, specify `glob` or `name`"));
        }

        Ok(())
    }
}

#[proc_macro_attribute]
pub fn test_each_file(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut attrs = Attrs::new();
    let attr_parser = syn::meta::parser(|meta| attrs.parse(meta));
    let input = parse_macro_input!(input as ItemFn);
    parse_macro_input!(args with attr_parser);

    match test_each(attrs, input, Kind::File) {
        Ok(output) => output,
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn test_each_blob(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut attrs = Attrs::new();
    let attr_parser = syn::meta::parser(|meta| attrs.parse(meta));
    let input = parse_macro_input!(input as ItemFn);
    parse_macro_input!(args with attr_parser);

    match test_each(attrs, input, Kind::Blob) {
        Ok(output) => output,
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn test_each_path(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut attrs = Attrs::new();
    let attr_parser = syn::meta::parser(|meta| attrs.parse(meta));
    let input = parse_macro_input!(input as ItemFn);
    parse_macro_input!(args with attr_parser);

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

fn test_each(attrs: Attrs, input: ItemFn, kind: Kind) -> Result<TokenStream> {
    let mut functions = vec![input.to_token_stream()];

    let name = &input.sig.ident;
    let vis = &input.vis;
    let ret = &input.sig.output;
    let n_args = input.sig.inputs.len();

    let pattern = attrs
        .glob
        .as_ref()
        .ok_or_else(|| Error::new(input.span(), "missing `glob` attribute"))?;

    let files = glob::glob(pattern)
        .map_err(|err| Error::new(input.span(), format!("invalid path glob pattern: {}", err)))?;

    for (i, file) in files.enumerate() {
        let mut file = file
            .map_err(|err| Error::new(input.span(), format!("could not read directory: {}", err)))?
            .canonicalize()
            .map_err(|err| Error::new(input.span(), format!("could not read file: {}", err)))?;

        match kind {
            Kind::File | Kind::Blob if file.is_dir() => continue,
            _ => {}
        };

        let path = file.to_string_lossy().to_string();

        if !attrs.extension {
            file.set_extension("");
        }

        let mut path_segments = file
            .iter()
            .rev()
            .take(attrs.segments)
            .map(|s| make_safe_ident(&s.to_string_lossy()))
            .collect::<Vec<_>>();

        path_segments.reverse();

        let path_name = path_segments.join("_");

        let test_name = if attrs.index {
            format_ident!("{}_{}_{}", name, path_name, i)
        } else {
            format_ident!("{}_{}", name, path_name)
        };

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
            #[allow(non_snake_case)]
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

    result.trim_matches('_').to_string()
}
