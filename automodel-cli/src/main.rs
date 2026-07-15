use anyhow::{Context, Result};
use automodel::*;
use clap::{Arg, ArgMatches, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            generate_command(sub_matches).await?;
        }
        _ => {
            build_cli().print_help()?;
            std::process::exit(1);
        }
    }

    Ok(())
}

fn build_cli() -> Command {
    Command::new("automodel")
        .version("0.1.0")
        .author("AutoModel Team")
        .about("Generate typed Rust functions from SQL query files")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate")
                .about("Generate Rust code from SQL query files")
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("FILE")
                        .help("Path to automodel.yml config file")
                        .default_value("automodel.yml"),
                )
                .arg(
                    Arg::new("database-url")
                        .short('d')
                        .long("database-url")
                        .value_name("URL")
                        .help("PostgreSQL database connection URL (overrides AUTOMODEL_DATABASE_URL env var)"),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .action(clap::ArgAction::SetTrue)
                        .help("Regenerate even when AUTOMODEL_HASH indicates output is up to date"),
                ),
        )
}

async fn generate_command(matches: &ArgMatches) -> Result<()> {
    let config_path = matches.get_one::<String>("config").unwrap();
    let config = AutoModelConfig::from_file(config_path)?;

    let database_url = match matches.get_one::<String>("database-url") {
        Some(url) => url.clone(),
        None => std::env::var("AUTOMODEL_DATABASE_URL").map_err(|_| {
            anyhow::anyhow!(
                "Database URL must be provided via --database-url or AUTOMODEL_DATABASE_URL env var"
            )
        })?,
    };

    println!("Config: {}", config_path);
    println!("Queries: {}", config.queries_dir);
    println!("Output: {}", config.output_dir);

    let force = matches.get_flag("force");

    AutoModel::generate(
        || Ok(database_url.clone()),
        &config.queries_dir,
        &config.output_dir,
        config.defaults(),
        force,
    )
    .await
    .context("Code generation failed")?;

    println!("✓ Code generation complete!");

    Ok(())
}
