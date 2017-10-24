//! This trait provides functionality for derivation  `IntoCDRSBytes` trait implementation
//! for underlying

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(IntoCDRSValue)]
pub fn into_cdrs_value(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_into_cdrs_value(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_into_cdrs_value(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
        let conver_into_bytes: quote::Tokens = fields
            .iter()
            .map(|field| field.ident.clone().unwrap())
            .map(|field| {
                quote! {
                    let field_value = self.#field.into_cdrs_value();
                    bytes.extend_from_slice(field_value.into_cbytes().as_slice());
                }
            })
            .fold(quote!{}, |acc, line| quote!{#acc #line});

        quote! {
            impl IntoCDRSValue for #name {
                fn into_cdrs_value(self) -> Value {
                    let mut bytes: Vec<u8> = vec![];
                    #conver_into_bytes
                    Bytes::new(bytes).into()
                }
            }
        }
    } else {
        panic!("#[derive(IntoCDRSValue)] is only defined for structs, not for enums!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
