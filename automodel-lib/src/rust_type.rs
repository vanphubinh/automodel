use std::path::Path;
use tokio::fs;
use tokio_postgres::types::Field as PgField;
use tokio_postgres::types::Type as PgType;

use crate::datetime_crate::DateTimeCrate;
use crate::utils::{schema_to_module_name, to_pascal_case, to_snake_case};

pub struct UnsupportedTypeError {
    pub schema: String,
    pub name: String,
}

impl std::fmt::Display for UnsupportedTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unsupported PostgreSQL type: {}.{}",
            self.schema, self.name
        )
    }
}
pub trait RustName {
    fn rust_name(&self, datetime_crate: DateTimeCrate) -> Result<String, UnsupportedTypeError>;
}

impl RustName for PgType {
    fn rust_name(&self, datetime_crate: DateTimeCrate) -> Result<String, UnsupportedTypeError> {
        match self.kind() {
            tokio_postgres::types::Kind::Simple => {
                if let Some(datetime_type) = datetime_crate.simple_pg_type_rust_name(self)? {
                    return Ok(datetime_type);
                }

                let maybe_rust_type = match self {
                    // Boolean & Numeric Types
                    &PgType::BOOL => "bool",
                    &PgType::CHAR => "i8",
                    &PgType::INT2 => "i16",
                    &PgType::INT4 => "i32",
                    &PgType::INT8 => "i64",
                    &PgType::FLOAT4 => "f32",
                    &PgType::FLOAT8 => "f64",
                    &PgType::NUMERIC => "rust_decimal::Decimal",

                    // UUID Type
                    &PgType::UUID => "uuid::Uuid",

                    // special identifiers
                    &PgType::REGPROC => "u32",
                    &PgType::OID => "u32",
                    &PgType::TID => "(u32, u32)",
                    &PgType::XID => "u32",
                    &PgType::CID => "u32",
                    &PgType::XID8 => "u64",

                    // String & Text Types
                    &PgType::NAME => "String",
                    &PgType::TEXT => "String",
                    &PgType::BPCHAR => "String",
                    &PgType::VARCHAR => "String",
                    &PgType::XML => "String",

                    // Json Types
                    &PgType::JSON => "serde_json::Value",
                    &PgType::JSONB => "serde_json::Value",
                    &PgType::JSONPATH => "String",

                    // Binary & Bit Types
                    &PgType::BYTEA => "Vec<u8>",
                    &PgType::BIT => "bit_vec::BitVec",
                    &PgType::VARBIT => "bit_vec::BitVec",

                    // Date & Time Types (DATE/TIME/TIMESTAMP/TIMESTAMPTZ handled above)
                    &PgType::INTERVAL => "sqlx::postgres::types::PgInterval",
                    &PgType::TIMETZ => "sqlx::postgres::types::PgTimeTz",

                    // Network & Address Types
                    &PgType::CIDR => "std::net::IpAddr",
                    &PgType::INET => "std::net::IpAddr",
                    &PgType::MACADDR => "mac_address::MacAddress",

                    // Geometric Types
                    &PgType::POINT => "sqlx::postgres::types::PgPoint",
                    &PgType::LSEG => "sqlx::postgres::types::PgLseg",
                    &PgType::PATH => "sqlx::postgres::types::PgPath",
                    &PgType::BOX => "sqlx::postgres::types::PgBox",
                    &PgType::POLYGON => "sqlx::postgres::types::PgPolygon",
                    &PgType::CIRCLE => "sqlx::postgres::types::PgCircle",
                    &PgType::LINE => "sqlx::postgres::types::PgLine",

                    // Special & System Types
                    &PgType::ACLITEM => "String",
                    &PgType::TSQUERY => "String",
                    &PgType::REGCONFIG => "u32",
                    &PgType::REGDICTIONARY => "u32",
                    &PgType::REGNAMESPACE => "u32",
                    &PgType::REGROLE => "u32",
                    &PgType::REGCOLLATION => "u32",
                    &PgType::PG_NDISTINCT => "String",
                    &PgType::PG_DEPENDENCIES => "String",
                    &PgType::PG_BRIN_BLOOM_SUMMARY => "String",
                    &PgType::PG_BRIN_MINMAX_MULTI_SUMMARY => "String",
                    &PgType::PG_MCV_LIST => "String",
                    &PgType::PG_SNAPSHOT => "String",
                    &PgType::PG_LSN => "u64",
                    &PgType::TXID_SNAPSHOT => "String",

                    // Pseudo-types, handlers, and unknowns: map to serde_json::Value
                    _ => "",
                };
                if maybe_rust_type.is_empty() {
                    Err(UnsupportedTypeError {
                        schema: self.schema().to_string(),
                        name: self.name().to_string(),
                    })
                } else {
                    Ok(maybe_rust_type.to_string())
                }
            }
            tokio_postgres::types::Kind::Enum(_) => Ok(format!(
                "super::{}::{}",
                schema_to_module_name(self.schema()),
                to_pascal_case(self.name())
            )),
            tokio_postgres::types::Kind::Array(elem_type) => {
                Ok(format!(
                    "Vec<{}>",
                    elem_type.rust_name(datetime_crate)?
                ))
            }
            tokio_postgres::types::Kind::Range(elem_type) => {
                let elem_wire = elem_type.rust_name(datetime_crate)?;
                let elem_range = datetime_crate.pg_range_element_type(&elem_wire);
                Ok(format!("sqlx::postgres::types::PgRange<{}>", elem_range))
            }
            tokio_postgres::types::Kind::Domain(_) => Ok(format!(
                "super::{}::{}",
                schema_to_module_name(self.schema()),
                to_pascal_case(self.name())
            )),
            tokio_postgres::types::Kind::Composite(_) => Ok(format!(
                "super::{}::{}",
                schema_to_module_name(self.schema()),
                to_pascal_case(self.name())
            )),
            tokio_postgres::types::Kind::Pseudo => Err(UnsupportedTypeError {
                schema: self.schema().to_string(),
                name: self.name().to_string(),
            }),
            tokio_postgres::types::Kind::Multirange(_) => {
                // this is unsupported by sqlx, automodel would fine to have it supported
                Err(UnsupportedTypeError {
                    schema: self.schema().to_string(),
                    name: self.name().to_string(),
                })
            }
            _ => Err(UnsupportedTypeError {
                schema: self.schema().to_string(),
                name: self.name().to_string(),
            }),
        }
    }
}

pub struct TypeSystem {
    types: indexmap::IndexMap<TypeRef, TypeInfo>,
    datetime_crate: DateTimeCrate,
}

impl TypeSystem {
    pub fn new(datetime_crate: DateTimeCrate) -> Self {
        Self {
            types: indexmap::IndexMap::new(),
            datetime_crate,
        }
    }

    pub fn insert(&mut self, pg_type: &PgType) -> Result<(), UnsupportedTypeError> {
        let type_info = TypeInfo::try_from(pg_type, self.datetime_crate)?;
        if self.types.contains_key(&type_info.rust_name) {
            return Ok(());
        }
        self.types.insert(type_info.rust_name.clone(), type_info);

        match pg_type.kind() {
            tokio_postgres::types::Kind::Composite(fields) => {
                for field in fields {
                    self.insert(field.type_())?;
                }
            }
            tokio_postgres::types::Kind::Array(elem_type) => {
                self.insert(elem_type)?;
            }
            tokio_postgres::types::Kind::Range(elem_type) => {
                self.insert(elem_type)?;
            }
            tokio_postgres::types::Kind::Domain(base_type) => {
                self.insert(base_type)?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Query pg_attribute for all composite/table types and set field nullability.
    /// Call once after the type system is fully built.
    pub async fn resolve_nullability(
        &mut self,
        client: &tokio_postgres::Client,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Collect OIDs for all struct types
        let struct_oids: Vec<u32> = self
            .types
            .values()
            .filter(|ti| matches!(ti.kind, TypeKind::Struct(_)))
            .map(|ti| ti.id)
            .collect();

        if struct_oids.is_empty() {
            return Ok(());
        }

        // Batch query all fields for all composite/table types at once
        let rows = client
            .query(
                "SELECT t.oid, a.attname, a.attnotnull \
                 FROM pg_attribute a \
                 JOIN pg_type t ON t.typrelid = a.attrelid \
                 WHERE t.oid = ANY($1) AND a.attnum > 0 AND NOT a.attisdropped \
                 ORDER BY t.oid, a.attnum",
                &[&struct_oids],
            )
            .await?;

        // Build a lookup: type OID -> Vec<(field_name, is_nullable)>
        let mut nullability: std::collections::HashMap<u32, Vec<(String, bool)>> =
            std::collections::HashMap::new();
        for row in &rows {
            let oid: u32 = row.get(0);
            let name: String = row.get(1);
            let not_null: bool = row.get(2);
            nullability.entry(oid).or_default().push((name, !not_null));
        }

        // Apply nullability to struct types
        for type_info in self.types.values_mut() {
            if let TypeKind::Struct(ref mut fields) = type_info.kind {
                if let Some(field_nulls) = nullability.get(&type_info.id) {
                    for (name, is_nullable) in field_nulls {
                        if let Some(field) = fields.iter_mut().find(|f| f.pg_name == *name) {
                            field.is_nullable = *is_nullable;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Convert domain types with `CHECK (VALUE IN (...))` constraints into Rust enums.
    pub fn apply_domain_enums(
        &mut self,
        domain_enums: &std::collections::HashMap<String, crate::domain_enum::DomainEnumConstraint>,
    ) {
        for (qualified_name, constraint) in domain_enums {
            let Some(type_info) = self
                .types
                .values_mut()
                .find(|ti| format!("{}.{}", ti.pg_schema, ti.pg_name) == *qualified_name)
            else {
                continue;
            };

            if !matches!(type_info.kind, TypeKind::Alias(_)) {
                continue;
            }

            let enum_variants = constraint
                .variants
                .iter()
                .map(|variant| EnumVariant {
                    pg_name: variant.clone(),
                    rust_name: to_pascal_case(variant),
                })
                .collect();

            type_info.kind = TypeKind::Enum(EnumInfo {
                variants: enum_variants,
                // Domains are transmitted as their base type on the PostgreSQL wire protocol.
                sqlx_type_name: Some(constraint.base_type.clone()),
            });
        }
    }

    /// Apply a custom type alias override to a domain type.
    ///
    /// Changes the generated `pub type Alias = BaseType;` to use `mapped_type` instead.
    pub fn apply_alias_mapping(&mut self, schema: &str, type_name: &str, mapped_type: &str) {
        let type_info = self
            .types
            .values_mut()
            .find(|ti| ti.pg_schema == schema && ti.pg_name == type_name);

        if let Some(type_info) = type_info {
            if let TypeKind::Alias(ref mut alias) = type_info.kind {
                alias.mapped_type_ref = Some(mapped_type.to_string());
            }
        }
    }

    /// Apply a single custom type mapping to a composite type field.
    ///
    /// Finds the composite type by `schema.type_name` and sets `mapped_type_ref`
    /// and `needs_json_wrapper` on the matching field.
    pub fn apply_field_mapping(
        &mut self,
        schema: &str,
        type_name: &str,
        field_name: &str,
        mapped_type: &str,
        needs_json_wrapper: bool,
    ) {
        let type_info = self
            .types
            .values_mut()
            .find(|ti| ti.pg_schema == schema && ti.pg_name == type_name);

        if let Some(type_info) = type_info {
            if let TypeKind::Struct(ref mut fields) = type_info.kind {
                if let Some(field) = fields.iter_mut().find(|f| f.pg_name == *field_name) {
                    field.mapped_type_ref = Some(mapped_type.to_string());
                    field.needs_json_wrapper = needs_json_wrapper;
                }
            }
        }
    }

    /// Generate type definition files grouped by rust_module, plus a mod.rs.
    /// Only Enum and Struct kinds produce output; Simple/Array/Range/Alias are skipped.
    pub async fn codegen(&self, output_dir: &Path) -> std::io::Result<()> {
        let datetime_crate = self.datetime_crate;
        // Group types by module
        let mut modules: std::collections::BTreeMap<String, Vec<&TypeInfo>> =
            std::collections::BTreeMap::new();
        for type_info in self.types.values() {
            if type_info.codegen(&[], datetime_crate).is_some() {
                modules
                    .entry(type_info.rust_module.clone())
                    .or_default()
                    .push(type_info);
            }
        }

        fs::create_dir_all(output_dir).await?;

        // Generate each module file
        for (module_name, types) in &modules {
            let mut code = String::new();
            code.push_str(
                "// This file was automatically generated by AutoModel. Do not edit manually.\n",
            );
            code.push_str("// @generated\n\n");
            if datetime_crate.needs_to_sqlx_import() {
                code.push_str("use jiff_sqlx::ToSqlx;\n\n");
            }

            for type_info in types {
                if let Some(type_code) = type_info.codegen(&[], datetime_crate) {
                    code.push_str(&type_code);
                }
            }

            let file_path = output_dir.join(format!("{}.rs", module_name));
            fs::write(&file_path, &code).await?;
        }

        // Generate mod.rs
        let mut mod_content = String::new();
        mod_content.push_str(
            "// This file was automatically generated by AutoModel. Do not edit manually.\n",
        );
        mod_content.push_str("// @generated\n\n");
        for module_name in modules.keys() {
            mod_content.push_str(&format!("pub mod {};\n", module_name));
        }

        let mod_file = output_dir.join("mod.rs");
        fs::write(&mod_file, &mod_content).await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub id: u32,
    pub pg_name: String,
    pub pg_schema: String,
    pub rust_module: String,
    pub rust_name: String,
    pub kind: TypeKind,
}

impl TypeInfo {
    pub fn try_from(pg_type: &PgType, datetime_crate: DateTimeCrate) -> Result<Self, UnsupportedTypeError> {
        Ok(Self {
            id: pg_type.oid(),
            pg_name: pg_type.name().to_string(),
            pg_schema: pg_type.schema().to_string(),
            rust_module: schema_to_module_name(pg_type.schema()),
            rust_name: pg_type.rust_name(datetime_crate)?,
            kind: match pg_type.kind() {
                tokio_postgres::types::Kind::Simple => TypeKind::Simple,
                tokio_postgres::types::Kind::Enum(variants) => {
                    let mut enum_variants = Vec::with_capacity(variants.len());
                    for variant in variants {
                        enum_variants.push(EnumVariant::try_from(variant)?);
                    }
                    TypeKind::Enum(EnumInfo {
                        variants: enum_variants,
                        sqlx_type_name: None,
                    })
                }
                tokio_postgres::types::Kind::Array(elem_type) => {
                    TypeKind::Array(elem_type.rust_name(datetime_crate)?)
                }
                tokio_postgres::types::Kind::Range(elem_type) => {
                    TypeKind::Range(elem_type.rust_name(datetime_crate)?)
                }
                tokio_postgres::types::Kind::Domain(base_type) => TypeKind::Alias(AliasInfo {
                    type_ref: base_type.rust_name(datetime_crate)?,
                    mapped_type_ref: None,
                }),
                tokio_postgres::types::Kind::Composite(fields) => {
                    let mut struct_fields = Vec::with_capacity(fields.len());
                    for field in fields {
                        struct_fields.push(StructField::try_from(field, datetime_crate)?);
                    }
                    TypeKind::Struct(struct_fields)
                }
                _ => {
                    return Err(UnsupportedTypeError {
                        schema: pg_type.schema().to_string(),
                        name: pg_type.name().to_string(),
                    })
                }
            },
        })
    }
}

type TypeRef = String;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TypeKind {
    Simple,
    Array(TypeRef),
    Range(TypeRef),
    Alias(AliasInfo),
    Enum(EnumInfo),
    Struct(Vec<StructField>),
}

#[derive(Debug, Clone)]
pub struct EnumInfo {
    pub variants: Vec<EnumVariant>,
    /// SQLx wire type name. `None` uses the PostgreSQL type name (native enums).
    pub sqlx_type_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AliasInfo {
    /// The Rust type of the domain's base type (e.g. "i32" for a domain over integer)
    pub type_ref: TypeRef,
    /// Custom type override (e.g. "std::num::NonZeroI32") set via `types:` config
    pub mapped_type_ref: Option<TypeRef>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub pg_name: String,
    pub rust_name: String,
}

impl TryFrom<&String> for EnumVariant {
    type Error = UnsupportedTypeError;

    fn try_from(pg_name: &String) -> Result<Self, Self::Error> {
        Ok(Self {
            pg_name: pg_name.clone(),
            rust_name: to_pascal_case(pg_name),
        })
    }
}

/// Core field info shared between composite type fields, input params, and output columns
#[derive(Debug, Clone)]
pub struct StructField {
    pub pg_name: String,
    pub rust_name: String,
    /// The Rust type derived from the PostgreSQL type (e.g. "serde_json::Value" for jsonb)
    pub type_ref: TypeRef,
    /// Custom type mapping override (e.g. "Vec<crate::models::UserSocialLink>" for jsonb)
    pub mapped_type_ref: Option<String>,
    /// Whether this field is nullable (wraps in Option<T>)
    pub is_nullable: bool,
    /// Whether this field needs JSON serialization wrapper (for custom type mappings)
    pub needs_json_wrapper: bool,
}

impl StructField {
    pub fn try_from(pg_field: &PgField, datetime_crate: DateTimeCrate) -> Result<Self, UnsupportedTypeError> {
        Ok(Self {
            pg_name: pg_field.name().to_string(),
            rust_name: to_snake_case(pg_field.name()),
            type_ref: pg_field.type_().rust_name(datetime_crate)?,
            mapped_type_ref: None,
            is_nullable: false,
            needs_json_wrapper: false,
        })
    }
}

impl StructField {
    /// Whether the underlying PostgreSQL type is an array
    pub fn is_pg_array(&self) -> bool {
        self.type_ref.starts_with("Vec<")
    }

    /// The effective Rust type for codegen (mapped type if set, otherwise type_ref)
    pub fn rust_type(&self) -> &str {
        self.mapped_type_ref.as_deref().unwrap_or(&self.type_ref)
    }

    /// Generates a struct field declaration line: `pub field_name: Type,`
    /// Always produces clean types without `sqlx::types::Json` wrappers.
    pub fn codegen(&self) -> String {
        let rust_type = self.rust_type();
        let type_str = if self.is_nullable {
            format!("Option<{}>", rust_type)
        } else {
            rust_type.to_string()
        };
        format!("    pub {}: {},\n", self.rust_name, type_str)
    }

    /// Like `codegen`, but uses serde-compatible types for datetime fields in composite structs.
    pub fn codegen_for_serde(&self, datetime_crate: crate::datetime_crate::DateTimeCrate) -> String {
        let rust_type = self.rust_type();
        let type_str = if let Some(serde_type) = datetime_crate.serde_type_for_wire_type(rust_type) {
            if self.is_nullable {
                format!("Option<{}>", serde_type)
            } else {
                serde_type.to_string()
            }
        } else if self.is_nullable {
            format!("Option<{}>", rust_type)
        } else {
            rust_type.to_string()
        };
        format!("    pub {}: {},\n", self.rust_name, type_str)
    }

    /// Generates the encode expression for this field inside a `PgRecordEncoder`.
    /// For JSON-wrapped fields, wraps the value in `sqlx::types::Json` internally.
    fn codegen_encode_expr(
        &self,
        datetime_crate: crate::datetime_crate::DateTimeCrate,
    ) -> String {
        if !self.needs_json_wrapper {
            if datetime_crate.is_datetime_wire_type(self.rust_type()) {
                return match datetime_crate {
                    crate::datetime_crate::DateTimeCrate::Jiff => {
                        if self.is_nullable {
                            format!(
                                "        encoder.encode(&self.{f}.as_ref().map(|v| v.to_sqlx()))?;\n",
                                f = self.rust_name
                            )
                        } else {
                            format!(
                                "        encoder.encode(&self.{f}.to_sqlx())?;\n",
                                f = self.rust_name
                            )
                        }
                    }
                    crate::datetime_crate::DateTimeCrate::Time => {
                        format!("        encoder.encode(&self.{})?;\n", self.rust_name)
                    }
                };
            }
            return format!("        encoder.encode(&self.{})?;\n", self.rust_name);
        }

        if self.is_pg_array() {
            let rust_type = self.rust_type();
            let inner = &rust_type[4..rust_type.len() - 1]; // strip "Vec<" and ">"
            let has_inner_option = inner.starts_with("Option<");

            if self.is_nullable && has_inner_option {
                format!(
                    "        encoder.encode(&self.{f}.as_ref().map(|v| v.iter().map(|e| e.as_ref().map(sqlx::types::Json)).collect::<Vec<_>>()))?;\n",
                    f = self.rust_name
                )
            } else if self.is_nullable {
                format!(
                    "        encoder.encode(&self.{f}.as_ref().map(|v| v.iter().map(|e| sqlx::types::Json(e)).collect::<Vec<_>>()))?;\n",
                    f = self.rust_name
                )
            } else if has_inner_option {
                format!(
                    "        encoder.encode(&self.{f}.iter().map(|e| e.as_ref().map(sqlx::types::Json)).collect::<Vec<_>>())?;\n",
                    f = self.rust_name
                )
            } else {
                format!(
                    "        encoder.encode(&self.{f}.iter().map(|e| sqlx::types::Json(e)).collect::<Vec<_>>())?;\n",
                    f = self.rust_name
                )
            }
        } else if self.is_nullable {
            format!(
                "        encoder.encode(&self.{f}.as_ref().map(sqlx::types::Json))?;\n",
                f = self.rust_name
            )
        } else {
            format!(
                "        encoder.encode(&sqlx::types::Json(&self.{f}))?;\n",
                f = self.rust_name
            )
        }
    }

    /// Generates an inline decode expression for use in struct field initialization.
    /// For JSON-wrapped fields, decodes as `Json<T>` and unwraps to the clean type.
    fn codegen_decode_expr(
        &self,
        datetime_crate: crate::datetime_crate::DateTimeCrate,
    ) -> String {
        if !self.needs_json_wrapper {
            if datetime_crate.is_datetime_wire_type(self.rust_type()) {
                let wire = self.rust_type();
                return match datetime_crate {
                    crate::datetime_crate::DateTimeCrate::Jiff => {
                        if self.is_nullable {
                            format!(
                                "decoder.try_decode::<Option<{wire}>>()?.map(|v| v.to_jiff())"
                            )
                        } else {
                            format!("decoder.try_decode::<{wire}>()?.to_jiff()")
                        }
                    }
                    crate::datetime_crate::DateTimeCrate::Time => {
                        "decoder.try_decode()?".to_string()
                    }
                };
            }
            return "decoder.try_decode()?".to_string();
        }

        let rust_type = self.rust_type();

        if self.is_pg_array() {
            let inner = &rust_type[4..rust_type.len() - 1]; // strip "Vec<" and ">"
            let has_inner_option = inner.starts_with("Option<");
            let elem = if has_inner_option {
                &inner[7..inner.len() - 1] // strip "Option<" and ">"
            } else {
                inner
            };

            if self.is_nullable && has_inner_option {
                format!(
                    "decoder.try_decode::<Option<Vec<Option<sqlx::types::Json<{e}>>>>>()?.map(|v| v.into_iter().map(|e| e.map(|j| j.0)).collect())",
                    e = elem
                )
            } else if self.is_nullable {
                format!(
                    "decoder.try_decode::<Option<Vec<sqlx::types::Json<{e}>>>>()?.map(|v| v.into_iter().map(|j| j.0).collect())",
                    e = elem
                )
            } else if has_inner_option {
                format!(
                    "decoder.try_decode::<Vec<Option<sqlx::types::Json<{e}>>>>()?.into_iter().map(|e| e.map(|j| j.0)).collect::<Vec<_>>()",
                    e = elem
                )
            } else {
                format!(
                    "decoder.try_decode::<Vec<sqlx::types::Json<{e}>>>()?.into_iter().map(|j| j.0).collect::<Vec<_>>()",
                    e = elem
                )
            }
        } else if self.is_nullable {
            format!(
                "decoder.try_decode::<Option<sqlx::types::Json<{t}>>>()?.map(|v| v.0)",
                t = rust_type
            )
        } else {
            format!(
                "decoder.try_decode::<sqlx::types::Json<{t}>>()?.0",
                t = rust_type
            )
        }
    }
}

/// An input parameter to a SQL query
#[derive(Debug, Clone)]
pub struct InputParam {
    pub field: StructField,
    /// Whether this is an optional (conditional) parameter (`?` suffix)
    pub is_optional: bool,
    /// Whether array elements are nullable (`[?]` suffix): Vec<T> → Vec<Option<T>>
    /// The actual type wrapping happens during extraction; this field records the annotation.
    #[allow(dead_code)]
    pub is_nullable_element: bool,
}

impl std::ops::Deref for InputParam {
    type Target = StructField;
    fn deref(&self) -> &StructField {
        &self.field
    }
}

/// An output column from a SQL query
#[derive(Debug, Clone)]
pub struct OutputColumn {
    pub field: StructField,
}

impl std::ops::Deref for OutputColumn {
    type Target = StructField;
    fn deref(&self) -> &StructField {
        &self.field
    }
}

impl TypeInfo {
    /// Generate the full type definition (enum, struct, or type alias).
    /// Returns `None` for Simple, Array, and Range kinds.
    pub fn codegen(
        &self,
        custom_derives: &[String],
        datetime_crate: crate::datetime_crate::DateTimeCrate,
    ) -> Option<String> {
        match &self.kind {
            TypeKind::Enum(enum_info) => Some(self.codegen_enum(enum_info, custom_derives)),
            TypeKind::Struct(fields) => Some(self.codegen_struct(fields, custom_derives, datetime_crate)),
            TypeKind::Alias(alias) => Some(self.codegen_alias(alias)),
            _ => None,
        }
    }

    fn codegen_enum(&self, enum_info: &EnumInfo, custom_derives: &[String]) -> String {
        let variants = &enum_info.variants;
        let name = self
            .rust_name
            .strip_prefix(&format!("super::{}::", self.rust_module))
            .unwrap_or(&self.rust_name);
        let derive_attr = Self::build_derive_attribute(
            &[
                "Debug",
                "Clone",
                "PartialEq",
                "Eq",
                "serde::Serialize",
                "serde::Deserialize",
                "sqlx::Type",
            ],
            custom_derives,
        );

        let sqlx_type_name = enum_info
            .sqlx_type_name
            .as_deref()
            .unwrap_or(&self.pg_name);

        let mut code = format!(
            "{}\n#[sqlx(type_name = \"{}\")]\npub enum {} {{\n",
            derive_attr, sqlx_type_name, name
        );
        for v in variants {
            code.push_str(&format!(
                "    #[sqlx(rename = \"{}\")]\n    {},\n",
                v.pg_name, v.rust_name
            ));
        }
        code.push_str("}\n\n");

        // FromStr
        code.push_str(&format!(
            r#"impl std::str::FromStr for {name} {{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
"#
        ));
        for v in variants {
            code.push_str(&format!(
                "            \"{}\" => Ok({}::{}),\n",
                v.pg_name, name, v.rust_name
            ));
        }
        code.push_str(&format!(
            "            _ => Err(format!(\"Invalid {name} variant: {{}}\", s)),\n        }}\n    }}\n}}\n\n"
        ));

        // Display
        code.push_str(&format!(
            "impl std::fmt::Display for {name} {{\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n        let s = match self {{\n"
        ));
        for v in variants {
            code.push_str(&format!(
                "            {}::{} => \"{}\",\n",
                name, v.rust_name, v.pg_name
            ));
        }
        code.push_str("        };\n        write!(f, \"{}\", s)\n    }\n}\n\n");

        code
    }

    fn codegen_struct(
        &self,
        fields: &[StructField],
        custom_derives: &[String],
        datetime_crate: crate::datetime_crate::DateTimeCrate,
    ) -> String {
        let name = self
            .rust_name
            .strip_prefix(&format!("super::{}::", self.rust_module))
            .unwrap_or(&self.rust_name);

        let has_json_fields = fields.iter().any(|f| f.needs_json_wrapper);
        let has_datetime_fields = fields
            .iter()
            .any(|f| datetime_crate.is_datetime_wire_type(f.rust_type()));

        if has_json_fields || has_datetime_fields {
            // Manual impl path: clean struct + hand-written Type/Encode/Decode
            let derive_attr = Self::build_derive_attribute(
                &["Debug", "Clone", "serde::Serialize", "serde::Deserialize"],
                custom_derives,
            );

            let mut code = format!("{}\npub struct {} {{\n", derive_attr, name);
            for field in fields {
                code.push_str(&field.codegen_for_serde(datetime_crate));
            }
            code.push_str("}\n\n");

            // impl Type
            code.push_str(&format!(
                "impl sqlx::Type<sqlx::Postgres> for {} {{\n    \
                 fn type_info() -> sqlx::postgres::PgTypeInfo {{\n        \
                 sqlx::postgres::PgTypeInfo::with_name(\"{}\")\n    \
                 }}\n}}\n\n",
                name, self.pg_name
            ));

            // impl PgHasArrayType (needed when the type is used as an array parameter)
            code.push_str(&format!(
                "impl sqlx::postgres::PgHasArrayType for {} {{\n    \
                 fn array_type_info() -> sqlx::postgres::PgTypeInfo {{\n        \
                 sqlx::postgres::PgTypeInfo::with_name(\"_{}\")\n    \
                 }}\n}}\n\n",
                name, self.pg_name
            ));

            // impl Encode
            code.push_str(&format!(
                "impl sqlx::Encode<'_, sqlx::Postgres> for {} {{\n    \
                 fn encode_by_ref(\n        \
                 &self,\n        \
                 buf: &mut sqlx::postgres::PgArgumentBuffer,\n    \
                 ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {{\n        \
                 let mut encoder = sqlx::postgres::types::PgRecordEncoder::new(buf);\n",
                name
            ));
            for field in fields {
                code.push_str(&field.codegen_encode_expr(datetime_crate));
            }
            code.push_str(
                "        encoder.finish();\n        \
                 Ok(sqlx::encode::IsNull::No)\n    \
                 }\n}\n\n",
            );

            // impl Decode
            code.push_str(&format!(
                "impl<'r> sqlx::Decode<'r, sqlx::Postgres> for {} {{\n    \
                 fn decode(\n        \
                 value: sqlx::postgres::PgValueRef<'r>,\n    \
                 ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {{\n        \
                 let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;\n",
                name
            ));
            code.push_str("        Ok(Self {\n");
            for field in fields {
                code.push_str(&format!(
                    "            {}: {},\n",
                    field.rust_name,
                    field.codegen_decode_expr(datetime_crate)
                ));
            }
            code.push_str("        })\n    }\n}\n\n");

            code
        } else {
            // Derive path: standard #[derive(sqlx::Type)]
            let derive_attr = Self::build_derive_attribute(
                &[
                    "Debug",
                    "Clone",
                    "serde::Serialize",
                    "serde::Deserialize",
                    "sqlx::Type",
                ],
                custom_derives,
            );

            let mut code = format!(
                "{}\n#[sqlx(type_name = \"{}\")]\npub struct {} {{\n",
                derive_attr, self.pg_name, name
            );
            for field in fields {
                code.push_str(&field.codegen());
            }
            code.push_str("}\n\n");
            code
        }
    }

    fn codegen_alias(&self, alias: &AliasInfo) -> String {
        let name = self
            .rust_name
            .strip_prefix(&format!("super::{}::", self.rust_module))
            .unwrap_or(&self.rust_name);
        let rhs = alias.mapped_type_ref.as_deref().unwrap_or(&alias.type_ref);
        format!("pub type {} = {};\n\n", name, rhs)
    }

    fn build_derive_attribute(default_derives: &[&str], custom_derives: &[String]) -> String {
        let mut all_derives = default_derives
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        all_derives.extend(custom_derives.iter().cloned());
        let mut seen = std::collections::HashSet::new();
        all_derives.retain(|d| seen.insert(d.clone()));
        format!("#[derive({})]", all_derives.join(", "))
    }
}
