use crate::{
    rust_type::{InputParam, OutputColumn},
    utils::{to_pascal_case, to_snake_case},
};

/// Strip input parameter suffixes (??[?], ?[?], ??, [?], ?) from a raw parameter name
pub fn strip_input_suffix(name: &str) -> String {
    let s = if name.ends_with("[?]") {
        &name[..name.len() - 3]
    } else {
        name
    };
    if s.ends_with("??") {
        s[..s.len() - 2].to_string()
    } else if s.ends_with('?') {
        s[..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Build derive attribute string from a list of custom derives and default derives
/// Returns a string like "#[derive(Debug, Clone, Serialize, Deserialize)]"
fn build_derive_attribute(default_derives: &[&str], custom_derives: &[String]) -> String {
    let mut all_derives = default_derives
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    all_derives.extend(custom_derives.iter().cloned());

    // Remove duplicates while preserving order
    let mut seen = std::collections::HashSet::new();
    all_derives.retain(|d| seen.insert(d.clone()));

    format!("#[derive({})]", all_derives.join(", "))
}

/// Generate function parameter list with custom parameter names
pub fn generate_input_params_with_names(
    input_types: &[InputParam],
    param_names: &[String],
) -> String {
    if input_types.is_empty() {
        return String::new();
    }

    // Build a map of unique parameter names to their types
    let mut unique_params: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut param_order: Vec<String> = Vec::new();

    for (i, rust_type) in input_types.iter().enumerate() {
        let default_name = format!("param_{}", i + 1);
        let raw_param_name = param_names.get(i).unwrap_or(&default_name);

        // Strip all suffixes (??[?], ?[?], ??, [?], ?) for parameter names
        let clean_param_name = strip_input_suffix(raw_param_name);

        // Only add if we haven't seen this parameter name before
        if !unique_params.contains_key(&clean_param_name) {
            let final_type = if rust_type.is_optional && rust_type.is_nullable {
                // ?? suffix: Option<Option<T>> for conditional + nullable
                format!("Option<Option<{}>>", rust_type.rust_type())
            } else if rust_type.is_nullable || rust_type.is_optional {
                format!("Option<{}>", rust_type.rust_type())
            } else {
                rust_type.rust_type().to_string()
            };
            unique_params.insert(clean_param_name.clone(), final_type);
            param_order.push(clean_param_name);
        }
    }

    // Generate the parameter list in the order we first encountered each parameter
    param_order
        .iter()
        .map(|param_name| {
            let param_type = unique_params.get(param_name).unwrap();
            format!("{}: {}", param_name, param_type)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Generate function parameter for multiunzip pattern
/// Returns a single parameter that is a Vec of structs containing all the input types
/// For example: items: Vec<InsertUsersBatchRecord>
pub fn generate_multiunzip_param(query_name: &str, param_name: &str) -> String {
    let struct_name = format!("{}Record", to_pascal_case(query_name));
    format!("{}: Vec<{}>", param_name, struct_name)
}

/// Generate return type for single column results or empty results
pub fn generate_return_type(output_column: Option<&OutputColumn>) -> String {
    match output_column {
        None => "()".to_string(),
        Some(col) => {
            if col.is_nullable {
                format!("Option<{}>", col.rust_type())
            } else {
                col.rust_type().to_string()
            }
        }
    }
}

/// Generate a result struct with a custom struct name
pub fn generate_result_struct_with_name(
    struct_name: &str,
    output_types: &[OutputColumn],
    custom_derives: &[String],
    _emitted_struct_names: &mut std::collections::HashSet<String>,
) -> Option<String> {
    if output_types.is_empty() {
        return None;
    }

    let mut all_structs = String::new();

    // Generate the main result struct
    let derive_attr = build_derive_attribute(&["Debug", "Clone"], custom_derives);
    all_structs.push_str(&format!("{}\npub struct {} {{\n", derive_attr, struct_name));

    for col in output_types {
        let field_type = if col.is_nullable {
            format!("Option<{}>", col.rust_type())
        } else {
            col.rust_type().to_string()
        };
        all_structs.push_str(&format!("    pub {}: {},\n", col.rust_name, field_type));
    }

    all_structs.push_str("}\n");
    Some(all_structs)
}

/// Generate an input struct for multiunzip pattern
/// Creates a struct with fields matching the parameter names and types
pub fn generate_multiunzip_input_struct(
    query_name: &str,
    param_names: &[String],
    input_types: &[InputParam],
    custom_derives: &[String],
) -> Option<String> {
    if input_types.is_empty() {
        return None;
    }

    let struct_name = format!("{}Record", to_pascal_case(query_name));
    let derive_attr = build_derive_attribute(&["Debug", "Clone"], custom_derives);
    let mut struct_def = format!("{}\npub struct {} {{\n", derive_attr, struct_name);

    for (i, param_name) in param_names.iter().enumerate() {
        if let Some(rust_type) = input_types.get(i) {
            // Extract base type from Vec<T> for array parameters
            // But NOT for JSON-wrapped types where Vec<T> is the actual value type (e.g., JSONB arrays)
            let effective = rust_type.rust_type();
            let base_type = if !rust_type.needs_json_wrapper
                && effective.starts_with("Vec<")
                && effective.ends_with('>')
            {
                &effective[4..effective.len() - 1]
            } else {
                effective
            };

            let field_type = if rust_type.is_nullable || rust_type.is_optional {
                format!("Option<{}>", base_type)
            } else {
                base_type.to_string()
            };

            struct_def.push_str(&format!(
                "    pub {}: {},\n",
                to_snake_case(param_name),
                field_type
            ));
        }
    }

    struct_def.push_str("}\n");
    Some(struct_def)
}

/// Generate struct for conditions_type pattern
/// This creates a struct with ONLY the conditional parameters (those ending with '?')
/// preserving their nullable types to support setting values to NULL (e.g., age: Some(40) -> None)
pub fn generate_conditional_diff_struct(
    struct_name: &str,
    param_names: &[String],
    input_types: &[InputParam],
    custom_derives: &[String],
) -> Option<String> {
    if input_types.is_empty() {
        return None;
    }

    let mut code = String::new();

    let derive_attr = build_derive_attribute(&["Debug", "Clone", "PartialEq"], custom_derives);
    code.push_str(&derive_attr);
    code.push('\n');
    code.push_str(&format!("pub struct {} {{\n", struct_name));

    // Build a map of unique parameter names to their types
    // Only include conditional parameters (those with '?')
    let mut unique_params: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut param_order: Vec<String> = Vec::new();

    for (i, rust_type) in input_types.iter().enumerate() {
        let default_name = format!("param_{}", i + 1);
        let raw_param_name = param_names.get(i).unwrap_or(&default_name);

        // Only process conditional parameters (those ending with '?' or '??')
        if raw_param_name.ends_with('?') {
            // Strip all suffixes
            let clean_param_name = strip_input_suffix(raw_param_name);

            // Only add if we haven't seen this parameter name before
            if !unique_params.contains_key(&clean_param_name) {
                // For conditions_type, preserve nullable types to support setting values to NULL
                let final_type = if rust_type.is_nullable {
                    format!("Option<{}>", rust_type.rust_type())
                } else {
                    rust_type.rust_type().to_string()
                };
                unique_params.insert(clean_param_name.clone(), final_type);
                param_order.push(clean_param_name);
            }
        }
    }

    // If no conditional parameters found, don't generate the struct
    if param_order.is_empty() {
        return None;
    }

    // Generate struct fields
    for param_name in &param_order {
        let param_type = unique_params.get(param_name).unwrap();
        code.push_str(&format!("    pub {}: {},\n", param_name, param_type));
    }

    code.push_str("}\n");

    Some(code)
}

/// Generate function parameters for conditions_type pattern
/// Returns: "old: &QueryNameParams, new: &QueryNameParams, non_conditional_params..."
pub fn generate_conditional_diff_params(
    query_name: &str,
    param_names: &[String],
    input_types: &[InputParam],
    struct_name_override: Option<&str>,
) -> String {
    let struct_name = if let Some(override_name) = struct_name_override {
        override_name.to_string()
    } else {
        format!("{}Params", to_pascal_case(query_name))
    };

    // Separate conditional and non-conditional parameters
    let mut non_conditional_params = Vec::new();

    for (i, param_name) in param_names.iter().enumerate() {
        // Only include non-conditional parameters (those without '?')
        if !param_name.ends_with('?') {
            if let Some(rust_type) = input_types.get(i) {
                let final_type = if rust_type.is_nullable {
                    format!("Option<{}>", rust_type.rust_type())
                } else {
                    rust_type.rust_type().to_string()
                };
                non_conditional_params.push(format!(
                    "{}: {}",
                    strip_input_suffix(param_name),
                    final_type
                ));
            }
        }
    }

    // Build parameter string - old and new structs, then non-conditional params
    let mut params = vec![
        format!("old: &{}", struct_name),
        format!("new: &{}", struct_name),
    ];

    params.extend(non_conditional_params);

    params.join(", ")
}

/// Generate a struct for structured parameters pattern
/// Returns: "pub struct QueryNameParams { pub param1: Type1, pub param2: Type2, ... }"
pub fn generate_structured_params_struct(
    query_name: &str,
    param_names: &[String],
    input_types: &[InputParam],
    custom_derives: &[String],
) -> Option<String> {
    if input_types.is_empty() {
        return None;
    }

    let struct_name = format!("{}Params", to_pascal_case(query_name));
    let mut code = String::new();

    let derive_attr = build_derive_attribute(&["Debug", "Clone"], custom_derives);
    code.push_str(&derive_attr);
    code.push('\n');
    code.push_str(&format!("pub struct {} {{\n", struct_name));

    // Build a map of unique parameter names to their types
    let mut unique_params: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut param_order: Vec<String> = Vec::new();

    for (i, rust_type) in input_types.iter().enumerate() {
        let default_name = format!("param_{}", i + 1);
        let param_name = param_names.get(i).unwrap_or(&default_name);

        // Clean parameter name (remove suffixes if present)
        let clean_param_name = strip_input_suffix(param_name);

        // Only add if we haven't seen this parameter name before
        if !unique_params.contains_key(&clean_param_name) {
            // Use the type as-is (including Option wrapper if nullable or conditional)
            let final_type = if rust_type.is_nullable || rust_type.is_optional {
                format!("Option<{}>", rust_type.rust_type())
            } else {
                rust_type.rust_type().to_string()
            };
            unique_params.insert(clean_param_name.clone(), final_type);
            param_order.push(clean_param_name);
        }
    }

    // Generate struct fields
    for param_name in &param_order {
        let param_type = unique_params.get(param_name).unwrap();
        code.push_str(&format!("    pub {}: {},\n", param_name, param_type));
    }

    code.push_str("}\n");

    Some(code)
}

/// Generate function parameters for parameters_type pattern
/// Returns: "params: &QueryNameParams" or "params: &OverrideName" if override is provided
pub fn generate_structured_params_signature(
    query_name: &str,
    struct_name_override: Option<&str>,
) -> String {
    let struct_name = if let Some(override_name) = struct_name_override {
        override_name.to_string()
    } else {
        format!("{}Params", to_pascal_case(query_name))
    };
    format!("params: &{}", struct_name)
}
