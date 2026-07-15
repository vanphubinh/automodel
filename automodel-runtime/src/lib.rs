//! Shared persistence error types used by AutoModel-generated query functions.
//!
//! Generated bounded-context crates depend on this crate instead of inlining
//! identical error definitions into every `persistence/mod.rs`.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorConstraintInfo {
    /// Name of the violated constraint, or empty if the driver did not report one.
    pub constraint_name: String,
    /// Table associated with the constraint, or empty if the driver did not report one.
    pub table_name: String,
    pub kind: ErrorConstraintKind,
}

impl std::fmt::Display for ErrorConstraintInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let constraint = if self.constraint_name.is_empty() {
            "<unknown>"
        } else {
            self.constraint_name.as_str()
        };
        let table = if self.table_name.is_empty() {
            "<unknown>"
        } else {
            self.table_name.as_str()
        };
        write!(f, "{constraint} on table {table} ({})", self.kind)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorConstraintKind {
    UniqueViolation,
    ForeignKeyViolation,
    NotNullViolation,
    CheckViolation,
    Other,
}

impl From<sqlx::error::ErrorKind> for ErrorConstraintKind {
    fn from(kind: sqlx::error::ErrorKind) -> Self {
        match kind {
            sqlx::error::ErrorKind::UniqueViolation => Self::UniqueViolation,
            sqlx::error::ErrorKind::ForeignKeyViolation => Self::ForeignKeyViolation,
            sqlx::error::ErrorKind::NotNullViolation => Self::NotNullViolation,
            sqlx::error::ErrorKind::CheckViolation => Self::CheckViolation,
            _ => Self::Other,
        }
    }
}

/// Generic error type for mutation queries.
#[derive(Debug)]
pub enum Error<C: TryFrom<ErrorConstraintInfo>> {
    /// Catches the cases when a mutation query violates a constraint.
    /// Type `C` is an enum specific to each query or aggregate, enumerating
    /// variants in PascalCase for each constraint that can be violated.
    /// The list of constraints is inferred automatically by AutoModel from the
    /// table schema involved in the query.
    /// The `Option<C>` is `None` when the constraint name is not recognized.
    ConstraintViolation(Option<C>, ErrorConstraintInfo),

    /// Row not found error
    RowNotFound,
    /// System under stress, timeout
    PoolTimeout,

    InternalError(String, sqlx::Error),
}

impl<C: TryFrom<ErrorConstraintInfo>> Error<C> {
    pub fn is_row_not_found(&self) -> bool {
        matches!(self, Self::RowNotFound)
    }

    pub fn is_pool_timeout(&self) -> bool {
        matches!(self, Self::PoolTimeout)
    }

    /// Typed constraint variant when the violation was recognized for this query.
    pub fn constraint(&self) -> Option<&C> {
        match self {
            Self::ConstraintViolation(Some(c), _) => Some(c),
            _ => None,
        }
    }

    pub fn constraint_info(&self) -> Option<&ErrorConstraintInfo> {
        match self {
            Self::ConstraintViolation(_, info) => Some(info),
            _ => None,
        }
    }
}

impl<C: TryFrom<ErrorConstraintInfo>> From<sqlx::Error> for Error<C> {
    fn from(error: sqlx::Error) -> Self {
        match &error {
            sqlx::Error::RowNotFound => Self::RowNotFound,
            sqlx::Error::ColumnNotFound(col) => {
                Self::InternalError(format!("Column not found: {col}"), error)
            }
            sqlx::Error::Database(db_err) => {
                let kind = db_err.kind();
                match kind {
                    sqlx::error::ErrorKind::UniqueViolation
                    | sqlx::error::ErrorKind::ForeignKeyViolation
                    | sqlx::error::ErrorKind::NotNullViolation
                    | sqlx::error::ErrorKind::CheckViolation => {
                        let violation = ErrorConstraintInfo {
                            constraint_name: db_err.constraint().unwrap_or("").to_string(),
                            table_name: db_err.table().unwrap_or("").to_string(),
                            kind: kind.into(),
                        };
                        Self::ConstraintViolation(violation.clone().try_into().ok(), violation)
                    }
                    _ => Self::InternalError(
                        format!("Database error: {}", db_err.message()),
                        error,
                    ),
                }
            }
            sqlx::Error::Configuration(_) => {
                Self::InternalError("Configuration error".to_string(), error)
            }
            sqlx::Error::InvalidArgument(_) => {
                Self::InternalError("Invalid argument".to_string(), error)
            }
            sqlx::Error::Io(_) => Self::InternalError("IO error".to_string(), error),
            sqlx::Error::Tls(_) => Self::InternalError("TLS error".to_string(), error),
            sqlx::Error::Protocol(_) => Self::InternalError("Protocol error".to_string(), error),
            sqlx::Error::TypeNotFound { type_name } => {
                Self::InternalError(format!("Type not found: {type_name}"), error)
            }
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => Self::InternalError(
                format!("Column index out of bounds: index {index}, len {len}"),
                error,
            ),
            sqlx::Error::ColumnDecode { index, source } => Self::InternalError(
                format!("Column decode error at index {index}: {source}"),
                error,
            ),
            sqlx::Error::Encode(_) => Self::InternalError("Encode error".to_string(), error),
            sqlx::Error::Decode(_) => Self::InternalError("Decode error".to_string(), error),
            sqlx::Error::AnyDriverError(_) => {
                Self::InternalError("Driver error".to_string(), error)
            }
            sqlx::Error::PoolTimedOut => Self::PoolTimeout,
            sqlx::Error::PoolClosed => Self::InternalError("Pool closed".to_string(), error),
            sqlx::Error::WorkerCrashed => Self::InternalError("Worker crashed".to_string(), error),
            sqlx::Error::Migrate(_) => Self::InternalError("Migration error".to_string(), error),
            sqlx::Error::InvalidSavePointStatement => {
                Self::InternalError("Invalid save point statement".to_string(), error)
            }
            sqlx::Error::BeginFailed => Self::InternalError("Begin failed".to_string(), error),
            _ => Self::InternalError("Unknown sqlx error".to_string(), error),
        }
    }
}

impl<C> std::fmt::Display for Error<C>
where
    C: std::fmt::Debug + TryFrom<ErrorConstraintInfo>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConstraintViolation(constraint, info) => {
                if let Some(c) = constraint {
                    // Prefer Display when generated constraint enums implement it;
                    // Debug still works for older generated code.
                    write!(f, "Constraint violation: {c:?} ({info})")
                } else {
                    write!(f, "Unknown constraint violation: {info}")
                }
            }
            Error::RowNotFound => write!(f, "Row not found"),
            Error::PoolTimeout => write!(f, "Pool timeout"),
            Error::InternalError(msg, err) => {
                write!(f, "Internal error: {msg}, caused by: {err}")
            }
        }
    }
}

impl<C> std::error::Error for Error<C>
where
    C: std::fmt::Debug + TryFrom<ErrorConstraintInfo>,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InternalError(_, err) => Some(err),
            _ => None,
        }
    }
}

/// Generic error type for read-only queries.
#[derive(Debug)]
pub enum ErrorReadOnly {
    /// Row not found error
    RowNotFound,
    /// System under stress, timeout
    PoolTimeout,
    /// A constraint violation was reported in a read-only query context.
    /// This should not occur for pure SELECT queries; retained for conversion safety.
    UnexpectedConstraintViolation(ErrorConstraintInfo),

    InternalError(String, sqlx::Error),
}

impl ErrorReadOnly {
    pub fn is_row_not_found(&self) -> bool {
        matches!(self, Self::RowNotFound)
    }

    pub fn is_pool_timeout(&self) -> bool {
        matches!(self, Self::PoolTimeout)
    }
}

impl From<sqlx::Error> for ErrorReadOnly {
    fn from(error: sqlx::Error) -> Self {
        Error::<ErrorConstraintInfo>::from(error).into()
    }
}

impl From<ErrorReadOnly> for Error<ErrorConstraintInfo> {
    fn from(value: ErrorReadOnly) -> Self {
        match value {
            ErrorReadOnly::RowNotFound => Self::RowNotFound,
            ErrorReadOnly::PoolTimeout => Self::PoolTimeout,
            ErrorReadOnly::UnexpectedConstraintViolation(info) => {
                Self::ConstraintViolation(info.clone().try_into().ok(), info)
            }
            ErrorReadOnly::InternalError(msg, err) => Self::InternalError(msg, err),
        }
    }
}

impl From<Error<ErrorConstraintInfo>> for ErrorReadOnly {
    fn from(error: Error<ErrorConstraintInfo>) -> Self {
        match error {
            Error::RowNotFound => Self::RowNotFound,
            Error::PoolTimeout => Self::PoolTimeout,
            Error::InternalError(msg, err) => Self::InternalError(msg, err),
            Error::ConstraintViolation(_, info) => Self::UnexpectedConstraintViolation(info),
        }
    }
}

impl std::fmt::Display for ErrorReadOnly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorReadOnly::RowNotFound => write!(f, "Row not found"),
            ErrorReadOnly::PoolTimeout => write!(f, "Pool timeout"),
            ErrorReadOnly::UnexpectedConstraintViolation(info) => {
                write!(f, "Unexpected constraint violation in read-only query: {info}")
            }
            ErrorReadOnly::InternalError(msg, err) => {
                write!(f, "Internal error: {msg}, caused by: {err}")
            }
        }
    }
}

impl std::error::Error for ErrorReadOnly {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InternalError(_, err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for ErrorConstraintKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UniqueViolation => write!(f, "unique violation"),
            Self::ForeignKeyViolation => write!(f, "foreign key violation"),
            Self::NotNullViolation => write!(f, "not null violation"),
            Self::CheckViolation => write!(f, "check violation"),
            Self::Other => write!(f, "other constraint violation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_info_display_handles_missing_names() {
        let info = ErrorConstraintInfo {
            constraint_name: String::new(),
            table_name: String::new(),
            kind: ErrorConstraintKind::UniqueViolation,
        };
        assert_eq!(
            info.to_string(),
            "<unknown> on table <unknown> (unique violation)"
        );
    }

    #[test]
    fn error_helpers_and_display() {
        let info = ErrorConstraintInfo {
            constraint_name: "users_email_key".into(),
            table_name: "users".into(),
            kind: ErrorConstraintKind::UniqueViolation,
        };
        let err = Error::<ErrorConstraintInfo>::ConstraintViolation(Some(info.clone()), info);
        assert!(!err.is_pool_timeout());
        assert!(!err.is_row_not_found());
        assert!(err.constraint().is_some());
        assert!(err.to_string().contains("users_email_key"));
        assert!(err.to_string().contains("unique violation"));
    }

    #[test]
    fn read_only_preserves_unexpected_constraint() {
        let info = ErrorConstraintInfo {
            constraint_name: "users_pkey".into(),
            table_name: "users".into(),
            kind: ErrorConstraintKind::UniqueViolation,
        };
        let err: ErrorReadOnly =
            Error::<ErrorConstraintInfo>::ConstraintViolation(None, info.clone()).into();
        match err {
            ErrorReadOnly::UnexpectedConstraintViolation(got) => {
                assert_eq!(got, info);
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }
}
