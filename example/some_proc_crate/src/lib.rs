use proc_macro::TokenStream;
use syn::DeriveInput;

// This proc macro is only used by the example folder for struct SomeStruct
#[proc_macro_derive(SomeProcCrateStruct, attributes(someattr))]
pub fn for_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let fields = proc_macro2_helper::named_struct_fields_from_data(input.data);
    let attrs = proc_macro2_helper::filter_attributes_from_fields(&fields, "someattr");

    // Only 1 field should have the option type
    let options = fields
        .iter()
        .filter(|field| proc_macro2_helper::has_first_type_option(field))
        .count();

    assert_eq!(1, options);

    // Only 2 attributes have an attribute 'someattr'
    assert_eq!(2, attrs.len());
    assert!(proc_macro2_helper::contains_ident(&fields, "field0"));
    assert!(!proc_macro2_helper::contains_ident(&fields, "this field does not exists"));

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