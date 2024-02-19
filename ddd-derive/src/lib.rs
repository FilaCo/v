use proc_macro::TokenStream;

mod entity;
mod vo;

#[proc_macro_derive(VO, attributes(vo))]
pub fn derive_vo(input: TokenStream) -> TokenStream {
    vo::derive(input)
}

#[proc_macro_derive(Entity, attributes(entity))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    entity::derive(input)
}
