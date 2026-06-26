use std::collections::HashMap;

/// A domain type whose CHECK constraint restricts values to a fixed set of string literals.
#[derive(Debug, Clone)]
pub struct DomainEnumConstraint {
    pub variants: Vec<String>,
    /// PostgreSQL base type name used on the wire (e.g. `text` for `CREATE DOMAIN ... AS TEXT`).
    pub base_type: String,
}

/// PostgreSQL text-like base types that can back a domain enum.
fn is_text_like_base_type(base_type: &str) -> bool {
    matches!(base_type, "text" | "varchar" | "bpchar" | "name" | "citext")
}

/// Extract string literals from a `CHECK ... IN (...)` or `= ANY (ARRAY[...])` constraint.
pub fn parse_check_in_literals(constraint_def: &str) -> Option<Vec<String>> {
    let normalized = constraint_def.to_ascii_lowercase();

    if let Some(in_clause) = extract_in_clause(&normalized, constraint_def) {
        let literals = extract_string_literals(in_clause);
        if literals.len() >= 2 {
            return Some(literals);
        }
    }

    if let Some(array_clause) = extract_any_array_clause(&normalized, constraint_def) {
        let literals = extract_string_literals(array_clause);
        if literals.len() >= 2 {
            return Some(literals);
        }
    }

    None
}

fn extract_in_clause<'a>(normalized: &str, original: &'a str) -> Option<&'a str> {
    let in_pos = normalized.find(" in (")?;
    let open_paren = in_pos + 4; // position of '(' in " in ("
    let close_paren = find_matching_paren(original, open_paren)?;
    Some(&original[open_paren + 1..close_paren])
}

fn extract_any_array_clause<'a>(normalized: &str, original: &'a str) -> Option<&'a str> {
    let any_pos = normalized.find("= any (")?;
    let search_from = any_pos + "= any (".len();
    let array_pos = normalized[search_from..].find("array[")?;
    let open_bracket = search_from + array_pos + "array".len(); // points at '['
    let close_bracket = find_matching_bracket(original, open_bracket)?;
    Some(&original[open_bracket + 1..close_bracket])
}

fn find_matching_paren(s: &str, open: usize) -> Option<usize> {
    if s.as_bytes().get(open) != Some(&b'(') {
        return None;
    }
    let mut depth = 0;
    for (i, ch) in s[open..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + i);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_matching_bracket(s: &str, open: usize) -> Option<usize> {
    if s.as_bytes().get(open) != Some(&b'[') {
        return None;
    }
    let mut depth = 0;
    for (i, ch) in s[open..].char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Extract single-quoted string literals from a SQL fragment, preserving original casing.
fn extract_string_literals(fragment: &str) -> Vec<String> {
    let mut literals = Vec::new();
    let bytes = fragment.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'\'' {
            i += 1;
            let mut value = String::new();
            while i < bytes.len() {
                if bytes[i] == b'\'' {
                    if bytes.get(i + 1) == Some(&b'\'') {
                        value.push('\'');
                        i += 2;
                        continue;
                    }
                    literals.push(value);
                    i += 1;
                    break;
                }
                value.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }
        i += 1;
    }

    literals
}

/// Fetch domain types whose CHECK constraint restricts values to a fixed set of string literals.
pub async fn fetch_domain_enum_constraints(
    client: &tokio_postgres::Client,
) -> Result<HashMap<String, DomainEnumConstraint>, Box<dyn std::error::Error>> {
    let rows = client
        .query(
            r#"
            SELECT
                n.nspname AS schema_name,
                t.typname AS domain_name,
                bt.typname AS base_type,
                pg_get_constraintdef(c.oid) AS constraint_def
            FROM pg_constraint c
            JOIN pg_type t ON c.contypid = t.oid
            JOIN pg_namespace n ON t.typnamespace = n.oid
            JOIN pg_type bt ON t.typbasetype = bt.oid
            WHERE c.contype = 'c'
              AND t.typtype = 'd'
            "#,
            &[],
        )
        .await?;

    let mut domain_enums: HashMap<String, DomainEnumConstraint> = HashMap::new();

    for row in rows {
        let schema: String = row.get(0);
        let domain_name: String = row.get(1);
        let base_type: String = row.get(2);
        let constraint_def: String = row.get(3);

        if !is_text_like_base_type(&base_type) {
            continue;
        }

        let Some(variants) = parse_check_in_literals(&constraint_def) else {
            continue;
        };

        let key = format!("{}.{}", schema, domain_name);
        let constraint = DomainEnumConstraint {
            variants: variants.clone(),
            base_type: base_type.clone(),
        };
        domain_enums
            .entry(key)
            .and_modify(|existing| {
                if existing.variants.is_empty() {
                    *existing = constraint.clone();
                }
            })
            .or_insert(constraint);
    }

    Ok(domain_enums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_value_in_syntax() {
        let def = "CHECK ((VALUE)::text = ANY ((ARRAY['low'::text, 'medium'::text, 'high'::text])::text[]))";
        assert_eq!(
            parse_check_in_literals(def),
            Some(vec![
                "low".to_string(),
                "medium".to_string(),
                "high".to_string()
            ])
        );
    }

    #[test]
    fn parses_any_array_syntax() {
        let def = "CHECK ((value = ANY (ARRAY['a'::text, 'b'::text, 'c'::text])))";
        assert_eq!(
            parse_check_in_literals(def),
            Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        );
    }

    #[test]
    fn parses_in_parentheses_syntax() {
        let def = "CHECK (VALUE IN ('draft', 'published', 'archived'))";
        assert_eq!(
            parse_check_in_literals(def),
            Some(vec![
                "draft".to_string(),
                "published".to_string(),
                "archived".to_string()
            ])
        );
    }

    #[test]
    fn ignores_numeric_range_checks() {
        let def = "CHECK ((VALUE > 0))";
        assert_eq!(parse_check_in_literals(def), None);
    }

    #[test]
    fn ignores_regex_checks() {
        let def = "CHECK ((VALUE ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$'::text))";
        assert_eq!(parse_check_in_literals(def), None);
    }

    #[test]
    fn requires_at_least_two_variants() {
        let def = "CHECK (VALUE IN ('only_one'))";
        assert_eq!(parse_check_in_literals(def), None);
    }
}
