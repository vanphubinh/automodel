#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let defaults = automodel::DefaultsConfig {
        telemetry: automodel::DefaultsTelemetryConfig {
            level: automodel::TelemetryLevel::Debug,
            include_sql: true,
        },
        ensure_indexes: true,
        derives: automodel::DefaultsDerivesConfig {
            return_type: vec!["Clone".to_string()],
            parameters_type: vec!["Clone".to_string()],
            conditions_type: vec!["Clone".to_string()],
            error_type: vec!["Clone".to_string()],
        },
    };
    automodel::AutoModel::generate(
        || {
            if std::env::var("CI").is_err() {
                std::env::var("AUTOMODEL_DATABASE_URL").map_err(|_| {
                    "AUTOMODEL_DATABASE_URL environment variable must be set for code generation"
                        .to_string()
                })
            } else {
                Err(
                    "Detecting not up to date AutoModel generated code in CI environment"
                        .to_string(),
                )
            }
        },
        "queries",
        "src/generated",
        defaults,
    )
    .await
}
