/// Convert an arbitrary string (PG type name, enum variant, etc.) into a valid
/// PascalCase Rust identifier.
///
/// Splits on any non-alphanumeric character, capitalises each word, strips
/// remaining invalid chars, and prepends `_` if the result starts with a digit
/// or is empty.
pub(crate) fn to_pascal_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalise_next = true;

    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            if capitalise_next {
                for upper in ch.to_uppercase() {
                    result.push(upper);
                }
                capitalise_next = false;
            } else {
                result.push(ch);
            }
        } else {
            // Any non-alphanumeric character acts as a word boundary
            if !result.is_empty() {
                capitalise_next = true;
            }
        }
    }

    if result.is_empty() {
        return "_".to_string();
    }
    if result.starts_with(|c: char| c.is_ascii_digit()) {
        result.insert(0, '_');
    }
    result
}

/// Convert an arbitrary string (PG field name, column name, etc.) into a valid
/// snake_case Rust identifier.
///
/// Inserts `_` at camelCase boundaries, strips non-alphanumeric chars (replacing
/// them with `_`), collapses consecutive underscores, and prepends `_` if the
/// result starts with a digit or is empty.
pub(crate) fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    let mut prev_was_boundary = true; // suppress leading underscores

    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && !result.is_empty() && !prev_was_boundary {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
            prev_was_boundary = false;
        } else {
            // Any non-alphanumeric character becomes a word boundary
            if !result.is_empty() && !prev_was_boundary {
                result.push('_');
            }
            prev_was_boundary = true;
        }
    }

    // Trim trailing underscore
    while result.ends_with('_') {
        result.pop();
    }

    if result.is_empty() {
        return "_".to_string();
    }
    if result.starts_with(|c: char| c.is_ascii_digit()) {
        result.insert(0, '_');
    }
    result
}

pub(crate) fn schema_to_module_name(schema: &str) -> String {
    let sanitized: String = schema
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();

    // Ensure it starts with a letter or underscore
    if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{}", sanitized)
    } else if sanitized.is_empty() {
        "_unknown".to_string()
    } else {
        sanitized
    }
}
