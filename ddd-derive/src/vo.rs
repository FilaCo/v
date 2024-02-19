use darling::ast::{Data, Fields};
use darling::util::Ignored;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Generics, Ident};

#[derive(darling::FromDeriveInput)]
#[darling(attributes(vo), supports(struct_named))]
struct VO {
    ident: Ident,
    generics: Generics,
    data: Data<Ignored, VOField>,
}

#[derive(darling::FromMeta)]
struct EqMarker;

#[derive(darling::FromField)]
#[darling(attributes(value_object))]
struct VOField {
    ident: Option<Ident>,
    eq: Option<EqMarker>,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let VO {
        ident,
        generics,
        data,
        ..
    } = match VO::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let fields = data.take_struct().unwrap();

    derive_vo(ident, generics, fields)
}

fn derive_vo(ident: Ident, generics: Generics, fields: Fields<VOField>) -> TokenStream {
    let fields = fields
        .into_iter()
        .map(|f| (f.eq, f.ident.as_ref().map(|ident| quote!(#ident)).unwrap()))
        .collect::<Vec<_>>();

    let field = fields.iter().map(|(_, f)| f);
    let eq_field = fields.iter().filter_map(|(eq, f)| eq.as_ref().map(|_| f));

    quote! {
        impl #generics ddd::VO for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                Self {
                    #(#field: self.#field.clone(),)*
                }
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                true #( && self.#eq_field == other.#eq_field)*
            }
        }
    }
    .into()
}
