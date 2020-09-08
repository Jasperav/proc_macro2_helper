use proc_macro::TokenStream;
use syn::DeriveInput;

// This proc macro is only used by the example folder for struct SomeStruct
#[proc_macro_derive(SomeProcCrateStruct, attributes(someattr))]
pub fn for_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let fields = proc_macro2_helper::named_struct_fields_from_data(input.data);
    let attrs = proc_macro2_helper::filter_attributes_from_fields(&fields, "someattr");

    // Only 2 attributes have an attribute 'someattr'
    assert_eq!(2, attrs.len());

    TokenStream::new()
}

// This proc macro is only used by the example folder for enum SomeEnum
#[proc_macro_derive(SomeProcCrateEnum, attributes(someattr))]
pub fn for_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let variants = proc_macro2_helper::enum_data(input.data);
    let attrs = proc_macro2_helper::filter_attributes_from_variants(&variants, "someattr");

    // Only 2 attributes have an attribute 'someattr'
    assert_eq!(2, attrs.len());

    TokenStream::new()
}