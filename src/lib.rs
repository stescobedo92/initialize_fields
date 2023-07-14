use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Data, DataStruct, DeriveInput, Field, Lit, Meta, NestedMeta,
};

#[proc_macro_attribute]
pub fn field(args: TokenStream, input: TokenStream) -> TokenStream {

}