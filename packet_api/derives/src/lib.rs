extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(TestNeccessaryTrait)]
pub fn test_neccessary_trait_derive(input: TokenStream) -> TokenStream {
    let input0 = syn::parse_macro_input!(input as syn::DeriveInput);
    let gen = generate_tntd(&input0);
    gen
}
fn generate_tntd(input0: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input0.ident;
    let field = if let syn::Data::Struct(ref data_struct) = input0.data {
        if let Some(field) = data_struct.fields.iter().next() {
            (&field.ident ,&field.ty)
        } else {
            return syn::Error::new_spanned(input0, "could not get the datatype of the first element").to_compile_error().into();
        }
    } else {
        return syn::Error::new_spanned(input0, "Only structs are allowed to have this trait").to_compile_error().into();
    };
    let field_name = field.0;
    let field_type = field.1;
    if let Some(field_name) = field_name {
        quote! {
            impl TestNeccessaryTrait for #struct_name {
                type Value = #field_type;
                fn new(value: Self::Value) -> Self {
                    Self { #field_name: value }
                }
                fn get_value(&self) -> &Self::Value {
                    &self.#field_name
                }
            }
        }.into()
    } else {
        quote! {
            impl TestNeccessaryTrait for #struct_name {
            type Value = #field_type;
            fn new(value: Self::Value) -> Self {
            Self(value)
            }
            fn get_value(&self) -> &Self::Value {
            &self.0
            }
            }
        }.into()
    }
}
