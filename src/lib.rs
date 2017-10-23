//! This trait provides functionality for derivation  `IntoCDRSBytes` trait implementation
//! for underlying

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate cdrs;

use proc_macro::TokenStream;

#[proc_macro_derive(IntoCDRSBytes)]
pub fn into_cdrs_bytes(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_into_cdrs_bytes(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_into_cdrs_bytes(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    // Check if derive(HelloWorld) was specified for a struct
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
        let conver_into_bytes: quote::Tokens = fields
            .iter()
            .map(|field| field.ident.clone().unwrap())
            .map(|field| {
                quote! {
                    let field_bytes = self.#field.into_cdrs_bytes();
                    bytes.extend_from_slice(
                        Value::new_normal(field_bytes).into_cbytes().as_slice());
                }
            })
            .fold(quote!{}, |acc, line| quote!{#acc #line});

        quote! {
            impl IntoCDRSBytes for #name {
                fn into_cdrs_bytes(self) -> Bytes {
                    let mut bytes: Vec<u8> = vec![];
                    #conver_into_bytes
                    Bytes::new(bytes)
                }
            }
        }
    } else {
        //Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(IntoCDRSBytes)] is only defined for structs, not for enums!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
