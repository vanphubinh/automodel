use std::path::Path;
use tokio::fs;
use tokio_postgres::types::Field as PgField;
use tokio_postgres::types::Type as PgType;

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
    fn rust_name(&self) -> Result<String, UnsupportedTypeError>;
}

impl RustName for PgType {
    fn rust_name(&self) -> Result<String, UnsupportedTypeError> {
        match self.kind() {
            tokio_postgres::types::Kind::Simple => {
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

                    // Date & Time Types
                    &PgType::DATE => "chrono::NaiveDate",
                    &PgType::TIME => "chrono::NaiveTime",
                    &PgType::TIMESTAMP => "chrono::NaiveDateTime",
                    &PgType::TIMESTAMPTZ => "chrono::DateTime<chrono::Utc>",
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
                Ok(format!("std::vec::Vec<{}>", elem_type.rust_name()?))
            }
            tokio_postgres::types::Kind::Range(elem_type) => Ok(format!(
                "sqlx::postgres::types::PgRange<{}>",
                elem_type.rust_name()?
            )),
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
    types: std::collections::HashMap<TypeRef, TypeInfo>,
}

impl TypeSystem {
    pub fn new() -> Self {
        Self {
            types: std::collections::HashMap::new(),
        }
    }

    pub fn get(&self, type_ref: &TypeRef) -> Option<&TypeInfo> {
        self.types.get(type_ref)
    }

    pub fn insert(&mut self, pg_type: &PgType) -> Result<(), UnsupportedTypeError> {
        let type_info = TypeInfo::try_from(pg_type)?;
        self.types.insert(type_info.rust_name.clone(), type_info);

        match pg_type.kind() {
            tokio_postgres::types::Kind::Composite(fields) => {
                for field in fields {
                    self.insert(field.type_())?; // Ensure the field's type is also in the system
                }
            }
            tokio_postgres::types::Kind::Array(elem_type) => {
                self.insert(elem_type)?; // Ensure the element type is also in the system
            }
            tokio_postgres::types::Kind::Range(elem_type) => {
                self.insert(elem_type)?; // Ensure the element type is also in the system
            }
            tokio_postgres::types::Kind::Domain(base_type) => {
                self.insert(base_type)?; // Ensure the base type is also in the system
            }
            _ => {}
        }

        Ok(())
    }

    /// Generate type definition files grouped by rust_module, plus a mod.rs.
    /// Only Enum and Struct kinds produce output; Simple/Array/Range/Alias are skipped.
    pub async fn codegen(&self, output_dir: &Path) -> std::io::Result<()> {
        // Group types by module
        let mut modules: std::collections::BTreeMap<String, Vec<&TypeInfo>> =
            std::collections::BTreeMap::new();
        for type_info in self.types.values() {
            if type_info.codegen(&[]).is_some() {
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

            for type_info in types {
                if let Some(type_code) = type_info.codegen(&[]) {
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

impl TryFrom<&PgType> for TypeInfo {
    type Error = UnsupportedTypeError;

    fn try_from(pg_type: &PgType) -> Result<Self, Self::Error> {
        Ok(Self {
            id: pg_type.oid(),
            pg_name: pg_type.name().to_string(),
            pg_schema: pg_type.schema().to_string(),
            rust_module: schema_to_module_name(pg_type.schema()),
            rust_name: pg_type.rust_name()?,
            kind: match pg_type.kind() {
                tokio_postgres::types::Kind::Simple => TypeKind::Simple,
                tokio_postgres::types::Kind::Enum(variants) => {
                    let mut enum_variants = Vec::with_capacity(variants.len());
                    for variant in variants {
                        enum_variants.push(EnumVariant::try_from(variant)?);
                    }
                    TypeKind::Enum(enum_variants)
                }
                tokio_postgres::types::Kind::Array(elem_type) => {
                    TypeKind::Array(elem_type.rust_name()?)
                }
                tokio_postgres::types::Kind::Range(elem_type) => {
                    TypeKind::Range(elem_type.rust_name()?)
                }
                tokio_postgres::types::Kind::Domain(base_type) => {
                    TypeKind::Alias(base_type.rust_name()?)
                }
                tokio_postgres::types::Kind::Composite(fields) => {
                    let mut struct_fields = Vec::with_capacity(fields.len());
                    for field in fields {
                        struct_fields.push(StructField::try_from(field)?);
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
pub enum TypeKind {
    Simple,
    Array(TypeRef),
    Range(TypeRef),
    Alias(TypeRef),
    Enum(Vec<EnumVariant>),
    Struct(Vec<StructField>),
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

/// Information about a field in a PostgreSQL composite type
#[derive(Debug, Clone)]
pub struct StructField {
    pub pg_name: String,
    pub rust_name: String,
    pub is_nullable: bool,
    pub type_ref: TypeRef,
}

impl TryFrom<&PgField> for StructField {
    type Error = UnsupportedTypeError;

    fn try_from(pg_field: &PgField) -> Result<Self, Self::Error> {
        Ok(Self {
            pg_name: pg_field.name().to_string(),
            rust_name: to_snake_case(pg_field.name()),
            is_nullable: false,
            type_ref: pg_field.type_().rust_name()?,
        })
    }
}

impl StructField {
    pub fn with_nullable(mut self) -> Self {
        self.is_nullable = true;
        self
    }

    pub fn make_nullable(&mut self) {
        self.is_nullable = true;
    }

    /// Generates a struct field declaration line: `pub field_name: Type,`
    pub fn codegen(&self) -> String {
        let rename = if self.rust_name != self.pg_name {
            format!("    #[sqlx(rename = \"{}\")]\n", self.pg_name)
        } else {
            String::new()
        };
        if self.is_nullable {
            format!(
                "{}    pub {}: Option<{}>,\n",
                rename, self.rust_name, self.type_ref
            )
        } else {
            format!("{}    pub {}: {},\n", rename, self.rust_name, self.type_ref)
        }
    }
}

impl TypeInfo {
    /// Generate the full type definition (enum or struct with sqlx derive macros).
    /// Returns `None` for Simple, Array, Range, and Alias kinds.
    pub fn codegen(&self, custom_derives: &[String]) -> Option<String> {
        match &self.kind {
            TypeKind::Enum(variants) => Some(self.codegen_enum(variants, custom_derives)),
            TypeKind::Struct(fields) => Some(self.codegen_struct(fields, custom_derives)),
            _ => None,
        }
    }

    fn codegen_enum(&self, variants: &[EnumVariant], custom_derives: &[String]) -> String {
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

        let mut code = format!(
            "{}\n#[sqlx(type_name = \"{}\")]\npub enum {} {{\n",
            derive_attr, self.pg_name, name
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

    fn codegen_struct(&self, fields: &[StructField], custom_derives: &[String]) -> String {
        let name = self
            .rust_name
            .strip_prefix(&format!("super::{}::", self.rust_module))
            .unwrap_or(&self.rust_name);
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
