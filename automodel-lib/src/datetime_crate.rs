use serde::{Deserialize, Serialize};
use tokio_postgres::types::Type as PgType;

use crate::rust_type::UnsupportedTypeError;

/// Crate to use for PostgreSQL date/time type mappings in generated code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DateTimeCrate {
    /// Use [jiff](https://docs.rs/jiff) types via [jiff_sqlx](https://docs.rs/jiff-sqlx) for SQLx integration.
    Jiff,
    /// Use [time](https://docs.rs/time) types (SQLx `time` feature).
    Time,
}

impl Default for DateTimeCrate {
    fn default() -> Self {
        DateTimeCrate::Jiff
    }
}

impl DateTimeCrate {
    pub fn pg_date_type(&self) -> &'static str {
        match self {
            DateTimeCrate::Jiff => "jiff_sqlx::Date",
            DateTimeCrate::Time => "time::Date",
        }
    }

    pub fn pg_time_type(&self) -> &'static str {
        match self {
            DateTimeCrate::Jiff => "jiff_sqlx::Time",
            DateTimeCrate::Time => "time::Time",
        }
    }

    pub fn pg_timestamp_type(&self) -> &'static str {
        match self {
            DateTimeCrate::Jiff => "jiff_sqlx::DateTime",
            DateTimeCrate::Time => "time::PrimitiveDateTime",
        }
    }

    pub fn pg_timestamptz_type(&self) -> &'static str {
        match self {
            DateTimeCrate::Jiff => "jiff_sqlx::Timestamp",
            DateTimeCrate::Time => "time::OffsetDateTime",
        }
    }

    /// SQLx `PgRange` element type. SQLx 0.9 does not yet support Jiff in ranges, so Jiff mode
    /// uses `time` types for range bounds while scalar date/time columns use `jiff_sqlx`.
    pub fn pg_range_element_type(&self, wire_type: &str) -> String {
        match (self, wire_type) {
            (DateTimeCrate::Jiff, "jiff_sqlx::Date") => "time::Date".to_string(),
            (DateTimeCrate::Jiff, "jiff_sqlx::DateTime") => "time::PrimitiveDateTime".to_string(),
            (DateTimeCrate::Jiff, "jiff_sqlx::Timestamp") => "time::OffsetDateTime".to_string(),
            (_, other) => other.to_string(),
        }
    }

    /// Serde-compatible Rust type for a SQLx wire datetime type (used in composite structs).
    pub fn serde_type_for_wire_type(&self, wire_type: &str) -> Option<&'static str> {
        match (self, wire_type) {
            (DateTimeCrate::Jiff, "jiff_sqlx::Date") => Some("jiff::civil::Date"),
            (DateTimeCrate::Jiff, "jiff_sqlx::Time") => Some("jiff::civil::Time"),
            (DateTimeCrate::Jiff, "jiff_sqlx::DateTime") => Some("jiff::civil::DateTime"),
            (DateTimeCrate::Jiff, "jiff_sqlx::Timestamp") => Some("jiff::Timestamp"),
            (DateTimeCrate::Time, "time::Date") => Some("time::Date"),
            (DateTimeCrate::Time, "time::Time") => Some("time::Time"),
            (DateTimeCrate::Time, "time::PrimitiveDateTime") => Some("time::PrimitiveDateTime"),
            (DateTimeCrate::Time, "time::OffsetDateTime") => Some("time::OffsetDateTime"),
            _ => None,
        }
    }

    pub fn is_datetime_wire_type(&self, wire_type: &str) -> bool {
        self.serde_type_for_wire_type(wire_type).is_some()
    }

    pub fn needs_to_sqlx_import(&self) -> bool {
        matches!(self, DateTimeCrate::Jiff)
    }

    /// Map a simple PostgreSQL type to its Rust type name.
    pub fn simple_pg_type_rust_name(
        &self,
        pg_type: &PgType,
    ) -> Result<Option<String>, UnsupportedTypeError> {
        let maybe_rust_type = match pg_type {
            &PgType::DATE => self.pg_date_type(),
            &PgType::TIME => self.pg_time_type(),
            &PgType::TIMESTAMP => self.pg_timestamp_type(),
            &PgType::TIMESTAMPTZ => self.pg_timestamptz_type(),
            _ => return Ok(None),
        };
        Ok(Some(maybe_rust_type.to_string()))
    }
}
