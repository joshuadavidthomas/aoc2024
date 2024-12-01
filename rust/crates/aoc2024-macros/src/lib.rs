use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

#[proc_macro_attribute]
pub fn env_vars(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => Ok(&data.variants),
        _ => Err(Error::new_spanned(
            &input,
            "env_vars can only be used with enums",
        )),
    };

    let variants = match variants {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    // Validate that all variants are unit variants (no fields)
    for variant in variants {
        if !variant.fields.is_empty() {
            return Error::new_spanned(variant, "env_vars enum variants must not have fields")
                .to_compile_error()
                .into();
        }
    }

    let const_defs = variants.iter().map(|v| {
        let variant_name = &v.ident;
        let docs = v
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .collect::<Vec<_>>();

        quote! {
            #(#docs)*
            pub const #variant_name: &'static str = stringify!(#variant_name);
        }
    });

    let expanded = quote! {
        struct #name;

        impl #name {
            #(#const_defs)*
        }
    };

    TokenStream::from(expanded)
}
