use proc_macro::TokenStream;

extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields };
macro_rules! create_derive_for {
    ($trait_impl_name: ident, $trait_name: ident, $($functions: ident($($param: ident : $param_ty: ty),*) $(-> $ret_ty: ty)?),*) => { 
        paste::item! {
        #[proc_macro_derive($trait_impl_name)]
        pub fn [<$trait_impl_name:snake _impl_function>] (input: TokenStream) -> TokenStream {
            // Parse the input tokens into a syntax tree
            let original = input.clone();
            let input = parse_macro_input!(input as DeriveInput);

            // Get the struct identifier
            let struct_name = &input.ident;

            // Get the fields of the struct
            let fields = match input.data {
                Data::Struct(ref data) => match data.fields {
                    Fields::Named(ref fields) => &fields.named,
                    _ => return quote!().into(),
                },
                _ => return quote!().into(),
            };

            // Iterate through the fields to find the first one implementing EntityTrait
            let field = fields.iter().next();
            if let None = field {
                return original;
            }
            let field = field.unwrap();
            let field_ident = &field.ident;
            let implementation = quote! {
                impl $trait_name for #struct_name {
                    $(
                    fn $functions(&self$(, $param: $param_ty)*) $(-> $ret_ty)? {
                        self.#field_ident.$functions($($param,)*)
                    })*
                }
            };
            // for debuging: 
            // println!("{}", implementation);

            // Generate the output
            let expanded = implementation;

            // Return the generated implementation as a token stream
            TokenStream::from(expanded)
        }
        }
    };
}
create_derive_for!(TestEntityTraitImpl, EntityTraitTestTrait, do_something() -> &str, do_more() -> &str, add(a: u8, b: u8) -> u8);
/*
trait EntityFunctions {
    fn get_states(&self) -> &[EntityState];
    fn add_state(&mut self, state: EntityState);
    fn remove_state(&mut self, state: EntityState);
    fn get_air_ticks(&self) -> i32;
    fn set_air_ticks(&mut self, value: i32);
    fn increment_air_ticks(&mut self);
    fn get_custom_name(&self) -> Option<&nbt_lib::datatypes::TextComponent>;
    fn set_custom_name(&mut self, value: Option<nbt_lib::datatypes::TextComponent>);
    fn is_custom_name_visible(&self) -> bool;
    fn set_custom_name_visible(&mut self, value: bool);
    fn is_silent(&self) -> bool;
    fn set_silent(&mut self, value: bool);
    fn has_no_gravity(&self) -> bool;
    fn set_no_gravity(&mut self, value: bool);
    fn get_pose(&self) -> PoseEnum;
    fn set_pose(&mut self, value: PoseEnum);
    fn get_ticks_frozen_in_powdered_snow(&self) -> i32;
    fn set_ticks_frozen_in_powdered_snow(&mut self, value: i32);
    fn increment_ticks_frozen_in_powdered_snow(&mut self);
}
*/
create_derive_for!(EntityTraitImpl, EntityFunctions,
    get_states() -> &[EntityState],
    add_state(state: EntityState)
);
