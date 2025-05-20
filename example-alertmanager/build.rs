use openapiv3::VersionedOpenAPI;
use serde_yaml::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Dereferences OpenAPI references in a YAML string
///
/// This function resolves `$ref` references in an OpenAPI spec by:
/// 1. Parsing the YAML into a serde_yaml::Value
/// 2. Identifying and resolving all $ref pointers (both internal and external)
/// 3. Returning the dereferenced spec as a string
fn dereference_openapi_spec(spec: &str, base_path: Option<&Path>) -> Result<String, String> {
    // Parse the YAML into a Value
    let mut yaml_value: Value =
        serde_yaml::from_str(spec).map_err(|e| format!("Failed to parse YAML: {}", e))?;

    // Create a reference to the full document for resolving references
    let root = yaml_value.clone();

    // Recursively resolve references
    dereference_value(&mut yaml_value, &root, base_path)
        .map_err(|e| format!("Failed to dereference: {}", e))?;

    // Convert back to string
    serde_yaml::to_string(&yaml_value).map_err(|e| format!("Failed to serialize YAML: {}", e))
}

/// Recursively resolves $ref references in a YAML value
fn dereference_value(
    value: &mut Value,
    root: &Value,
    base_path: Option<&Path>,
) -> Result<(), String> {
    match value {
        Value::Mapping(map) => {
            // Check if this is a reference object
            if map.len() == 1 {
                if let Some(ref_value) = map.get(&Value::String("$ref".to_string())) {
                    if let Value::String(ref_path) = ref_value {
                        // Resolve the reference
                        let resolved = resolve_reference(ref_path, root, base_path)?;
                        // Replace the current value with the resolved reference
                        *value = resolved;
                        return Ok(());
                    }
                }
            }

            // Process each key-value pair in the mapping
            let keys: Vec<Value> = map.keys().cloned().collect();
            for key in keys {
                if let Some(v) = map.get_mut(&key) {
                    dereference_value(v, root, base_path)?;
                }
            }
        }
        Value::Sequence(seq) => {
            // Process each item in the sequence
            for item in seq.iter_mut() {
                dereference_value(item, root, base_path)?;
            }
        }
        _ => {} // Scalar values don't need dereferencing
    }

    Ok(())
}

/// Resolves a reference to its value in the root document or external file
///
/// Supports both internal references (starting with #/) and external references
/// (file paths or URLs)
fn resolve_reference(
    ref_path: &str,
    root: &Value,
    base_path: Option<&Path>,
) -> Result<Value, String> {
    // Handle internal references (those starting with #/)
    if ref_path.starts_with("#/") {
        return resolve_internal_reference(ref_path, root);
    }

    // Handle external references
    let (file_path, fragment) = if let Some(hash_idx) = ref_path.find('#') {
        let (file, fragment) = ref_path.split_at(hash_idx);
        (file, Some(&fragment[1..])) // Remove the # character
    } else {
        (ref_path, None)
    };

    // Resolve the file path relative to the base path if provided
    let resolved_path = if let Some(base) = base_path {
        if Path::new(file_path).is_relative() {
            base.parent()
                .ok_or_else(|| format!("Invalid base path: {}", base.display()))?
                .join(file_path)
        } else {
            PathBuf::from(file_path)
        }
    } else {
        PathBuf::from(file_path)
    };

    // Read and parse the external file
    let external_content = fs::read_to_string(&resolved_path).map_err(|e| {
        format!(
            "Failed to read external file {}: {}",
            resolved_path.display(),
            e
        )
    })?;

    let external_value: Value = if file_path.ends_with(".json") {
        serde_json::from_str(&external_content).map_err(|e| {
            format!(
                "Failed to parse JSON from {}: {}",
                resolved_path.display(),
                e
            )
        })?
    } else {
        // Assume YAML for all other files
        serde_yaml::from_str(&external_content).map_err(|e| {
            format!(
                "Failed to parse YAML from {}: {}",
                resolved_path.display(),
                e
            )
        })?
    };

    // If there's a fragment, resolve it within the external document
    if let Some(fragment_path) = fragment {
        if fragment_path.is_empty() {
            Ok(external_value)
        } else {
            let fragment_ref = format!("#{}", fragment_path);
            resolve_internal_reference(&fragment_ref, &external_value)
        }
    } else {
        // Return the entire external document if no fragment is specified
        Ok(external_value)
    }
}

/// Resolves an internal reference (starting with #/) to its value in the document
fn resolve_internal_reference(ref_path: &str, root: &Value) -> Result<Value, String> {
    if !ref_path.starts_with("#/") {
        return Err(format!("Not an internal reference: {}", ref_path));
    }

    // Split the path into components
    let path = &ref_path[2..]; // Remove the #/ prefix
    let components: Vec<&str> = path.split('/').collect();

    // Navigate through the document following the path
    let mut current = root;
    for component in components {
        // Unescape JSON pointer escapes
        let unescaped = component.replace("~1", "/").replace("~0", "~");

        match current {
            Value::Mapping(map) => {
                current = map.get(&Value::String(unescaped.clone())).ok_or_else(|| {
                    format!(
                        "Reference path not found: {} at component {}",
                        ref_path, unescaped
                    )
                })?;
            }
            _ => {
                return Err(format!(
                    "Invalid reference path: {} at component {}",
                    ref_path, unescaped
                ));
            }
        }
    }

    // Return a clone of the resolved value
    Ok(current.clone())
}

/*/// Consolidates multiple response types into a single response type
///
/// This function modifies the OpenAPI spec to ensure each operation has at most one response type,
/// which is required by the progenitor crate.
fn consolidate_response_types(spec: &mut openapiv3::OpenAPI) {
    // Process each path
    for (_, path_item) in &mut spec.paths {
        // Process each operation in the path
        for operation in [
            &mut path_item.delete,
            &mut path_item.get,
            &mut path_item.head,
            &mut path_item.options,
            &mut path_item.patch,
            &mut path_item.post,
            &mut path_item.put,
            &mut path_item.trace,
        ]
        .iter_mut()
        .filter_map(|op| op.as_mut())
        {
            // Get the successful response types (2xx)
            let success_responses: Vec<(&String, &openapiv3::ReferenceOr<openapiv3::Response>)> =
                operation.responses.responses.iter()
                    .filter(|(code, _)| code.starts_with('2'))
                    .collect();

            // If there are multiple success responses, consolidate them
            if success_responses.len() > 1 {
                // Create a new consolidated response
                let mut consolidated_response = openapiv3::Response {
                    description: "Consolidated successful response".to_string(),
                    headers: Default::default(),
                    content: Default::default(),
                    extensions: Default::default(),
                    links: Default::default(),
                };

                // Merge all content types from all success responses
                for (_, response_ref) in success_responses {
                    if let openapiv3::ReferenceOr::Item(response) = response_ref {
                        for (content_type, media_type) in &response.content {
                            consolidated_response.content.insert(content_type.clone(), media_type.clone());
                        }
                    }
                }

                // Remove all success responses except 200
                operation.responses.responses.retain(|(code, _)| !code.starts_with('2') || code == "200");

                // Add the consolidated response as 200
                operation.responses.responses.insert(
                    "200".to_string(),
                    openapiv3::ReferenceOr::Item(consolidated_response),
                );
            }
        }
    }
}*/

fn main() {
    let spec_path = Path::new("openapi/alertmanager_openapi.yaml");
    let spec = include_str!("openapi/alertmanager_openapi.yaml");

    // Dereference the OpenAPI spec before parsing, providing the base path for external references
    let dereferenced_spec =
        match dereference_openapi_spec(spec, Some(&Path::new("src").join(spec_path))) {
            Ok(spec) => spec,
            Err(e) => {
                eprintln!("Error dereferencing OpenAPI spec: {}", e);
                // Fall back to the original spec if dereferencing fails
                spec.to_string()
            }
        };

    // Parse the dereferenced spec
    let alertmanager_openapi: openapiv3::v2::OpenAPI =
        match serde_yaml::from_str(&dereferenced_spec) {
            Ok(api) => api,
            Err(e) => {
                eprintln!("Error parsing OpenAPI spec: {}", e);
                panic!("Failed to parse OpenAPI spec");
            }
        };

    let alertmanager_openapi = VersionedOpenAPI::V2(alertmanager_openapi);
    let alertmanager_openapi_upgraded = alertmanager_openapi.upgrade();

    // Consolidate multiple response types into a single response type
    //consolidate_response_types(&mut alertmanager_openapi_upgraded);

    let alertmanager_upgraded_string =
        serde_yaml::to_string(&alertmanager_openapi_upgraded).unwrap();
    let spec = serde_yaml::from_str(&alertmanager_upgraded_string).unwrap();

    let mut generator = progenitor::Generator::default();
    let tokens = generator.generate_tokens(&spec).unwrap();
    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    let mut out_file = std::path::Path::new("src").to_path_buf();
    out_file.push("lib.rs");
    std::fs::write(out_file, content).unwrap();

    print!("{}", alertmanager_upgraded_string);
}
