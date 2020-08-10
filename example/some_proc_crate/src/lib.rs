use proc_macro::TokenStream;
use syn::DeriveInput;

// This proc macro is only used by the example folder for struct SomeStruct
#[proc_macro_derive(SomeProcCrate, attributes(someattr))]
pub fn entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let fields = proc_macro2_helper::named_struct_fields_from_data(input.data);
    let attr_fields = proc_macro2_helper::filter_attributes(&fields, "someattr");

    // Only 2 attributes have an attribute 'someattr'
    assert_eq!(2, attr_fields.len());

    TokenStream::new()
}