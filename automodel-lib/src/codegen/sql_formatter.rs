//! Left-aligned SQL formatter (Style 1 / industry standard).
//!
//! Main clauses (`SELECT`, `FROM`, `WHERE`, …) are left-justified.
//! Sub-components are indented one level (4 spaces).
//! Comma-separated lists render one item per line with trailing commas.

const INDENT: &str = "    ";

/// Format SQL using left-aligned clauses with 4-space sub-indentation.
pub fn format_sql_style1(sql: &str) -> String {
    let sql = sql.trim();
    if sql.is_empty() {
        return String::new();
    }

    let had_semicolon = sql.ends_with(';');
    let sql = sql.trim_end_matches(';').trim();

    let upper = sql.to_ascii_uppercase();
    let formatted = if upper.starts_with("SELECT") || upper.starts_with("WITH") {
        format_select_like(sql)
    } else if upper.starts_with("UPDATE") {
        format_update(sql)
    } else if upper.starts_with("INSERT") {
        format_insert(sql)
    } else if upper.starts_with("DELETE") {
        format_delete(sql)
    } else {
        sql.to_string()
    };

    finalize_output(formatted, had_semicolon)
}

/// Format SQL for embedding in generated Rust raw string literals.
///
/// Applies Style 1 layout, then shifts every line right by one indent level so
/// clause keywords sit at 4 spaces and sub-components at 8 spaces inside the
/// raw string. The result begins with a leading newline and has no trailing newline.
pub fn format_sql_embedded(sql: &str) -> String {
    let formatted = format_sql_style1(sql);
    if formatted.is_empty() {
        return String::new();
    }

    let mut out = String::from('\n');
    for line in formatted.lines() {
        out.push_str(INDENT);
        out.push_str(line);
        out.push('\n');
    }
    while out.ends_with('\n') {
        out.pop();
    }
    out
}

fn finalize_output(mut out: String, had_semicolon: bool) -> String {
    if !had_semicolon {
        return out;
    }
    while out.ends_with('\n') {
        out.pop();
    }
    if !out.ends_with(';') {
        out.push(';');
    }
    out.push('\n');
    out
}

fn format_select_like(sql: &str) -> String {
    let (prefix, body) = if sql.to_ascii_uppercase().starts_with("WITH") {
        if let Some(select_at) = find_keyword_at_depth_zero(sql, "SELECT", 0) {
            let cte_part = sql[..select_at].trim();
            let mut out = String::new();
            out.push_str(cte_part);
            out.push('\n');
            return {
                out.push_str(&format_select_body(&sql[select_at + "SELECT".len()..]));
                out
            };
        }
        return sql.to_string();
    } else {
        ("SELECT", &sql["SELECT".len()..])
    };

    let mut out = String::from(prefix);
    out.push('\n');
    out.push_str(&format_select_body(body));
    out
}

fn format_select_body(body: &str) -> String {
    let mut out = String::new();

    let Some(from_at) = find_keyword_at_depth_zero(body, "FROM", 0) else {
        format_comma_list(&mut out, body.trim());
        return out;
    };

    format_comma_list(&mut out, body[..from_at].trim());

    out.push_str("FROM\n");
    let after_from = &body[from_at + "FROM".len()..];

    let tail_keywords = ["WHERE", "GROUP BY", "HAVING", "ORDER BY", "LIMIT", "OFFSET"];
    let tail_at = find_first_keyword_at_depth_zero(after_from, &tail_keywords);

    let (from_clause, tail) = match tail_at {
        Some(pos) => (after_from[..pos].trim(), after_from[pos..].trim()),
        None => (after_from.trim(), ""),
    };

    format_indented_lines(&mut out, from_clause, true);
    out.push_str(&format_tail_clauses(tail));
    out
}

fn format_update(sql: &str) -> String {
    let body = &sql["UPDATE".len()..];
    let mut out = String::from("UPDATE\n");

    let Some(set_at) = find_keyword_at_depth_zero(body, "SET", 0) else {
        format_indented_lines(&mut out, body.trim(), false);
        return out;
    };

    format_indented_lines(&mut out, body[..set_at].trim(), false);

    out.push_str("SET\n");
    let after_set = &body[set_at + "SET".len()..];

    let tail_keywords = ["WHERE", "RETURNING"];
    let tail_at = find_first_keyword_at_depth_zero(after_set, &tail_keywords);

    let (set_clause, tail) = match tail_at {
        Some(pos) => (after_set[..pos].trim(), after_set[pos..].trim()),
        None => (after_set.trim(), ""),
    };

    format_set_list(&mut out, set_clause);
    out.push_str(&format_tail_clauses(tail));
    out
}

fn format_insert(sql: &str) -> String {
    let upper = sql.to_ascii_uppercase();
    if upper.starts_with("INSERT INTO") {
        let body = &sql["INSERT INTO".len()..];
        let mut out = String::from("INSERT INTO\n");
        format_indented_lines(&mut out, body.trim(), false);
        return out;
    }
    sql.to_string()
}

fn format_delete(sql: &str) -> String {
    let body = if sql.to_ascii_uppercase().starts_with("DELETE FROM") {
        &sql["DELETE FROM".len()..]
    } else {
        &sql["DELETE".len()..]
    };

    let mut out = String::from("DELETE FROM\n");
    let tail_at = find_first_keyword_at_depth_zero(body, &["WHERE"]);

    match tail_at {
        Some(pos) => {
            format_indented_lines(&mut out, body[..pos].trim(), false);
            out.push_str(&format_tail_clauses(&body[pos..]));
        }
        None => format_indented_lines(&mut out, body.trim(), false),
    }
    out
}

fn format_tail_clauses(tail: &str) -> String {
    if tail.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    let mut rest = tail;

    while !rest.is_empty() {
        let rest_trimmed = rest.trim_start();
        if rest_trimmed.is_empty() {
            break;
        }

        let keyword = match find_clause_keyword(rest_trimmed) {
            Some(kw) => kw,
            None => {
                out.push_str(rest_trimmed);
                if !rest_trimmed.ends_with(';') {
                    out.push('\n');
                }
                break;
            }
        };

        let kw_len = keyword.len();
        out.push_str(keyword);
        out.push('\n');

        rest = &rest_trimmed[kw_len..];
        let next_keywords = [
            "WHERE",
            "GROUP BY",
            "HAVING",
            "ORDER BY",
            "LIMIT",
            "OFFSET",
            "RETURNING",
        ];
        let next_at = find_first_keyword_at_depth_zero(rest, &next_keywords);

        let (clause_body, remainder) = match next_at {
            Some(pos) => (rest[..pos].trim(), rest[pos..].trim_start()),
            None => (rest.trim(), ""),
        };

        match keyword {
            "WHERE" => format_and_or_list(&mut out, clause_body),
            "ORDER BY" | "GROUP BY" | "RETURNING" => format_comma_list(&mut out, clause_body),
            _ => format_indented_lines(&mut out, clause_body, false),
        }

        rest = remainder;
    }

    out
}

fn format_comma_list(out: &mut String, content: &str) {
    let content = content.trim().trim_end_matches(';');
    if content.is_empty() {
        return;
    }

    let items = split_at_depth_zero(content, ',');
    for (i, item) in items.iter().enumerate() {
        out.push_str(INDENT);
        out.push_str(item.trim());
        if i + 1 < items.len() {
            out.push(',');
        }
        out.push('\n');
    }
}

fn format_set_list(out: &mut String, content: &str) {
    let content = content.trim();
    if content.is_empty() {
        return;
    }

    for item in split_preserving_conditional_blocks(content, SplitMode::Comma) {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }
        out.push_str(INDENT);
        out.push_str(item);
        out.push('\n');
    }
}

fn format_and_or_list(out: &mut String, content: &str) {
    let content = content.trim();
    if content.is_empty() {
        return;
    }

    for item in split_preserving_conditional_blocks(content, SplitMode::AndOr) {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }
        out.push_str(INDENT);
        out.push_str(item);
        out.push('\n');
    }
}

/// Format a FROM clause, placing JOIN variants on separate indented lines.
fn format_indented_lines(out: &mut String, content: &str, split_joins: bool) {
    let content = content.trim().trim_end_matches(';');
    if content.is_empty() {
        return;
    }

    if split_joins {
        let join_keywords = [
            "LEFT OUTER JOIN",
            "RIGHT OUTER JOIN",
            "FULL OUTER JOIN",
            "INNER JOIN",
            "LEFT JOIN",
            "RIGHT JOIN",
            "FULL JOIN",
            "CROSS JOIN",
            "JOIN",
        ];

        let parts = split_by_top_level_keywords(content, &join_keywords);
        for segment in parts {
            let segment = segment.trim();
            if segment.is_empty() {
                continue;
            }
            out.push_str(INDENT);
            out.push_str(segment);
            out.push('\n');
        }
    } else {
        out.push_str(INDENT);
        out.push_str(content);
        out.push('\n');
    }
}

fn find_clause_keyword(sql: &str) -> Option<&'static str> {
    let keywords = [
        "GROUP BY",
        "ORDER BY",
        "RETURNING",
        "WHERE",
        "HAVING",
        "LIMIT",
        "OFFSET",
    ];

    for kw in keywords {
        if sql.len() >= kw.len() && sql[..kw.len()].eq_ignore_ascii_case(kw) {
            if is_word_boundary(sql, kw.len()) {
                return Some(kw);
            }
        }
    }
    None
}

fn find_first_keyword_at_depth_zero(sql: &str, keywords: &[&str]) -> Option<usize> {
    let mut best: Option<usize> = None;
    for kw in keywords {
        if let Some(pos) = find_keyword_at_depth_zero(sql, kw, 0) {
            if best.map(|p| pos < p).unwrap_or(true) {
                best = Some(pos);
            }
        }
    }
    best
}

fn find_keyword_at_depth_zero(sql: &str, keyword: &str, start: usize) -> Option<usize> {
    let upper_kw = keyword.to_ascii_uppercase();
    let bytes = sql.as_bytes();
    let mut i = start;
    let mut depth = 0;
    let mut state = ScanState::Normal;

    while i < bytes.len() {
        let remaining = &sql[i..];

        if matches!(state, ScanState::Normal) && depth == 0 {
            if remaining.len() >= upper_kw.len()
                && remaining[..upper_kw.len()].eq_ignore_ascii_case(&upper_kw)
                && is_word_boundary(sql, i + upper_kw.len())
                && (i == 0 || is_word_boundary_before(sql, i))
            {
                return Some(i);
            }
        }

        let (next_i, next_state) = advance_scan_state(sql, i, state, &mut depth);
        if next_i <= i {
            break;
        }
        i = next_i;
        state = next_state;
    }

    None
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScanState {
    Normal,
    SingleQuote,
    DoubleQuote,
    ConditionalBlock,
    Parameter,
}

fn advance_scan_state(
    sql: &str,
    i: usize,
    state: ScanState,
    depth: &mut i32,
) -> (usize, ScanState) {
    let bytes = sql.as_bytes();
    if i >= bytes.len() {
        return (i, state);
    }

    match state {
        ScanState::Normal => {
            if sql[i..].starts_with("#[") {
                return (i + 2, ScanState::ConditionalBlock);
            }
            if sql[i..].starts_with("#{") {
                return (i + 2, ScanState::Parameter);
            }
            match bytes[i] as char {
                '\'' => (i + 1, ScanState::SingleQuote),
                '"' => (i + 1, ScanState::DoubleQuote),
                '(' => {
                    *depth += 1;
                    (i + 1, ScanState::Normal)
                }
                ')' => {
                    *depth -= 1;
                    (i + 1, ScanState::Normal)
                }
                _ => (i + 1, ScanState::Normal),
            }
        }
        ScanState::SingleQuote => {
            if bytes[i] as char == '\'' {
                if i + 1 < bytes.len() && bytes[i + 1] as char == '\'' {
                    return (i + 2, ScanState::SingleQuote);
                }
                return (i + 1, ScanState::Normal);
            }
            (i + 1, ScanState::SingleQuote)
        }
        ScanState::DoubleQuote => {
            if bytes[i] as char == '"' {
                if i + 1 < bytes.len() && bytes[i + 1] as char == '"' {
                    return (i + 2, ScanState::DoubleQuote);
                }
                return (i + 1, ScanState::Normal);
            }
            (i + 1, ScanState::DoubleQuote)
        }
        ScanState::ConditionalBlock => {
            if bytes[i] as char == ']' {
                return (i + 1, ScanState::Normal);
            }
            (i + 1, ScanState::ConditionalBlock)
        }
        ScanState::Parameter => {
            if bytes[i] as char == '}' {
                return (i + 1, ScanState::Normal);
            }
            (i + 1, ScanState::Parameter)
        }
    }
}

fn is_word_boundary(sql: &str, pos: usize) -> bool {
    sql[pos..]
        .chars()
        .next()
        .map(|c| !c.is_ascii_alphanumeric() && c != '_')
        .unwrap_or(true)
}

fn is_word_boundary_before(sql: &str, pos: usize) -> bool {
    if pos == 0 {
        return true;
    }
    sql[..pos]
        .chars()
        .last()
        .map(|c| !c.is_ascii_alphanumeric() && c != '_')
        .unwrap_or(true)
}

fn split_at_depth_zero(input: &str, delimiter: char) -> Vec<String> {
    let mut items = Vec::new();
    let mut start = 0;
    let mut i = 0;
    let mut depth = 0;
    let mut state = ScanState::Normal;

    while i < input.len() {
        if state == ScanState::Normal && depth == 0 && input[i..].starts_with(delimiter) {
            items.push(input[start..i].to_string());
            i += delimiter.len_utf8();
            start = i;
            continue;
        }

        let (next_i, next_state) = advance_scan_state(input, i, state, &mut depth);
        if next_i <= i {
            break;
        }
        i = next_i;
        state = next_state;
    }

    items.push(input[start..].to_string());
    items
}

#[derive(Clone, Copy)]
enum SplitMode {
    Comma,
    AndOr,
}

fn split_preserving_conditional_blocks(content: &str, mode: SplitMode) -> Vec<String> {
    let mut segments = Vec::new();
    let mut buffer = String::new();
    let mut i = 0;

    while i < content.len() {
        if content[i..].starts_with("#[") {
            if !buffer.trim().is_empty() {
                segments.extend(split_segment_buffer(buffer.trim(), mode));
                buffer.clear();
            }
            if let Some((block, next_i)) = read_conditional_block(content, i) {
                segments.push(block);
                i = next_i;
                continue;
            }
        }

        buffer.push(content[i..].chars().next().unwrap());
        i += content[i..].chars().next().unwrap().len_utf8();
    }

    if !buffer.trim().is_empty() {
        segments.extend(split_segment_buffer(buffer.trim(), mode));
    }

    segments
}

fn split_segment_buffer(content: &str, mode: SplitMode) -> Vec<String> {
    match mode {
        SplitMode::Comma => split_at_depth_zero(content, ',')
            .into_iter()
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect(),
        SplitMode::AndOr => split_at_depth_zero_by_and_or(content),
    }
}

fn read_conditional_block(content: &str, start: usize) -> Option<(String, usize)> {
    if !content[start..].starts_with("#[") {
        return None;
    }

    let mut i = start + 2;
    let mut depth = 1;

    while i < content.len() {
        match content[i..].chars().next()? {
            '[' => {
                depth += 1;
                i += 1;
            }
            ']' => {
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return Some((content[start..i].to_string(), i));
                }
            }
            ch => i += ch.len_utf8(),
        }
    }

    None
}

fn split_at_depth_zero_by_and_or(input: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut start = 0;
    let mut i = 0;
    let mut depth = 0;
    let mut state = ScanState::Normal;

    while i < input.len() {
        if state == ScanState::Normal && depth == 0 {
            if let Some((_kw, kw_len)) = and_or_at(input, i) {
                if i > start {
                    items.push(input[start..i].trim().to_string());
                }
                start = i;
                i += kw_len;
                continue;
            }
        }

        let (next_i, next_state) = advance_scan_state(input, i, state, &mut depth);
        if next_i <= i {
            break;
        }
        i = next_i;
        state = next_state;
    }

    if start < input.len() {
        items.push(input[start..].trim().to_string());
    }

    items
}

fn and_or_at(input: &str, i: usize) -> Option<(&'static str, usize)> {
    for kw in ["AND", "OR"] {
        if input[i..].len() >= kw.len() + 1
            && input[i..][..kw.len()].eq_ignore_ascii_case(kw)
            && input.as_bytes()[i + kw.len()].is_ascii_whitespace()
            && (i == 0 || is_word_boundary_before(input, i))
        {
            return Some((kw, kw.len()));
        }
    }
    None
}

fn split_by_top_level_keywords(input: &str, keywords: &[&'static str]) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut i = 0;
    let mut depth = 0;
    let mut state = ScanState::Normal;

    while i < input.len() {
        if state == ScanState::Normal && depth == 0 {
            for kw in keywords {
                if input[i..].len() >= kw.len()
                    && input[i..][..kw.len()].eq_ignore_ascii_case(kw)
                    && is_word_boundary(input, i + kw.len())
                    && (i == 0 || is_word_boundary_before(input, i))
                {
                    let segment = input[start..i].trim();
                    if !segment.is_empty() {
                        parts.push(segment.to_string());
                    }
                    parts.push(kw.to_string());
                    i += kw.len();
                    start = i;
                    continue;
                }
            }
        }

        let (next_i, next_state) = advance_scan_state(input, i, state, &mut depth);
        if next_i <= i {
            break;
        }
        i = next_i;
        state = next_state;
    }

    let segment = input[start..].trim();
    if !segment.is_empty() {
        parts.push(segment.to_string());
    }

    if parts.is_empty() {
        return vec![input.trim().to_string()];
    }

    // Merge keyword tokens with the expression that follows them.
    let mut merged = Vec::new();
    let mut iter = parts.into_iter();
    if let Some(first) = iter.next() {
        merged.push(first);
    }
    while let Some(part) = iter.next() {
        if keywords.iter().any(|kw| part.eq_ignore_ascii_case(kw)) {
            if let Some(next) = iter.next() {
                merged.push(format!("{} {}", part, next));
            } else {
                merged.push(part);
            }
        } else {
            merged.push(part);
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_select_with_where_and_order_by() {
        let sql = "SELECT user_id, first_name, last_name, email FROM users WHERE status = 'active' AND join_date >= '2026-01-01' ORDER BY last_name ASC;";
        let formatted = format_sql_style1(sql);
        assert_eq!(
            formatted,
            "SELECT\n    user_id,\n    first_name,\n    last_name,\n    email\nFROM\n    users\nWHERE\n    status = 'active'\n    AND join_date >= '2026-01-01'\nORDER BY\n    last_name ASC;\n"
        );
    }

    #[test]
    fn formats_list_parties_query() {
        let sql = "SELECT\n    party_id,\n    party_type,\n    display_name,\n    parent_party_id,\n    created_at,\n    updated_at,\n    archived_at\nFROM public.parties\nWHERE 1=1\nAND (#{include_archived} OR archived_at IS NULL)\n#[AND party_type = #{party_type?}]\n#[AND display_name &@~ pgroonga_query_escape(#{search?})]\nORDER BY created_at DESC, party_id DESC\nLIMIT #{limit};";
        let formatted = format_sql_style1(sql);
        assert_eq!(
            formatted,
            "SELECT\n    party_id,\n    party_type,\n    display_name,\n    parent_party_id,\n    created_at,\n    updated_at,\n    archived_at\nFROM\n    public.parties\nWHERE\n    1=1\n    AND (#{include_archived} OR archived_at IS NULL)\n    #[AND party_type = #{party_type?}]\n    #[AND display_name &@~ pgroonga_query_escape(#{search?})]\nORDER BY\n    created_at DESC,\n    party_id DESC\nLIMIT\n    #{limit};\n"
        );
    }

    #[test]
    fn formats_update_with_conditional_set() {
        let sql = "UPDATE public.users SET updated_at = NOW() #[, name = #{name?}] #[, email = #{email?}] WHERE id = #{user_id} RETURNING id, name, email";
        let formatted = format_sql_style1(sql);
        assert!(formatted.starts_with("UPDATE\n    public.users\nSET\n"));
        assert!(formatted.contains("    updated_at = NOW()\n    #[, name = #{name?}]\n"));
        assert!(formatted.contains("WHERE\n    id = #{user_id}\n"));
        assert!(formatted.contains("RETURNING\n    id,\n    name,\n    email\n"));
    }

    #[test]
    fn format_sql_embedded_indents_for_raw_string_literals() {
        let sql = "SELECT party_id, party_type FROM public.parties WHERE 1=1 LIMIT 50;";
        let formatted = format_sql_embedded(sql);
        assert_eq!(
            formatted,
            "\n    SELECT\n        party_id,\n        party_type\n    FROM\n        public.parties\n    WHERE\n        1=1\n    LIMIT\n        50;"
        );
    }

    #[test]
    fn format_sql_embedded_formats_list_parties_query() {
        let sql = "SELECT\n    party_id,\n    party_type,\n    display_name,\n    parent_party_id,\n    created_at,\n    updated_at,\n    archived_at\nFROM public.parties\nWHERE 1=1\nAND (#{include_archived} OR archived_at IS NULL)\n#[AND party_type = #{party_type?}]\n#[AND display_name &@~ pgroonga_query_escape(#{search?})]\nORDER BY created_at DESC, party_id DESC\nLIMIT #{limit};";
        let formatted = format_sql_embedded(sql);
        assert!(formatted.starts_with("\n    SELECT\n        party_id,"));
        assert!(formatted.contains("\n    WHERE\n        1=1\n        AND (#{include_archived} OR archived_at IS NULL)\n        #[AND party_type = #{party_type?}]\n"));
        assert!(formatted.ends_with("\n    LIMIT\n        #{limit};"));
    }
}
