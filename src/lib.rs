use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Data, DataStruct, DeriveInput, Field, Lit, Meta, NestedMeta, punctuated::Punctuated, token::Comma,
};

#[proc_macro_attribute]
pub fn field(args: TokenStream, input: TokenStream) -> TokenStream {
    let field_attr: Vec<NestedMeta> = parse_macro_input!(args as AttributeArgs);
    let input_struct: DeriveInput = parse_macro_input!(input as DeriveInput);

    let struct_name = &input_struct.ident;

    let fields: &Punctuated<Field, Comma> = if let Data::Struct(DataStruct {
        fields: syn::Fields::Named(fields),
        ..
    }) = &input_struct.data
    {
        &fields.named
    } else {
        panic!("This attribute only supports named fields in a struct");
    };

    let init_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field should have an identifier");
        let field_name_str = field_name.to_string();

        let field_init_value = field_attr
            .iter()
            .find_map(|attr| {
                if let NestedMeta::Meta(Meta::NameValue(nv)) = attr {
                    if nv.path.is_ident("field") {
                        if let Lit::Str(field_value) = &nv.lit {
                            Some(field_value)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Field '{}' is missing #[field(...)] attribute", field_name_str));

        quote! {
            #field_name: #field_init_value.into()
        }
    });

    let expanded = quote! {
        impl #struct_name {
            fn new() -> Self {
                Self {
                    #(#init_fields),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}