// Copyright 2022 Oxide Computer Company

use std::collections::BTreeMap;

use indexmap::IndexMap;
use openapiv3::{Components, Parameter, ReferenceOr, RequestBody, Response, Schema};
use unicode_ident::{is_xid_continue, is_xid_start};

use crate::Result;

/*pub trait ReferenceOrExt<T> {
    fn item(&self, components: &Option<Components>) -> Result<&T>;
}*/

pub(crate) trait ReferenceOrExt<T: ComponentLookup> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T>;
}

/*impl<T> ReferenceOrExt<T> for ReferenceOr<T> {
    fn item(&self, components: &Option<Components>) -> Result<&T> {
        match self {
            ReferenceOr::Item(item) => Ok(item),
            ReferenceOr::Reference { reference } => {
                // Handle references more robustly
                let components = components.as_ref().ok_or_else(|| {
                    Error::UnexpectedFormat(format!(
                        "reference {} without components",
                        reference
                    ))
                })?;

                // Extract the reference path
                let parts: Vec<&str> = reference.split('/').collect();
                if parts.len() != 4 || parts[0] != "#" || parts[1] != "components" {
                    return Err(Error::UnexpectedFormat(format!(
                        "invalid reference format: {}",
                        reference
                    )));
                }

                // Get the component type and name
                let component_type = parts[2];
                let component_name = parts[3];

                // Look up the component based on its type
                match component_type {
                    "schemas" => {
                        if let Some(schema) = components.schemas.get(component_name) {
                            // This is a type conversion that won't work directly
                            // We need to handle this case specially based on T
                            return Err(Error::UnexpectedFormat(format!(
                                "reference to schema {} not supported in this context",
                                reference
                            )));
                        }
                    }
                    "parameters" => {
                        if let Some(param) = components.parameters.get(component_name) {
                            // Similar issue with type conversion
                            return Err(Error::UnexpectedFormat(format!(
                                "reference to parameter {} not supported in this context",
                                reference
                            )));
                        }
                    }
                    "requestBodies" => {
                        if let Some(body) = components.request_bodies.get(component_name) {
                            // Similar issue with type conversion
                            return Err(Error::UnexpectedFormat(format!(
                                "reference to request body {} not supported in this context",
                                reference
                            )));
                        }
                    }
                    "responses" => {
                        if let Some(response) = components.responses.get(component_name) {
                            // Similar issue with type conversion
                            return Err(Error::UnexpectedFormat(format!(
                                "reference to response {} not supported in this context",
                                reference
                            )));
                        }
                    }
                    _ => {}
                }

                Err(Error::UnexpectedFormat(format!(
                    "reference not found: {}",
                    reference
                )))
            }
        }
    }
}*/

pub(crate) trait ComponentLookup: Sized {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>>;
}

impl<T: ComponentLookup> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T> {
        match self {
            ReferenceOr::Item(item) => Ok(item),
            ReferenceOr::Reference { reference } => {
                let idx = reference.rfind('/').unwrap();
                let key = &reference[idx + 1..];
                let parameters = T::get_components(components.as_ref().unwrap());
                parameters
                    .get(key)
                    .unwrap_or_else(|| panic!("key {} is missing", key))
                    .item(components)
            }
        }
    }
}

pub(crate) fn items<'a, T>(
    refs: &'a [ReferenceOr<T>],
    components: &'a Option<Components>,
) -> impl Iterator<Item = Result<&'a T>>
where
    T: ComponentLookup,
{
    refs.iter().map(|r| r.item(components))
}

pub(crate) fn parameter_map<'a>(
    refs: &'a [ReferenceOr<Parameter>],
    components: &'a Option<Components>,
) -> Result<BTreeMap<&'a String, &'a Parameter>> {
    items(refs, components)
        .map(|res| res.map(|param| (&param.parameter_data_ref().name, param)))
        .collect()
}

impl ComponentLookup for Parameter {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.parameters
    }
}

impl ComponentLookup for RequestBody {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.request_bodies
    }
}

impl ComponentLookup for Response {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.responses
    }
}

impl ComponentLookup for Schema {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.schemas
    }
}

pub(crate) enum Case {
    Pascal,
    Snake,
}

pub(crate) fn sanitize(input: &str, case: Case) -> String {
    use heck::{ToPascalCase, ToSnakeCase};
    let to_case = match case {
        Case::Pascal => str::to_pascal_case,
        Case::Snake => str::to_snake_case,
    };
    // If every case was special then none of them would be.
    let out = match input {
        "+1" => "plus1".to_string(),
        "-1" => "minus1".to_string(),
        _ => to_case(
            &input
                .replace('\'', "")
                .replace(|c: char| !is_xid_continue(c), "-"),
        ),
    };

    let out = match out.chars().next() {
        None => to_case("x"),
        Some(c) if is_xid_start(c) => out,
        Some(_) => format!("_{}", out),
    };

    // Make sure the string is a valid Rust identifier.
    if syn::parse_str::<syn::Ident>(&out).is_ok() {
        out
    } else {
        format!("{}_", out)
    }
}

/// Given a desired name and a slice of proc_macro2::Ident, generate a new
/// Ident that is unique from the slice.
pub(crate) fn unique_ident_from(
    name: &str,
    identities: &[proc_macro2::Ident],
) -> proc_macro2::Ident {
    let mut name = name.to_string();

    loop {
        let ident = quote::format_ident!("{}", name);

        if !identities.contains(&ident) {
            return ident;
        }

        name.insert_str(0, "_");
    }
}
