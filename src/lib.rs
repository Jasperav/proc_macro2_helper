use syn::{Attribute, Data, Variant};

pub fn enum_data(data: Data) -> Vec<Variant> {
    match data {
        Data::Enum(e) => e.variants.into_iter().collect(),
        _ => panic!("Expected enum"),
    }
}

/// Extracts named struct fields from data
/// Panics if either the data does not represent a struct or the struct is a unit struct or tuple struct
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

pub fn contains_ident(fields: &[syn::Field], ident: &str) -> bool {
    fields
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .any(|i| *i == ident)
}

/// Extracts attributes from fields
pub fn filter_attributes_from_fields<'a>(
    fields: &'a [syn::Field],
    att_to_find: &'static str,
) -> Vec<&'a syn::Field> {
    fields
        .iter()
        .filter(|f| attributes_contains(&f.attrs, att_to_find))
        .collect()
}

/// Extracts attributes from variants
pub fn filter_attributes_from_variants<'a>(
    variants: &'a [syn::Variant],
    att_to_find: &'static str,
) -> Vec<&'a syn::Variant> {
    variants
        .iter()
        .filter(|f| attributes_contains(&f.attrs, att_to_find))
        .collect()
}

pub fn attributes_contains(attrs: &[syn::Attribute], att_to_find: &str) -> bool {
    attrs.iter().any(|a| attribute_contains(a, att_to_find))
}

pub fn attribute_contains(attr: &Attribute, att_to_find: &str) -> bool {
    attr.path.segments.iter().any(|a| {
        // Dunno why I explicitly need to mention separate variables,
        // but that's the only way the compiler is happy
        let y = a.ident.to_string();
        let r = y.as_str();

        r == att_to_find
    })
}
