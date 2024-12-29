use cargo_leptos::{check_wasm_bindgen_version, run};
use cargo_leptos::{
    config::{Cli, Config},
    get_current_dir,
};
use cargo_manifest::Manifest;
// use cargo_metadata::MetadataCommand;
use clap::Parser;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let manifest_path = &cli.manifest_path;

    // This will panic and inform the user that their wasm-bindgen version doesn't match.
    check_wasm_bindgen_version(manifest_path.as_str());

    // Determine whether we're in a workspace
    let manifest = Manifest::from_path(&manifest_path)
        .expect("Failed to find or parse Cargo.toml at manifest path");

    // cargo-manifest can tell us whether the Cargo.toml manifest we're analyzing is a workspace or not
    let is_workspace = match &manifest.package {
        Some(package) => match package.workspace.is_some() {
            true => true,
            false => false,
        },
        None => false,
    };

    let root_path = get_current_dir(Some(&manifest_path));
    let mut config = Config::load(root_path, &cli.config)?;

    if !is_workspace && config.build.output_name.is_none() {
        let name = match &manifest.package {
            Some(package) => package.name.clone(),
            None => panic!("No package name found in manifest and no output-name provided."),
        };
        config.build.output_name = Some(name);
    }

    println!("CLI: {cli:#?}");
    println!("Config: {config:#?}");
    run(cli, config).await
}
