// Extracts named struct fields from data
// Panics if either the data does not represent a struct or the struct is a unit struct or tuple struct
pub fn named_struct_fields_from_data(data: syn::Data) -> Vec<syn::Field> {
    if let syn::Data::Struct(e) = data {
        if let syn::Fields::Named(e) = e.fields {
            e.named.into_iter().collect::<Vec<_>>()
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

// Extracts attributes from fields
pub fn filter_attributes_from_fields<'a>(fields: &'a Vec<syn::Field>, att_to_find: &'static str) -> Vec<&'a syn::Field> {
    fields
        .iter()
        .filter(|f| filter_attributes(&f.attrs, att_to_find))
        .collect()
}

// Extracts attributes from variants
pub fn filter_attributes_from_variants<'a>(variants: &'a Vec<syn::Variant>, att_to_find: &'static str) -> Vec<&'a syn::Variant> {
    variants
        .iter()
        .filter(|f| filter_attributes(&f.attrs, att_to_find))
        .collect()
}

pub fn filter_attributes(attrs: &Vec<syn::Attribute>, att_to_find: &str) -> bool {
    attrs
        .iter()
        .any(|attr| attr
            .path
            .segments
            .iter()
            .any(|a| {
                // Dunno why I explicitly need to mention separate variables,
                // but that's the only way the compiler is happy
                let y = a.ident.to_string();
                let r = y.as_str();

                r == att_to_find
            }))
}