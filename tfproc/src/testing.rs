use std::fs;
use std::path::Path;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{Ident, LitStr, Token};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

const TF_FILE_EXT: &str = ".tf";

struct Args {
    name: Ident,
    dir: String,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let lit: LitStr = input.parse()?;
        Ok(Args { name, dir: lit.value() })
    }
}

pub fn proc_test_suite(input: TokenStream) -> TokenStream {
    let Args { name, dir } = parse_macro_input!(input as Args);

    let tests = build_tests(dir);
    let output = quote! {
        mod #name {
            #( #tests )*
        }
    };

    TokenStream::from(output)
}

fn build_tests<P>(dir: P) -> Vec<proc_macro2::TokenStream>
where P: AsRef<Path> {
    let mut out = vec![];
    let mut files = vec![];
    let mut subdirs = vec![];

    let entries = fs::read_dir(dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let mdata = entry.metadata().unwrap();
        let name = entry.file_name().into_string().unwrap();
        if mdata.is_file() && name.ends_with(TF_FILE_EXT) {
            files.push((name, entry));
        } else if mdata.is_dir() {
            subdirs.push((name, entry));
        }
    }

    files.sort_by(|a, b| a.0.cmp(&b.0));
    for entry in files {
        let (name, entry) = entry;
        let ident = format_ident!("{}", name.replace(TF_FILE_EXT, ""));
        let path = syn::LitStr::new(entry.path().to_str().unwrap(), ident.span());
        out.push(quote! {
            #[test]
            fn #ident() {
                crate::test_script(#path);
            }
        });
    }

    subdirs.sort_by(|a, b| a.0.cmp(&b.0));
    for entry in subdirs {
        let (name, entry) = entry;
        let tests = build_tests(entry.path());
        let ident = format_ident!("{}", name);
        out.push(quote! {
            mod #ident {
                #( #tests )*
            }
        });
    }

    out
}
